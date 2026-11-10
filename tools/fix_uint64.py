#!/usr/bin/env python3
"""Fix quicktype output so unsigned 64-bit fields can hold every value Monero sends.

  python3 tools/fix_uint64.py go   daemon.schema.json types.go

quicktype maps every JSON Schema integer to int64 in Go and ignores the
bounds. Monero sends uint64 values above 2^63-1 (for example
already_generated_coins = 18186022843595960691), which those types can't decode. This
switches every field whose schema has maximum > 2^63-1 to the unsigned type.
The file is edited in place; the number of changed fields goes to stderr.
"""
import json
import re
import sys

INT64_MAX = 2 ** 63 - 1


def load_defs(path):
    defs = json.load(open(path))["definitions"]
    # quicktype re-cases names (MakeUriRequest -> MakeURIRequest), so match case-insensitively
    return defs, {re.sub(r"\W", "", k).lower(): v for k, v in defs.items()}


def unsigned64(schema, defs):
    s = schema
    while isinstance(s, dict) and len(s.get("allOf", [])) == 1:
        s = {**s["allOf"][0], **{k: v for k, v in s.items() if k != "allOf"}}
    if isinstance(s, dict) and "$ref" in s:
        s = defs.get(s["$ref"].rsplit("/", 1)[-1], {})
    s = s or {}
    if s.get("type") == "array":
        return unsigned64(s.get("items", {}), defs)
    return s.get("type") == "integer" and s.get("maximum", 0) > INT64_MAX


def fix_go(lines, defs, by_name):
    struct, changed = None, 0
    field = re.compile(r'^(\s+\w+\s+)(\S+)(\s+`json:"([^",]+))')
    for i, line in enumerate(lines):
        if (m := re.match(r"^type (\w+) struct \{", line)):
            struct = by_name.get(m.group(1).lower())
        elif line.startswith("}"):
            struct = None
        elif struct and (m := field.match(line)) and "int64" in m.group(2):
            prop = struct.get("properties", {}).get(m.group(4))
            if prop is not None and unsigned64(prop, defs):
                head, typ = m.group(1), m.group(2)
                lines[i] = head + typ.replace("int64", "uint64") + line[len(head) + len(typ):]
                changed += 1
    return changed


if __name__ == "__main__":
    if len(sys.argv) != 4 or sys.argv[1] != "go":
        sys.exit(__doc__)
    lang, schema_path, code_path = sys.argv[1:]
    defs, by_name = load_defs(schema_path)
    lines = open(code_path).read().split("\n")
    changed = fix_go(lines, defs, by_name)
    open(code_path, "w").write("\n".join(lines))
    print(f"{code_path}: {changed} fields changed to unsigned 64-bit", file=sys.stderr)
