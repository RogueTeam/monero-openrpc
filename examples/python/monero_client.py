"""Minimal client for monerod / monero-wallet-rpc using the generated pydantic models.

    from monero_client import Client
    import monerod_models as m

    node = Client("http://127.0.0.1:18081")
    info = node.call("get_info", m.GetInfoRequest(), m.GetInfoResponse)
    print(info.height)

Doesn't implement HTTP digest authentication (--rpc-login).
"""
import json
import urllib.request

from pydantic import BaseModel


class RPCError(Exception):
    def __init__(self, code, message):
        super().__init__(f"json-rpc error {code}: {message}")
        self.code, self.message = code, message


class Client:
    def __init__(self, base_url, timeout=30):
        self.base_url, self.timeout = base_url.rstrip("/"), timeout

    def _post(self, path, payload):
        req = urllib.request.Request(self.base_url + path, data=json.dumps(payload).encode(),
                                     headers={"Content-Type": "application/json"})
        with urllib.request.urlopen(req, timeout=self.timeout) as res:
            return json.load(res)

    @staticmethod
    def _dump(params):
        if isinstance(params, BaseModel):    # only the fields you set, under their wire names
            return params.model_dump(mode="json", by_alias=True, exclude_unset=True)
        return params

    def call(self, method, params=None, result_model=None):
        """JSON-RPC call to /json_rpc. params: a request model, a list (by-position
        methods such as on_get_block_hash) or None. Returns result_model or raw JSON."""
        payload = {"jsonrpc": "2.0", "id": "0", "method": method}
        if params is not None:
            payload["params"] = self._dump(params)
        reply = self._post("/json_rpc", payload)
        if reply.get("error"):
            raise RPCError(reply["error"].get("code"), reply["error"].get("message"))
        result = reply.get("result")
        return result_model.model_validate(result) if result_model else result

    def post(self, path, request=None, result_model=None):
        """Plain JSON endpoint such as /get_height."""
        reply = self._post(path, self._dump(request) if request is not None else {})
        return result_model.model_validate(reply) if result_model else reply
