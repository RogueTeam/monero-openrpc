# monero-rpc-schema

Generate an [OpenRPC](https://open-rpc.org/) schema for **monerod** and **monero-wallet-rpc** from the Monero C++ source, instead of writing it by hand. Then generate typed clients from it in the language you use.

```
python3 node_schema.py   /path/to/monero > schemas/daemon.openrpc.json
python3 wallet_schema.py /path/to/monero > schemas/wallet.openrpc.json
```

The argument is a Monero source checkout ([monero-project/monero](https://github.com/monero-project/monero)). The schema goes to stdout and warnings go to stderr. Python 3.8+, standard library only.

The schemas in `schemas/` were generated from `master` at commit `0120dda4` (2026-11-07; core RPC 3.19, wallet RPC 1.38), recorded in `schemas/source.json`. The generator also runs cleanly on the v0.18.5.0 and v0.18.5.4 tags.

## Layout

| Path | What it holds |
|---|---|
| `node_schema.py`, `wallet_schema.py` | Where each server's RPC is defined: header paths, version macro, default URL |
| `rpc_schema.py` | Shared code: the header parser and the OpenRPC builder |
| `schemas/` | The generated schemas, and the commit they came from |
| `tests/` | Tests for the generator, with a small fake checkout in `tests/fake_monero/` |
| `tools/` | Helpers for code generators and tests (below) |
| `examples/` | Generated clients for Go, Python, Rust and TypeScript, each with tests |

Keep the three `.py` files in the root together; the two scripts import `rpc_schema`. Adding another target means one more small file with its own `Target(...)`.

## Why

Monero has no machine-readable RPC spec, and the hand-written docs drift because nothing forces anyone to update them. See [monero-project/monero#9186](https://github.com/monero-project/monero/issues/9186). Generating the spec from the source means it can't fall behind the code.

## How it works

| | Daemon (`node_schema.py`) | Wallet (`wallet_schema.py`) |
|---|---|---|
| Structs | `src/rpc/core_rpc_server_commands_defs.h`, `src/cryptonote_protocol/cryptonote_protocol_defs.h`, `src/rpc/rpc_handler.h` | `src/wallet/wallet_rpc_server_commands_defs.h`, plus the core and protocol defs and `src/cryptonote_basic/subaddress_index.h` |
| Routes | `src/rpc/core_rpc_server.h` | `src/wallet/wallet_rpc_server.h` |
| Handlers (only to count positional params) | `src/rpc/core_rpc_server.cpp` | `src/wallet/wallet_rpc_server.cpp` |
| Error codes | `src/rpc/core_rpc_server_error_codes.h` | `src/wallet/wallet_rpc_server_error_codes.h` |

- **Fields** come from the epee `KV_SERIALIZE` maps, which define the wire format:
  - `KV_SERIALIZE_N` sets the wire name.
  - `KV_SERIALIZE_OPT` makes a field optional and gives its `default`.
  - `KV_SERIALIZE_PARENT` pulls in base fields.
  - `*_POD_AS_BLOB` fields are raw bytes, not hex (`contentEncoding: binary`). In JSON, epee writes them into the string as-is, so they may not be valid UTF-8.
  - A struct with no map of its own uses the one it inherits from its base class.
- **Methods** come from the route macros. `MAP_JON_RPC*` routes become OpenRPC `methods`. `MAP_URI_AUTO_JON2` / `MAP_URI_AUTO_BIN2` endpoints (`/get_height`, `/get_blocks.bin`, ...) go in `x-uri-endpoints`, because OpenRPC only describes JSON-RPC.
- **Branches.** Fields inside `if`/`else` in a map aren't required. If the same key is written in several branches, every shape is kept in an `anyOf`: for example, `get_output_distribution`'s `distribution` is an integer array or a blob, depending on `binary`.
- **Positional params.** For requests that aren't structs, the handler in the `.cpp` is checked: `on_get_block_hash` and `submit_block` take exactly one argument, and `get_block_count` takes none.
- **`required`** lists what the server always writes. Epee never rejects a request with missing fields (a missing field keeps its default), so schemas used only in requests have no `required`.
- **Arrays are never required.** Epee doesn't write an empty container at all (`if(!container.size()) return true;` in `keyvalue_serialization_overloads.h`), so a list with no items is simply missing from the response.
- **Integers** carry their C++ width as standard `minimum`/`maximum` bounds (e.g. 0 to 18446744073709551615 for `uint64_t`), the same way as `docs/schema/*.json` in the Monero repo. A custom `format` such as `uint64` would make strict validators like ajv reject the schema.
- **Type names.** Every schema has a `title` (`GetInfoResponse`, `TransferRequest`, `BlockCompleteEntry`, ...), which code generators use to name types.
- **Errors**: the `*_RPC_ERROR_CODE_*` defines, plus the standard JSON-RPC errors epee returns itself.
- Trailing `//` comments in the source become `description`s.

## Extensions

| Field | Where | Meaning |
|---|---|---|
| `x-uri-endpoints` | root | Non-JSON-RPC endpoints, with `encoding: json` or `epee-binary` |
| `x-bigint` | property | 64-bit integer. Amounts are in atomic units (10^12 per XMR), so values above about 9,007 XMR exceed 2^53; don't parse them into JS numbers or doubles |
| `x-alias-of` | method / endpoint | Same handler as an earlier name (`getblockcount` → `get_block_count`) |
| `x-hidden-in-restricted-mode` | method / endpoint | Unavailable when the node runs with `--restricted-rpc` |

## Tests

```
python3 -m unittest discover -s tests
MONERO_REPO=/path/to/monero python3 -m unittest discover -s tests
```

`make test` runs these plus every example's tests (needs Go, Rust, Node.js and pydantic). The first line runs the generator on `tests/fake_monero`, a tiny checkout with one example of each C++ construct it handles (inherited maps, `if`/`else` branches, hand-written `serialize` calls, positional params, defaults, comments, and so on), plus unit tests for the parser. With `MONERO_REPO` set it also runs on a real checkout, and if that checkout is at the commit in `schemas/source.json`, it checks that `schemas/` is up to date.

## Using it in CI

Regenerate the schemas and fail if anything changed:

```
python3 node_schema.py   "$MONERO_REPO" > schemas/daemon.openrpc.json
python3 wallet_schema.py "$MONERO_REPO" > schemas/wallet.openrpc.json
git diff --exit-code schemas/
```

## Generating clients

Each directory in `examples/` has a `generate.sh` that rebuilds its code from `schemas/`, the generated code itself, and tests. The tests use the request and response examples from the official docs ([monero-docs](https://github.com/monero-project/monero-docs), MIT, commit `9764d8dc`), extracted into `examples/fixtures/` by `tools/make_fixtures.py`. Each example is checked against the schema and marked:

- **ok** (263 of 282): matches the schema. Tests decode it, encode it back, and require the same JSON, which catches wrong field names and types, not just missing fields.
- **stale** (13): an older example that lacks fields added since. The Go tests require that it still decodes; the Python and Rust tests require that the fields reported missing are exactly the new ones.
- **docs-mistake** (6): contradicts the source (list below). Tests require that the generated code rejects or changes it, so a fixed docs example shows up as a test failure until the fixtures are regenerated.

| Language | Generator | Generated | Tests | Result |
|---|---|---|---|---|
| Go | quicktype 26.0.0 | types + a small JSON-RPC client (`jsonrpc/`) | `go test ./...` | pass: 259 typed docs examples, client tests, `go vet` clean |
| Python | datamodel-code-generator 0.91.0 | pydantic v2 models + a small client | `python -m unittest` | pass |
| Rust | quicktype 26.0.0 | serde types + JSON-RPC envelope helpers | `cargo test` | pass: 259 typed docs examples, 8 unit tests |
| TypeScript | @open-rpc/generator 2.1.1 | full client | `npm test` | pass: 226 docs examples type-checked, 113 calls through a fake server; 3 marked TODO for generator bugs |
| Java, C# | quicktype 26.0.0 | types | none | not tested (see below) |

To check that the tests can actually fail, I broke the generated code on purpose in each language (renamed a field, changed a field's type; in Rust also switched a uint64 field back to `i64`, in TypeScript renamed a param in the embedded schema). Every change made the tests fail.

### Go

```
cd examples/go && ./generate.sh && go test ./...
```

`generate.sh` runs three steps you'd also need in your own project:

```
python3 tools/to_jsonschema.py schemas/daemon.openrpc.json > daemon.schema.json
quicktype --src-lang schema --lang go --package monerod --just-types-and-package \
  -o types.go "daemon.schema.json#/definitions/"
python3 tools/fix_uint64.py go daemon.schema.json types.go
```

- `to_jsonschema.py` puts the component schemas under `definitions`, keyed by their titles, so the types get readable names (`GetInfoResponse`). Pointed at the OpenRPC file directly, quicktype makes names like `COMMANDRPCGETBANSRequestT`.
- `fix_uint64.py`: quicktype maps every integer to `int64` and ignores the bounds. Monero sends uint64 values above 2^63−1 (for example `already_generated_coins` = 18186022843595960691 in the docs' `get_miner_data` example). This switches those fields to `uint64`: 184 daemon fields and 93 wallet fields.

```go
client := jsonrpc.New("http://127.0.0.1:18081")
var info monerod.GetInfoResponse
err := client.Call(ctx, "get_info", &monerod.GetInfoRequest{}, &info)
```

There's no maintained OpenRPC client generator for Go, hence quicktype plus a 100-line client.

### Python

```
cd examples/python
pip install -r requirements-dev.txt && ./generate.sh && python -m unittest
```

pydantic's `int` has no size limit, so uint64 needs no fix, and the bounds from the schema are enforced. The `in` param of `get_transfers` (a Python keyword) becomes `in_` with alias `in`; the client sends wire names.

```python
node = Client("http://127.0.0.1:18081")
info = node.call("get_info", m.GetInfoRequest(), m.GetInfoResponse)
```

### Rust

```
cd examples/rust && ./generate.sh && cargo test
```

Same steps as Go, with `--lang rust --visibility public --skip-serializing-none --derive-partial-eq` and `fix_uint64.py rust` (`i64` → `u64`, the same 184 and 93 fields). Required fields aren't `Option`, so serde rejects a response that lacks one. There's no HTTP client in the example: `jsonrpc::request` builds the body and `jsonrpc::parse_response` reads the reply, so you can use any HTTP crate.

### TypeScript / JavaScript

```
cd examples/typescript && npm install && ./generate.sh && npm test
```

This uses the official OpenRPC generator, which builds a complete client. Two problems come from the tool, not the schema, and are kept visible as TODO tests:

- Wallet `get_transfers` has a param named `in`, a reserved word, so the generated code doesn't compile. `generate.sh` renames it in the generated type; the request still uses the name from the schema.
- The generated client passes its arguments to the validator from `@open-rpc/schema-utils-js` as an array, but for by-name methods the validator looks each param up by name. So by-name params are never validated (a docs example sending a string where an integer belongs goes out unchecked), and wallet `get_address_book`, whose param is named `entries`, is checked against `Array.prototype.entries` and always rejected. Call it through the transport instead:
  ```ts
  await client.rpc.request({ method: "get_address_book", params: { entries: [0, 1] } });
  ```

Also, `JSON.parse` loses precision on numbers above 2^53 (fields marked `x-bigint`). The generated client covers JSON-RPC methods only, not the `x-uri-endpoints`.

### Java and C# (not tested)

quicktype generates both, but I couldn't build them here (Maven Central and NuGet weren't reachable), so treat this as a starting point:

```
quicktype --src-lang schema --lang java   --package monero.rpc        -o src/ "daemon.schema.json#/definitions/"
quicktype --src-lang schema --lang csharp --namespace Monero.Rpc --framework SystemTextJson \
  -o Monerod.cs "daemon.schema.json#/definitions/"
```

Both map integers to signed `long`, so uint64 fields above 2^63−1 (like `already_generated_coins`) will overflow. `fix_uint64.py` only handles Go and Rust; for Java you'd want `BigInteger` and for C# `ulong` on the fields whose `maximum` is above 2^63−1.

### Older nodes

The examples are generated from `master`. A strict decoder (Rust, pydantic) rejects a response from an older node that lacks a field added since. Generate from the tag your nodes run (`git checkout v0.18.5.4`), or relax those fields.

## Mistakes in the official docs

Found while building the fixtures and confirmed against the source:

- `flush_cache`: the example sends `bad_txs`, which was removed.
- `check_reserve_proof`: has no `txid` param.
- `exchange_multisig_keys`: `multisig_info` is a list, not a string.
- `get_bulk_payments`: `min_block_height` is a number, not a string. Strings aren't accepted since #11337.
- `label_address`: the section shows `validate_address`'s params.
- `get_account_tags`: the example's `params` is `''` instead of an object.
- `label_account`: the response example shows `account_tags`, but the method returns an empty object.

## Warnings and limitations

- Two warnings are expected on current sources. The maps for `get_output_distribution`'s `distribution` and for `block_complete_entry` contain hand-written C++. The script still describes both, but flags them so someone checks them by hand.
- The parser is regex-based and follows Monero's header style. Anything it can't read produces a warning rather than being silently dropped.
- No request field is required at the wire level, even when a handler can't do anything useful without it. Which params really matter, descriptions and examples still need people.
- Behavior inside handlers isn't read, apart from counting positional params. That includes restrictions enforced in code (e.g. the wallet's `set_log_level` in restricted mode) and which error codes each method returns.
- The ZMQ interface and the P2P (levin) messages aren't covered: ZMQ uses a different serialization system, and P2P isn't JSON-RPC.
- The upcoming serialization rewrite ([#8867](https://github.com/monero-project/monero/pull/8867)) changes some of the macros this reads; the generator will need updates when it lands.
