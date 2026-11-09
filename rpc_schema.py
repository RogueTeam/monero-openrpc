"""Shared code for node_schema.py and wallet_schema.py: reads a Monero source
checkout and builds an OpenRPC document for one RPC server.

Fields come from the epee KV_SERIALIZE maps, methods from the URI / JSON-RPC
route maps in the server header. Python 3.8+, standard library only.
"""
import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import List


@dataclass
class Target:
    """Where one RPC server is defined, as paths relative to the checkout."""
    title: str              # OpenRPC info.title
    url: str                # default JSON-RPC endpoint
    version_macro: str      # e.g. CORE_RPC_VERSION (_MAJOR / _MINOR are appended)
    defs: List[str]         # headers with the request/response structs
    server: str             # header with the URI / JSON-RPC route maps

# ----------------------------------------------------------------- C++ types
STRINGS = {"std::string", "string", "blobdata", "cryptonote::blobdata",
           "epee::wipeable_string"}
SEQUENCES = {"vector", "list", "deque", "set", "unordered_set"}
# epee calls written by hand inside a KV map, and the macro each one equals.
RAW_CALLS = {"serialize": "KV_SERIALIZE_N",
             "serialize_t_val_as_blob": "KV_SERIALIZE_VAL_POD_AS_BLOB_N",
             "serialize_stl_container_pod_val_as_blob": "KV_SERIALIZE_CONTAINER_POD_AS_BLOB_N"}


def primitive(name):
    """JSON Schema for a C++ scalar or string type, or None."""
    m = re.fullmatch(r"(?:std::)?(u?)int(8|16|32|64)_t", name)
    if m or name in ("int", "unsigned", "unsigned int", "size_t"):
        unsigned = m.group(1) == "u" if m else name != "int"
        return {"type": "integer", "minimum": 0} if unsigned else {"type": "integer"}
    if name == "bool":
        return {"type": "boolean"}
    if name in ("double", "float"):
        return {"type": "number"}
    if name in STRINGS:
        return {"type": "string"}
    return None


def parse_type(s):
    """'std::vector<std::pair<a, b>>' -> ('std::vector', [('std::pair', [...])])"""
    s = re.sub(r"\b(?:const|volatile|typename|struct|class)\b|[&*]", " ", s).strip()
    m = re.fullmatch(r"([\w:]+(?:\s+[\w:]+)*?)\s*(?:<(.*)>)?", s, re.S)
    if not m:
        raise ValueError(f"cannot parse type {s!r}")
    name = re.sub(r"\s+", " ", m.group(1)).lstrip(":")
    return name, [parse_type(a) for a in split_args(m.group(2) or "")]


def is_optional_type(t):
    try:
        return parse_type(t)[0].split("::")[-1] == "optional"
    except ValueError:
        return False


# ------------------------------------------------------------ header parsing
class Struct:
    def __init__(self, ns, path, bases):
        self.ns, self.path, self.bases = ns, path, bases
        self.fields, self.aliases = {}, {}
        self.kv = None          # KV_SERIALIZE entries; None = no map of its own
        self.custom = []        # hand-written C++ statements inside the map

    @property
    def qualname(self):
        return "::".join(p for p in (self.ns, *self.path) if p)


def ns_match(ns, hint):
    return ns == hint or ns.endswith("::" + hint) or hint.endswith("::" + ns)


class Registry:
    def __init__(self):
        self.structs, self.aliases, self.enums = {}, {}, set()

    def find(self, path, hint):
        cands = self.structs.get(path, [])
        return next((c for c in cands if hint and ns_match(c.ns, hint)),
                    cands[0] if cands else None)

    def resolve(self, name, ctx):
        """C++-like lookup from ctx outward.
        -> ('struct', S) | ('enum', path) | ('alias', type_str, owner) | None"""
        parts = [p for p in name.strip().split("::") if p]
        base, ctx_ns = (ctx.path, ctx.ns) if ctx else ((), "")
        for k in range(len(parts)):
            hint, rel = "::".join(parts[:k]) or ctx_ns, tuple(parts[k:])
            for i in range(len(base), -1, -1):
                path = base[:i] + rel
                s = self.find(path, hint)
                if s:
                    return ("struct", s)
                if path in self.enums:
                    return ("enum", path)
                owner = self.find(path[:-1], hint) if len(path) > 1 else None
                if owner and path[-1] in owner.aliases:
                    return ("alias", owner.aliases[path[-1]], owner)
                if len(path) == 1 and path[0] in self.aliases:
                    opts = self.aliases[path[0]]
                    t = next((t for n, t in opts if hint and ns_match(n, hint)), opts[0][1])
                    return ("alias", t, None)
        return None


