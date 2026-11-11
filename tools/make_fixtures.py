#!/usr/bin/env python3
"""Build test fixtures from the request/response examples in the official Monero docs.

  python3 tools/make_fixtures.py /path/to/monero-docs schemas examples/fixtures

Reads docs/en/rpc-library/{monerod-rpc,wallet-rpc}.md from a monero-docs checkout
(github.com/monero-project/monero-docs, MIT licensed) and writes daemon.json and
wallet.json. Each case is checked against the generated schema and labelled:

  ok            matches the schema
  stale         only lacks fields the server now always sends (the example is older);
                "missing" lists them as "path/field"
  docs-mistake  wrong type, or a param the source doesn't have

"ok" cases also get "known_json": the example with only the fields the schema defines.
A correct generated type decodes "json" and re-encodes it to exactly "known_json".

Needs the `jsonschema` package (pip install jsonschema).
"""
import json
import re
import subprocess
import sys
from pathlib import Path

import jsonschema

HEAD = re.compile(r"^###\s+\*\*(.+?)\*\*\s*$")
FENCE = re.compile(r'^```(\w+)?(?:\s+title="(\w+)")?')


def docs_examples(path):
    """{section: {"request": obj|None, "response": obj|None}} from one docs page."""
    out, name, block, kind = {}, None, None, None
    for line in open(path, encoding="utf-8"):
        line = line.rstrip("\n")
        if block is None:
            if (m := HEAD.match(line)):
                name = re.sub(r"\s*\(.*\)$", "", m.group(1).strip())
            elif (m := FENCE.match(line)) and name:
                block, kind = [], (m.group(2) or "").lower()
            continue
        if line.startswith("```"):
            entry = out.setdefault(name, {"request": None, "response": None})
            text = "\n".join(block)
            try:
                if kind == "request" and (m := re.search(r"-d\s+'(.*?)'", text, re.S)):
                    entry["request"] = json.loads(m.group(1))
                elif kind == "response" and entry["response"] is None:
                    entry["response"] = json.loads(text)
            except ValueError:
                pass                                    # example isn't valid JSON
            block = None
        else:
            block.append(line)
    return out


def classify(schema, value, comps):
    """-> (status, notes, missing): missing lists "path/field" for stale cases."""
    errors = list(jsonschema.Draft7Validator({"allOf": [schema], "components": comps})
                  .iter_errors(value))
    notes = sorted(f"{'/'.join(map(str, e.absolute_path)) or '(root)'}: {e.message[:160]}"
                   for e in errors)
    if not errors:
        return "ok", notes, None
    if all(e.validator == "required" for e in errors):
        missing = sorted("/".join([*map(str, e.absolute_path),
                                   re.match(r"'(.+)' is a required property", e.message).group(1)])
                         for e in errors)
        return "stale", notes, missing
    return "docs-mistake", notes, None


def known(schema, value, schemas):
    """value with only the object keys the schema defines, recursively. For "ok" cases
    a correct generated type re-encodes the example to exactly this."""
    while isinstance(schema, dict) and ("$ref" in schema or len(schema.get("allOf", [])) == 1):
        schema = (schemas[schema["$ref"].rsplit("/", 1)[-1]] if "$ref" in schema
                  else schema["allOf"][0])
    if not isinstance(schema, dict) or "anyOf" in schema:
        return value
    if isinstance(value, dict) and "properties" in schema:
        props = schema["properties"]
        return {k: known(props[k], v, schemas) for k, v in value.items() if k in props}
    if isinstance(value, list) and "items" in schema:
        return [known(schema["items"], v, schemas) for v in value]
    return value


def build(doc, examples):
    comps = doc["components"]["schemas"]
    title = lambda ref: comps[ref.rsplit("/", 1)[-1]]["title"]
    components = doc["components"]                      # $refs resolve against this
    methods = {m["name"]: m for m in doc["methods"]}
    endpoints = {e["path"]: e for e in doc["x-uri-endpoints"] if e["encoding"] == "json"}
    cases = []

    def add(name, kind, protocol, typ, schema, value):
        status, notes, missing = (classify(schema, value, components) if schema is not None
                                  else ("ok", [], None))
        case = {"name": name, "kind": kind, "protocol": protocol, "type": typ,
                "status": status, "notes": notes, "json": value}
        if missing:
            case["missing"] = missing
        if status == "ok" and schema is not None:
            case["known_json"] = known(schema, value, comps)
        cases.append(case)

    for name, x in sorted(examples.items()):
        if name in methods:
            m = methods[name]
            req, res = x["request"], x["response"]
            if req is not None:
                params = req.get("params")
                if m["paramStructure"] == "by-position":
                    if isinstance(params, list):
                        add(name, "request", "json-rpc", None,
                            {"type": "array", "items": m["params"][0]["schema"]}, params)
                else:
                    want = {p["name"]: p["schema"] for p in m["params"]}
                    # no params: every empty request type matches, so don't name one
                    match = [k for k, v in comps.items() if want and v.get("properties") == want]
                    typ = next((comps[k]["title"] for k in match
                                if comps[k]["title"].endswith("Request")),
                               comps[match[0]]["title"] if match else None)
                    add(name, "request", "json-rpc", typ,
                        {"type": "object", "properties": want, "additionalProperties": False},
                        params if params is not None else {})
            if res is not None and "error" in res:
                add(name, "error", "json-rpc", None, None, res["error"])
            elif res is not None and "result" in res:
                schema = m["result"]["schema"]
                add(name, "response", "json-rpc",
                    title(schema["$ref"]) if "$ref" in schema else None, schema, res["result"])
        else:
            e = endpoints.get(name) or endpoints.get("/" + name.lstrip("/"))
            if not e:
                continue
            for kind, sch in (("request", e["request"]), ("response", e["response"])):
                if x[kind] is not None:
                    add(e["path"], kind, "uri", title(sch["$ref"]) if "$ref" in sch else None,
                        sch, x[kind])
    return cases


def main():
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    docs, schemas, out = map(Path, sys.argv[1:])
    commit = subprocess.run(["git", "-C", str(docs), "rev-parse", "HEAD"],
                            capture_output=True, text=True).stdout.strip() or None
    out.mkdir(parents=True, exist_ok=True)
    for target, page in (("daemon", "monerod-rpc.md"), ("wallet", "wallet-rpc.md")):
        doc = json.load(open(schemas / f"{target}.openrpc.json"))
        cases = build(doc, docs_examples(docs / "docs/en/rpc-library" / page))
        fixture = {
            "source": "Request/response examples from the official Monero docs, "
                      "github.com/monero-project/monero-docs (MIT License, "
                      "Copyright (c) 2024-present Monero Community)",
            "docs_page": f"docs/en/rpc-library/{page}", "docs_commit": commit,
            "schema": f"schemas/{target}.openrpc.json", "schema_version": doc["info"]["version"],
            "cases": cases}
        (out / f"{target}.json").write_text(json.dumps(fixture, indent=1) + "\n")
        counts = {s: sum(c["status"] == s for c in cases) for s in ("ok", "stale", "docs-mistake")}
        print(f"{target}: {len(cases)} cases {counts}", file=sys.stderr)


if __name__ == "__main__":
    main()
