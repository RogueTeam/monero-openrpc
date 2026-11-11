"""Tests for the generated pydantic models and the example client.

    python3 -m unittest -v test_examples.py
"""
import json
import threading
import unittest
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

from pydantic import ValidationError

import monerod_models
import walletrpc_models
from monero_client import Client, RPCError

FIXTURES = Path(__file__).resolve().parent.parent / "fixtures"


class DocsExamples(unittest.TestCase):
    """Every docs example against its generated model, in strict mode (no coercion,
    so "120000" isn't accepted as an integer):
    ok           validates, and dumps back to exactly known_json
    stale        fails only because fields the server now always sends are missing
    docs-mistake fails, or changes when dumped back"""

    def run_fixture(self, name, module):
        cases = json.loads((FIXTURES / name).read_text())["cases"]
        checked = 0
        for c in cases:
            if c["type"] is None or c["kind"] not in ("request", "response"):
                continue
            with self.subTest(case=f'{c["name"]}/{c["kind"]}'):
                model = getattr(module, c["type"])
                try:
                    obj = model.model_validate(c["json"], strict=True)
                    error = None
                    dumped = obj.model_dump(mode="json", by_alias=True, exclude_unset=True)
                except ValidationError as e:
                    error, dumped = e, None
                if c["status"] == "ok":
                    self.assertIsNone(error)
                    self.assertEqual(dumped, c["known_json"])
                elif c["status"] == "stale":
                    self.assertIsNotNone(error)
                    self.assertEqual({e["type"] for e in error.errors()}, {"missing"}, error)
                    missing = sorted("/".join(map(str, e["loc"])) for e in error.errors())
                    self.assertEqual(missing, c["missing"])
                else:
                    self.assertTrue(error is not None or dumped != c["json"],
                                    f"docs example no longer shows the mistake {c['notes']}")
            checked += 1
        self.assertGreater(checked, 0)

    def test_daemon(self):
        self.run_fixture("daemon.json", monerod_models)

    def test_wallet(self):
        self.run_fixture("wallet.json", walletrpc_models)


class Models(unittest.TestCase):
    def test_uint64_max(self):
        r = monerod_models.GetminerdataResponse.model_validate({
            "status": "OK", "major_version": 16, "height": 3000000, "prev_id": "",
            "seed_hash": "", "difficulty": "0x1", "median_weight": 300000,
            "already_generated_coins": 18446744073709551615})
        self.assertEqual(r.already_generated_coins, 2 ** 64 - 1)

    def test_bounds_enforced(self):
        with self.assertRaises(ValidationError):
            monerod_models.GetBlockRequest(height=-1)

    def test_reserved_word_param(self):
        req = walletrpc_models.GetTransfersRequest.model_validate({"in": True})
        self.assertEqual(req.model_dump(by_alias=True, exclude_unset=True), {"in": True})


class FakeNode(BaseHTTPRequestHandler):
    reply, seen = None, None

    def do_POST(self):
        body = self.rfile.read(int(self.headers["Content-Length"]))
        type(self).seen = (self.path, json.loads(body))
        data = json.dumps(type(self).reply).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, *args):
        pass


class ClientTest(unittest.TestCase):
    def setUp(self):
        self.server = HTTPServer(("127.0.0.1", 0), FakeNode)
        threading.Thread(target=self.server.serve_forever, daemon=True).start()
        self.client = Client(f"http://127.0.0.1:{self.server.server_port}")

    def tearDown(self):
        self.server.shutdown()
        self.server.server_close()

    def test_call_by_name(self):
        FakeNode.reply = {"jsonrpc": "2.0", "id": "0", "result": {
            "status": "OK", "credits": 0, "top_hash": "", "count": 1, "headers": []}}
        req = monerod_models.GetBlockHeadersRangeRequest(start_height=10, end_height=11)
        res = self.client.call("get_block_headers_range", req, monerod_models.GetBlockHeadersRangeResponse)
        self.assertEqual(FakeNode.seen, ("/json_rpc", {"jsonrpc": "2.0", "id": "0",
                         "method": "get_block_headers_range",
                         "params": {"start_height": 10, "end_height": 11}}))
        self.assertEqual(res.status, "OK")

    def test_call_by_position(self):
        FakeNode.reply = {"jsonrpc": "2.0", "id": "0", "result": "e22cf75f39ae720e8b71b3d120a5ac03"}
        self.assertEqual(self.client.call("on_get_block_hash", [912345]), "e22cf75f39ae720e8b71b3d120a5ac03")
        self.assertEqual(FakeNode.seen[1]["params"], [912345])

    def test_error(self):
        FakeNode.reply = {"jsonrpc": "2.0", "id": "0", "error": {"code": -7, "message": "Block not accepted"}}
        with self.assertRaises(RPCError) as ctx:
            self.client.call("submit_block", ["0707"])
        self.assertEqual(ctx.exception.code, -7)

    def test_post(self):
        FakeNode.reply = {"hash": "7e23", "height": 2287217, "status": "OK", "untrusted": False}
        res = self.client.post("/get_height", None, monerod_models.GetHeightResponse)
        self.assertEqual((FakeNode.seen[0], res.height), ("/get_height", 2287217))


if __name__ == "__main__":
    unittest.main()