def code_lines(text):
    """Yield (code, trailing // comment) per line. Block comments are blanked out
    (keeping line breaks); a // comment wins over a /* after it on the same line.
    Assumes no // or /* inside string literals, which holds for the RPC headers."""
    text = re.sub(r"//[^\n]*|/\*.*?\*/", lambda m: m.group() if m.group()[1] == "/"
                  else "\n" * m.group().count("\n"), text, flags=re.S)
    for line in text.splitlines():
        code, _, comment = line.partition("//")
        yield code.strip(), comment.strip(" /*")


def split_args(s):
    """Split macro/template arguments on top-level commas."""
    out, depth, cur, in_str = [], 0, "", False
    for ch in s:
        if ch == '"':
            in_str = not in_str
        if not in_str:
            if ch in "(<[{":
                depth += 1
            elif ch in ")>]}":
                depth -= 1
            elif ch == "," and depth == 0:
                out.append(cur.strip())
                cur = ""
                continue
        cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


NS_RE = re.compile(r"^namespace\b\s*([\w:]*)")
STRUCT_RE = re.compile(r"^(?:struct|class)\s+(\w+)\s*(?:final\s*)?(?::\s*([^{]+?))?\s*(\{.*)?$")
ENUM_RE = re.compile(r"^enum\s+(?:class\s+)?(\w*)")
TYPEDEF_RE = re.compile(r"^typedef\s+(.+?)\s+(\w+)\s*;")
USING_RE = re.compile(r"^using\s+(\w+)\s*=\s*(.+?)\s*;")
DECL_RE = re.compile(r"^(?P<type>[\w:]+(?:\s*<.*>)?(?:\s+(?:int|long|char|short))*)"
                     r"\s+(?P<rest>.+)$", re.S)
MACRO_RE = re.compile(r"^(KV_SERIALIZE\w*)\s*\((.*)\)\s*;?$")
RAW_RE = re.compile(r"selector<\s*is_store\s*>::(\w+)\s*\(\s*([\w.]+)\s*,\s*stg\s*,"
                    r"\s*hparent_section\s*,\s*(\"\w+\")\s*\)")
NOT_A_TYPE = {"return", "typedef", "using", "static", "friend", "delete", "throw", "else",
              "goto", "break", "continue", "case", "template", "virtual", "explicit",
              "inline", "mutable", "if", "for", "while", "do", "switch", "public",
              "private", "protected"}


def parse_decl(code):
    """'struct foo a = 1, b{2};' -> ('foo', ['a', 'b']); None if not a variable."""
    if not code.endswith(";") or code.split()[0] in NOT_A_TYPE:
        return None
    m = DECL_RE.match(re.sub(r"^(?:(?:struct|class|typename|const)\s+)+", "", code[:-1]))
    if not m or m.group("type").split()[0] in NOT_A_TYPE:
        return None
    names = []
    for piece in split_args(m.group("rest")):
        pm = re.match(r"^(\w+)\s*(?:=.*|\{.*\})?$", piece, re.S)
        if not pm:
            return None                 # function, reference, pointer, ...
        names.append(pm.group(1))
    return m.group("type"), names


def is_branch(code):
    """True if the line is only braces and/or a branch: 'if (...)', 'else',
    '} else {', 'else if (...) {'. A branch followed by a statement on the same
    line returns False, so the statement gets a "hand-written C++" warning
    instead of being lost."""
    m = re.fullmatch(r"\}?\s*(?:else\b\s*)?(?:if\s*(\(.*\)))?\s*\{?", code)
    depth, cond = 0, (m.group(1) or "") if m else None
    for i, ch in enumerate(cond or ""):
        depth += (ch == "(") - (ch == ")")
        if depth == 0 and i < len(cond) - 1:
            return False                        # condition closed before the end
    return m is not None


