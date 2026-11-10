"""Tests for rpc_schema.py, node_schema.py and wallet_schema.py.

  python3 -m unittest discover -s tests

Most tests run the generator on tests/fake_monero, a tiny checkout with one example
of each C++ construct it has to handle. Set MONERO_REPO=/path/to/monero to also run
it on a real checkout (and, at the commit in schemas/source.json, compare the output
with the checked-in schemas).
"""
import contextlib
import io
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

import rpc_schema  # noqa: E402
from node_schema import MONEROD  # noqa: E402
from rpc_schema import Builder, Registry, Struct, Target  # noqa: E402
from wallet_schema import WALLET_RPC  # noqa: E402

FAKE = ROOT / "tests" / "fake_monero"
TEST_TARGET = Target(
    title="test RPC",
    url="http://127.0.0.1:1/json_rpc",
    version_macro="TEST_RPC_VERSION",
    defs=["src/rpc/test_defs.h"],
    server="src/rpc/test_server.h",
    impl="src/rpc/test_server.cpp",
    errors="src/rpc/test_errors.h",
)
U64 = {"type": "integer", "minimum": 0, "maximum": 2 ** 64 - 1, "x-bigint": True}
U32 = {"type": "integer", "minimum": 0, "maximum": 2 ** 32 - 1}


