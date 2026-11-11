// Calls every JSON-RPC docs example in ../fixtures through the generated clients,
// against a fake server that answers with the docs response. Checks the method name
// and params the client sends, and what it returns (or throws).
import { test } from "node:test";
import assert from "node:assert/strict";
import http from "node:http";
import { readFileSync } from "node:fs";
import Monerod from "../build/src/monerod.js";
import WalletRpc from "../build/src/walletrpc.js";
import { argsFor } from "./make_typecheck.mjs";

const read = (path) => JSON.parse(readFileSync(new URL(path, import.meta.url)));

// Bugs in the OpenRPC tools themselves, not in the schema. They're reported as TODO, so
// the run stays green but shows them, and says so if they ever get fixed.
// The generated client passes its arguments to the validator as an array, but for
// by-name methods the validator (@open-rpc/schema-utils-js) looks each param up by
// name: params["height"] is undefined on an array, so by-name params are never checked,
// and a param named like an Array method is checked against that method instead.
const NOT_VALIDATED = "the generated client never validates by-name params";
const TOOL_BUGS = {
  get_address_book: "the validator checks params.entries, i.e. Array.prototype.entries, and rejects the call",
};

async function fakeServer() {
  const state = { seen: null, reply: null };
  const server = http.createServer((req, res) => {
    let body = "";
    req.on("data", (chunk) => (body += chunk));
    req.on("end", () => {
      const request = JSON.parse(body);
      state.seen = { path: req.url, request };
      res.setHeader("Content-Type", "application/json");
      res.end(JSON.stringify({ jsonrpc: "2.0", id: request.id, ...state.reply }));
    });
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  const transport = { type: "http", host: "127.0.0.1", port: server.address().port, path: "/json_rpc" };
  return { state, server, transport };
}

// Docs examples by method: { request, response, error }.
function examplesByMethod(fixture) {
  const out = new Map();
  for (const c of read(fixture).cases) {
    if (c.protocol !== "json-rpc") continue;
    if (!out.has(c.name)) out.set(c.name, {});
    out.get(c.name)[c.kind] = c;
  }
  return out;
}

for (const [label, Client, fixture, schema] of [
  ["monerod", Monerod, "../../fixtures/daemon.json", "../../../schemas/daemon.openrpc.json"],
  ["monero-wallet-rpc", WalletRpc, "../../fixtures/wallet.json", "../../../schemas/wallet.openrpc.json"],
]) {
  test(`${label}: docs examples through the generated client`, async (t) => {
    const methods = new Map(read(schema).methods.map((m) => [m.name, m]));
    const { state, server, transport } = await fakeServer();
    const client = new Client({ transport });
    t.after(() => server.close());

    for (const [name, x] of examplesByMethod(fixture)) {
      const method = methods.get(name);
      const { request, response, error } = x;
      if (!method || !request) continue;
      const args = argsFor(method, request.json);
      const options = TOOL_BUGS[name] ? { todo: TOOL_BUGS[name] } : {};

      if (request.status === "docs-mistake") {
        // Only when the mistake is in a value (wrong type), not an unknown param name:
        // the client should check params against the schema and refuse to send them.
        if (Array.isArray(request.json) || typeof request.json !== "object") continue;
        if (!Object.keys(request.json).every((k) => method.params.some((p) => p.name === k))) continue;
        const todo = method.paramStructure === "by-name" ? { todo: NOT_VALIDATED } : options;
        await t.test(`${name} rejects the docs mistake (${request.notes.join("; ")})`, todo, async () => {
          state.seen = null;
          await assert.rejects(client[name](...args));
          assert.equal(state.seen, null, "the bad params were sent anyway");
        });
        continue;
      }
      if (!response && !error) continue;

      await t.test(name, options, async () => {
        state.seen = null;
        state.reply = error ? { error: error.json } : { result: response.json };
        const call = client[name](...args);
        if (error) {
          await assert.rejects(call, (e) => e.message === error.json.message && e.code === error.json.code);
        } else {
          assert.deepEqual(await call, response.json);
        }
        assert.equal(state.seen.path, "/json_rpc");
        assert.equal(state.seen.request.method, name);
        const sent = state.seen.request.params;
        if (method.params.length === 0) {
          assert.ok(
            sent === undefined || Object.keys(sent).length === 0,
            `no params expected, sent ${JSON.stringify(sent)}`,
          );
        } else {
          assert.deepEqual(sent, request.json);
        }
      });
    }
  });
}

test("reserved word param: get_transfers({in: true})", async (t) => {
  const { state, server, transport } = await fakeServer();
  t.after(() => server.close());
  state.reply = { result: {} };
  await new WalletRpc({ transport }).get_transfers(true);
  assert.deepEqual(state.seen.request.params, { in: true });
});

test("optional params that aren't set are left out", async (t) => {
  const { state, server, transport } = await fakeServer();
  t.after(() => server.close());
  state.reply = { result: { status: "OK" } };
  await new Monerod({ transport }).get_block(undefined, undefined, 912345);
  assert.deepEqual(state.seen.request.params, { height: 912345 });
});