def parse_header(text, reg):
    """Collect structs, members, typedefs, enums and KV_SERIALIZE entries."""
    stack, depth, pending = [], 0, None
    kv, kv_depth, cond_next, kv_locals, in_define = None, 0, False, {}, False
    for code, comment in code_lines(text):
        if in_define or code.startswith("#"):          # preprocessor + continuations
            in_define = code.endswith("\\")
            continue
        if not code:
            continue
        ns = "::".join(e["name"] for e in stack if e["kind"] == "ns" and e["name"])
        spath = tuple(e["name"] for e in stack if e["kind"] == "struct")
        top = stack[-1] if stack else None
        at_struct = top is not None and top["kind"] == "struct" and depth == top["depth"]

        if kv is not None:                              # inside a KV_SERIALIZE map
            if code.startswith("END_KV_SERIALIZE_MAP"):
                kv = None
            elif is_branch(code):
                cond_next = cond_next or bool(re.search(r"\b(?:if|else)\b", code))
            else:
                stmt, cond, cond_next = code, cond_next or depth > kv_depth, False
                if (m := MACRO_RE.match(stmt)):
                    kv.kv.append({"macro": m.group(1), "args": split_args(m.group(2)),
                                  "cond": cond, "comment": comment})
                elif (m := RAW_RE.search(stmt)) and m.group(1) in RAW_CALLS:
                    var = m.group(2).replace("this_ref.", "")
                    kv.kv.append({"macro": RAW_CALLS[m.group(1)], "args": [var, m.group(3)],
                                  "cond": cond, "comment": comment,
                                  "local_type": kv_locals.get(var)})
                elif (d := parse_decl(stmt)):
                    kv_locals.update((n, d[0]) for n in d[1])
                else:
                    kv.custom.append(stmt)
        elif (m := NS_RE.match(code)):
            pending = ("ns", m.group(1), None)
        elif (m := STRUCT_RE.match(code)) and not (code.endswith(";") and "{" not in code):
            bases = [re.sub(r"\b(public|protected|private|virtual)\b", "", b).strip()
                     for b in split_args(m.group(2) or "")]
            pending = ("struct", m.group(1), [b for b in bases if b])
        elif (m := ENUM_RE.match(code)) and not code.endswith(";"):
            pending = ("enum", m.group(1), None)
            if m.group(1):
                reg.enums.add(spath + (m.group(1),))
        elif at_struct:
            s = top["obj"]
            if code.startswith("BEGIN_KV_SERIALIZE_MAP"):
                s.kv, kv, kv_depth, kv_locals, cond_next = [], s, depth, {}, False
            elif (m := TYPEDEF_RE.match(code)):
                s.aliases[m.group(2)] = m.group(1)
            elif (m := USING_RE.match(code)):
                s.aliases[m.group(1)] = m.group(2)
            elif (d := parse_decl(code)):
                s.fields.update((n, (d[0], comment)) for n in d[1])
        elif top is None or (top["kind"] == "ns" and depth == top["depth"]):
            if (m := TYPEDEF_RE.match(code)):
                reg.aliases.setdefault(m.group(2), []).append((ns, m.group(1)))
            elif (m := USING_RE.match(code)):
                reg.aliases.setdefault(m.group(1), []).append((ns, m.group(2)))

        for ch in code:                                 # brace tracking
            if ch == "{":
                depth += 1
                kind, name, extra = pending or ("block", None, None)
                obj = None
                if kind == "struct":
                    obj = Struct(ns, spath + (name,), extra)
                    reg.structs.setdefault(obj.path, []).append(obj)
                stack.append({"kind": kind, "name": name, "depth": depth, "obj": obj})
                pending, cond_next = None, False
            elif ch == "}":
                if stack and stack[-1]["depth"] == depth:
                    stack.pop()
                depth -= 1


ROUTE_RE = re.compile(r"^(MAP_URI_AUTO_JON2|MAP_URI_AUTO_BIN2|MAP_JON_RPC)(?:_WE)?(?:_IF)?"
                      r"\s*\((.*)\)\s*;?$")


def parse_routes(text, warn):
    """Entries of BEGIN_URI_MAP2 / BEGIN_JSON_RPC_MAP in a server header."""
    routes = []
    for code, _ in code_lines(text):
        m = ROUTE_RE.match(code)
        if m and len(a := split_args(m.group(2))) >= 3:
            routes.append({"macro": m.group(1), "name": a[0].strip('"'), "handler": a[1],
                           "command": a[2], "cond": a[3] if len(a) > 3 else None})
        elif re.match(r"^MAP_\w+\s*\(", code):
            warn(f"unhandled route: {code}")
    return routes