class FakeCheckout(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.doc, cls.warnings = rpc_schema.build(FAKE, TEST_TARGET)
        cls.schemas = cls.doc["components"]["schemas"]
        cls.methods = {m["name"]: m for m in cls.doc["methods"]}

    def schema(self, title):
        hits = [s for s in self.schemas.values() if s["title"] == title]
        self.assertEqual(len(hits), 1, f"schemas titled {title}")
        return hits[0]

    def test_version_from_macros(self):
        self.assertEqual(self.doc["info"]["version"], "3.7")
        self.assertEqual(self.doc["openrpc"], "1.3.2")

    def test_titles_are_readable(self):
        self.assertEqual(sorted(s["title"] for s in self.schemas.values()),
                         ["Entry", "GetCountResponse", "GetOtherResponse", "GetThingRequest",
                          "GetThingResponse", "RawHolder"])

    def test_integer_widths_are_bounds(self):
        props = self.schema("GetThingResponse")["properties"]
        self.assertEqual(props["big"], U64)
        self.assertEqual(props["small"], {"type": "integer", "minimum": -128, "maximum": 127})
        self.assertEqual(props["ratio"], {"type": "number"})
        self.assertNotIn("format", json.dumps(self.schemas))

    def test_several_names_in_one_declaration(self):
        entry = self.schema("Entry")
        note = {"description": "several names in one declaration"}   # the comment covers both
        self.assertEqual(entry["properties"],
                         {"a": {**U64, **note}, "bee": {**U64, **note}, "hash": {"type": "string"}})
        self.assertEqual(entry["required"], ["a", "bee", "hash"])

    def test_parent_map_is_flattened(self):
        props = self.schema("GetThingResponse")["properties"]
        self.assertEqual(list(props)[0], "status")

    def test_inherited_map_without_its_own(self):
        self.assertEqual(self.schema("GetOtherResponse")["properties"],
                         self.schema("GetThingResponse")["properties"])

    def test_raw_bytes_are_binary_strings(self):
        props = self.schema("GetThingResponse")["properties"]
        self.assertEqual(props["raw"], {"type": "string", "contentEncoding": "binary"})

    def test_required_only_for_fields_always_written(self):
        res = self.schema("GetThingResponse")
        self.assertEqual(res["required"], ["status", "small", "big", "ratio", "raw"])
        # arrays: epee skips empty containers; optional<>: may be absent; value: in a branch
        for key in ("entries", "maybe", "value"):
            self.assertNotIn(key, res["required"])

    def test_branches_keep_every_shape(self):
        value = self.schema("GetThingResponse")["properties"]["value"]
        self.assertEqual(value, {"anyOf": [U64, {"type": "string"}]})

    def test_request_only_schemas_have_no_required(self):
        # epee never rejects a request for a missing field
        self.assertNotIn("required", self.schema("GetThingRequest"))

    def test_opt_defaults(self):
        props = self.schema("GetThingRequest")["properties"]
        self.assertEqual(props["count"]["default"], 10)
        self.assertEqual(props["verbose"]["default"], False)
        self.assertEqual(props["tag"]["default"], "")
        self.assertEqual(props["heights"]["default"], [])
        self.assertNotIn("default", props["name"])

    def test_comments_become_descriptions(self):
        props = self.schema("GetThingRequest")["properties"]
        self.assertEqual(props["count"]["description"], "how many")
        # "//*" starts a line comment, not a block comment: the next field must survive
        self.assertEqual(props["tag"]["description"], "a // comment that looks like a block comment")
        self.assertIn("name", props)

    def test_comments_and_macros_are_not_parsed_as_code(self):
        titles = json.dumps(self.schemas)
        self.assertNotIn("ignored", titles)
        self.assertNotIn("not_a_struct", titles)

    def test_hand_written_serialize_call(self):
        # serialize(narrowed, ..., "list") with a local std::vector<uint32_t>
        self.assertEqual(self.schema("RawHolder")["properties"],
                         {"list": {"type": "array", "items": U32}})

    def test_statement_after_branch_on_same_line_is_reported(self):
        self.assertIn("cryptonote::raw_holder: KV map contains hand-written C++; check by hand",
                      self.warnings)
        self.assertEqual(len(self.warnings), 1, self.warnings)

    def test_by_name_params(self):
        m = self.methods["get_thing"]
        self.assertEqual(m["paramStructure"], "by-name")
        self.assertEqual([p["name"] for p in m["params"]], ["count", "verbose", "tag", "name", "heights"])
        self.assertTrue(all(p["required"] is False for p in m["params"]))
        self.assertEqual(m["result"]["schema"]["$ref"],
                         "#/components/schemas/COMMAND_RPC_GET_THING.response_t")

    def test_no_params_when_handler_ignores_request(self):
        m = self.methods["get_count"]
        self.assertEqual((m["paramStructure"], m["params"]), ("either", []))

    def test_positional_params_counted_from_handler(self):
        m = self.methods["hash_of"]
        self.assertEqual(m["paramStructure"], "by-position")
        self.assertEqual(m["params"], [{"name": "arg", "schema": U64, "required": True}])
        self.assertEqual(m["result"]["schema"], {"type": "string"})

    def test_aliases_and_restricted_methods(self):
        self.assertEqual(self.methods["getthing"]["x-alias-of"], "get_thing")
        self.assertNotIn("x-alias-of", self.methods["get_thing"])
        self.assertTrue(self.methods["get_other"]["x-hidden-in-restricted-mode"])
        self.assertNotIn("x-hidden-in-restricted-mode", self.methods["get_thing"])

    def test_uri_endpoints(self):
        self.assertEqual([(e["path"], e["encoding"]) for e in self.doc["x-uri-endpoints"]],
                         [("/get_thing", "json"), ("/get_thing.bin", "epee-binary")])

    def test_error_codes(self):
        errors = self.doc["components"]["errors"]
        self.assertEqual(errors["TEST_RPC_ERROR_CODE_UNKNOWN_ERROR"]["code"], -1)
        self.assertEqual(errors["TEST_RPC_ERROR_CODE_WRONG_PARAM"]["code"], -2)
        self.assertEqual(errors["JSONRPC_METHOD_NOT_FOUND"]["code"], -32601)

    def test_every_ref_resolves(self):
        refs = rpc_schema.collect_refs(self.doc, self.schemas, set())
        self.assertLessEqual(refs, set(self.schemas))


class Parsing(unittest.TestCase):
    def test_primitive(self):
        p = rpc_schema.primitive
        self.assertEqual(p("uint64_t"), U64)
        self.assertEqual(p("std::uint32_t"), U32)
        self.assertEqual(p("int"), {"type": "integer", "minimum": -2 ** 31, "maximum": 2 ** 31 - 1})
        self.assertEqual(p("size_t"), U64)
        self.assertEqual(p("bool"), {"type": "boolean"})
        self.assertEqual(p("float"), {"type": "number"})
        self.assertEqual(p("cryptonote::blobdata"), {"type": "string"})
        self.assertIsNone(p("crypto::hash"))

    def test_parse_type(self):
        self.assertEqual(rpc_schema.parse_type("const std::vector<std::pair<uint64_t, std::string>>&"),
                         ("std::vector", [("std::pair", [("uint64_t", []), ("std::string", [])])]))

    def test_parse_default(self):
        d = rpc_schema.parse_default
        self.assertEqual(d("(uint64_t)0", {}), 0)
        self.assertEqual(d("static_cast<uint32_t>(5)", {}), 5)
        self.assertEqual(d("0xffffffffu", {}), 4294967295)
        self.assertEqual(d("-1", {}), -1)
        self.assertEqual(d("1.5f", {}), 1.5)
        self.assertEqual(d('"abc"', {}), "abc")
        self.assertEqual(d("true", {}), True)
        self.assertEqual(d("std::vector<uint64_t>()", {"type": "array"}), [])
        self.assertEqual(d("{}", {"type": "string"}), "")
        self.assertIsNone(d("DEFAULT_RING_SIZE", {}))

    def test_parse_decl(self):
        d = rpc_schema.parse_decl
        self.assertEqual(d("struct foo a = 1, b{2};"), ("foo", ["a", "b"]))
        self.assertEqual(d("std::map<std::string, int> m;"), ("std::map<std::string, int>", ["m"]))
        self.assertEqual(d("unsigned long n;"), ("unsigned long", ["n"]))
        self.assertIsNone(d("bool f(int x);"))
        self.assertIsNone(d("return x;"))
        self.assertIsNone(d("int x"))

    def test_is_branch(self):
        b = rpc_schema.is_branch
        for code in ("if (a)", "if (a.b(c))", "} else {", "else", "else if (x(y)) {", "{", "}"):
            self.assertTrue(b(code), code)
        for code in ("if (a) KV_SERIALIZE(x)", "if (a) return true;", "KV_SERIALIZE(x)"):
            self.assertFalse(b(code), code)

    def test_split_args(self):
        self.assertEqual(rpc_schema.split_args('a, f(b, c), "d,e", std::map<x, y>()'),
                         ["a", "f(b, c)", '"d,e"', "std::map<x, y>()"])

    def test_title_collision_gets_namespace(self):
        b = Builder(Registry())
        path = ("COMMAND_RPC_GET_HEIGHT", "response_t")
        self.assertEqual(b.title(Struct("cryptonote", path, [])), "GetHeightResponse")
        self.assertEqual(b.title(Struct("tools::wallet_rpc", path, [])), "ToolsWalletRpcGetHeightResponse")


class CommandLine(unittest.TestCase):
    def run_main(self, argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            rpc_schema.main(TEST_TARGET, argv)
        return out.getvalue(), err.getvalue()

    def test_prints_schema_and_warnings(self):
        out, err = self.run_main([str(FAKE)])
        self.assertEqual(json.loads(out)["info"]["title"], "test RPC")
        self.assertIn("warning: cryptonote::raw_holder", err)

    def test_not_a_directory(self):
        with self.assertRaises(SystemExit) as cm:
            self.run_main([str(FAKE / "nope")])
        self.assertIn("is not a directory", str(cm.exception.code))

    def test_not_a_monero_checkout(self):
        with tempfile.TemporaryDirectory() as empty, self.assertRaises(SystemExit) as cm:
            self.run_main([empty])
        self.assertIn("not found", str(cm.exception.code))
        self.assertIn("monero checkout", str(cm.exception.code))

    def test_scripts(self):
        for script in ("node_schema.py", "wallet_schema.py"):
            r = subprocess.run([sys.executable, str(ROOT / script), "--help"],
                               capture_output=True, text=True)
            self.assertEqual(r.returncode, 0, r.stderr)
            self.assertIn("REPO_PATH", r.stdout)
            r = subprocess.run([sys.executable, str(ROOT / script)], capture_output=True, text=True)
            self.assertEqual(r.returncode, 2)       # argparse: missing argument


@unittest.skipUnless(os.environ.get("MONERO_REPO"), "set MONERO_REPO to a monero checkout")
class RealCheckout(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.repo = Path(os.environ["MONERO_REPO"])
        cls.daemon, cls.daemon_warnings = rpc_schema.build(cls.repo, MONEROD)
        cls.wallet, cls.wallet_warnings = rpc_schema.build(cls.repo, WALLET_RPC)

    def test_known_methods(self):
        daemon = {m["name"]: m for m in self.daemon["methods"]}
        wallet = {m["name"]: m for m in self.wallet["methods"]}
        for name in ("get_info", "get_block", "get_block_count", "on_get_block_hash"):
            self.assertIn(name, daemon)
        for name in ("get_balance", "transfer", "get_transfers"):
            self.assertIn(name, wallet)
        self.assertEqual(daemon["on_get_block_hash"]["paramStructure"], "by-position")
        info = self.daemon["components"]["schemas"]["COMMAND_RPC_GET_INFO.response_t"]
        self.assertEqual(info["properties"]["height"], U64)

    def test_few_warnings(self):
        # On master (Nov 2026): two structs with hand-written serialization in the daemon.
        self.assertLessEqual(len(self.daemon_warnings), 2, self.daemon_warnings)
        self.assertEqual(self.wallet_warnings, [])

    def test_matches_checked_in_schemas(self):
        source = json.loads((ROOT / "schemas" / "source.json").read_text())
        head = subprocess.run(["git", "-C", str(self.repo), "rev-parse", "HEAD"],
                              capture_output=True, text=True).stdout.strip()
        if head != source["commit"]:
            self.skipTest(f"checkout is at {head[:12]}, schemas/ is from {source['commit'][:12]}")
        for doc, name in ((self.daemon, "daemon"), (self.wallet, "wallet")):
            committed = json.loads((ROOT / "schemas" / f"{name}.openrpc.json").read_text())
            self.assertEqual(doc, committed, f"schemas/{name}.openrpc.json is out of date")


if __name__ == "__main__":
    unittest.main()