# ---------------------------------------------------------- schema building
class Builder:
    def __init__(self, reg):
        self.reg, self.schemas, self.warnings = reg, {}, []

    def warn(self, msg):
        self.warnings.append(msg)

    def ref(self, s):
        name = ".".join(s.path)
        if len(self.reg.structs[s.path]) > 1 and s.ns:
            name = s.ns.replace("::", ".") + "." + name
        if name not in self.schemas:
            self.schemas[name] = {}                     # placeholder for recursion
            self.schemas[name] = self.struct_schema(s)
        return {"$ref": "#/components/schemas/" + name}

    def type_schema(self, t, ctx):
        try:
            return self.node_schema(parse_type(t), ctx, 0)
        except ValueError as e:
            self.warn(f"{ctx.qualname if ctx else '?'}: {e}")
            return {}

    def node_schema(self, node, ctx, depth):
        name, args = node
        bare = name.split("::")[-1]
        if depth < 16:
            if bare in ("struct_init", "optional") and args:
                return self.node_schema(args[0], ctx, depth + 1)
            if bare in SEQUENCES and args:
                return {"type": "array", "items": self.node_schema(args[0], ctx, depth + 1)}
            prim = primitive(name)
            if prim:
                return prim
            r = self.reg.resolve(name, ctx)
            if r and r[0] == "struct":
                return self.ref(r[1])
            if r and r[0] == "enum":
                return {"type": "integer"}
            if r:
                try:
                    return self.node_schema(parse_type(r[1]), r[2], depth + 1)
                except ValueError:
                    pass
        self.warn(f"{ctx.qualname if ctx else '?'}: unresolved type {name}")
        return {}

    def bases(self, s):
        for b in s.bases:
            try:
                r = self.reg.resolve(parse_type(b)[0], s)
            except ValueError:
                continue
            if r and r[0] == "struct" and r[1] is not s:
                yield r[1]

    def map_owner(self, s, seen=()):
        """Struct whose KV map s uses: its own, or else the one inherited from a base."""
        if s.kv is not None:
            return s
        for b in self.bases(s):
            if b not in seen and (owner := self.map_owner(b, seen + (s,))):
                return owner
        return None

    def kv_entries(self, s, seen, visited):
        """Yield (entry, owner) with KV_SERIALIZE_PARENT flattened."""
        visited.append(s)
        for e in s.kv:
            if e["macro"] != "KV_SERIALIZE_PARENT":
                yield e, s
                continue
            r = self.reg.resolve(e["args"][0], s)
            parent = self.map_owner(r[1]) if r and r[0] == "struct" else None
            if parent is None or parent in seen:
                self.warn(f"{s.qualname}: cannot resolve parent {e['args'][0]}")
                continue
            for pe, owner in self.kv_entries(parent, seen + (s,), visited):
                yield dict(pe, cond=pe["cond"] or e["cond"]), owner

    def member(self, s, name, seen=()):
        if name in s.fields:
            return s.fields[name][0], s, s.fields[name][1]
        for b in self.bases(s):
            if b not in seen and (hit := self.member(b, name, seen + (s,))):
                return hit
        return None

    def field_type(self, owner, var):
        """Declared type of a serialized member; follows data.start_height."""
        hit = None
        for i, part in enumerate(var.split(".")):
            if i:
                try:
                    r = self.reg.resolve(parse_type(hit[0])[0], hit[1])
                except ValueError:
                    return None
                if not r or r[0] != "struct":
                    return None
                owner = r[1]
            hit = self.member(owner, part)
            if not hit:
                return None
        return hit

    def kv_property(self, e, owner):
        """-> (wire name, schema, required)"""
        m, args = e["macro"], e["args"]
        var = args[0].strip().replace("this_ref.", "")
        wire = args[1].strip().strip('"') if m.endswith("_N") and len(args) > 1 else var
        hit = (e["local_type"], owner, "") if e.get("local_type") else self.field_type(owner, var)
        if hit is None:
            self.warn(f"{owner.qualname}: no declaration for serialized member '{var}'")
            return wire, {}, False
        ctype, ctx, _ = hit
        if "POD_AS_BLOB" in m:                 # raw bytes, not hex
            schema = {"type": "string", "contentEncoding": "binary"}
        else:
            schema = self.type_schema(ctype, ctx)
        optional = "OPT" in m or is_optional_type(ctype) or e["cond"]
        return wire, schema, not optional

    def struct_schema(self, s):
        out = {"type": "object", "properties": {}}
        owner = self.map_owner(s)
        if owner is None:
            self.warn(f"{s.qualname}: no KV_SERIALIZE map")
            return out
        variants, visited = {}, []
        for e, eowner in self.kv_entries(owner, (), visited):
            wire, prop, required = self.kv_property(e, eowner)
            variants.setdefault(wire, []).append((prop, required))
        required = []
        for wire, vs in variants.items():
            # The same key can be written in several if/else branches: keep each shape.
            alts = []
            for prop, _ in vs:
                if prop not in alts:
                    alts.append(prop)
            out["properties"][wire] = alts[0] if len(alts) == 1 else {"anyOf": alts}
            if len(vs) == 1 and vs[0][1]:
                required.append(wire)
        if required:
            out["required"] = required
        for v in visited:
            if v.custom:
                self.warn(f"{v.qualname}: KV map contains hand-written C++; check by hand")
        return out

    def io(self, cmd, which):
        """Schema of cmd::request or cmd::response (a $ref when it's a struct)."""
        r = self.reg.resolve(f"{cmd}::{which}", None)
        if r is None:
            self.warn(f"{cmd}: no {which} type found")
            return None
        if r[0] == "struct":
            return self.ref(r[1])
        if r[0] == "enum":
            return {"type": "integer"}
        return self.type_schema(r[1], r[2])     # typedef: struct_init<T>, std::vector<T>

    def params(self, req):
        if "$ref" in req:                       # struct: one named param per field
            params = []
            for k, v in self.schemas[req["$ref"].rsplit("/", 1)[-1]]["properties"].items():
                params.append({"name": k, "schema": v, "required": False})
            return "by-name", params
        # not a struct (std::vector<T>, ...): the params are sent as an array
        return "by-position", [{"name": "arg", "schema": req.get("items", req), "required": False}]


def build(repo, target):
    """-> (OpenRPC document, sorted warnings) for one Target in the checkout at repo."""
    reg = Registry()

    def read(rel):
        path = repo / rel
        if not path.is_file():
            sys.exit(f"error: {path} not found; is {repo} a monero checkout?")
        return path.read_text(errors="replace")

    defs = [read(rel) for rel in target.defs]
    for text in defs:
        parse_header(text, reg)
    b = Builder(reg)
    ver = dict(re.findall(rf"#define\s+{target.version_macro}_(MAJOR|MINOR)\s+(\d+)", "\n".join(defs)))
    if len(ver) != 2:
        b.warn(f"{target.version_macro}_MAJOR/MINOR not found")

    methods, endpoints, first_seen = [], [], {}
    for r in parse_routes(read(target.server), b.warn):
        req, res = b.io(r["command"], "request"), b.io(r["command"], "response")
        if req is None or res is None:
            continue
        extra = {}
        key = (r["macro"], r["command"], r["handler"])
        if key in first_seen:
            extra["x-alias-of"] = first_seen[key]
        first_seen.setdefault(key, r["name"])
        if r["cond"] and r["cond"].replace(" ", "") == "!m_restricted":
            extra["x-hidden-in-restricted-mode"] = True
        elif r["cond"]:
            b.warn(f"{r['name']}: unknown route condition {r['cond']}")
        if r["macro"] == "MAP_JON_RPC":
            structure, params = b.params(req)
            methods.append({"name": r["name"], "paramStructure": structure, "params": params,
                            "result": {"name": "result", "schema": res}, **extra})
        else:
            endpoints.append({"path": r["name"], "httpMethod": "POST",
                              "encoding": "json" if "JON" in r["macro"] else "epee-binary",
                              "request": req, "response": res, **extra})

    doc = {
        "openrpc": "1.3.2",
        "info": {"title": target.title,
                 "version": f"{ver.get('MAJOR', '?')}.{ver.get('MINOR', '?')}",
                 "description": "Generated from the Monero C++ source."},
        "servers": [{"name": "json_rpc", "url": target.url}],
        "methods": methods,
        "components": {"schemas": dict(sorted(b.schemas.items()))},
        "x-uri-endpoints": endpoints,
    }
    return doc, sorted(set(b.warnings))


def main(target, argv=None):
    """Command line for one target: REPO_PATH; schema to stdout, warnings to stderr."""
    ap = argparse.ArgumentParser(
        description=f"Print the OpenRPC schema of {target.title}, generated from a "
                    "Monero source checkout.")
    ap.add_argument("repo", type=Path, metavar="REPO_PATH",
                    help="path to a monero source checkout (github.com/monero-project/monero)")
    args = ap.parse_args(argv)
    if not args.repo.is_dir():
        sys.exit(f"error: {args.repo} is not a directory")
    doc, warnings = build(args.repo, target)
    for w in warnings:
        print("warning: " + w, file=sys.stderr)
    print(json.dumps(doc, indent=2))
