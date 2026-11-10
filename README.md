# monero-rpc-schema

Generate an [OpenRPC](https://open-rpc.org/) schema for **monerod** and **monero-wallet-rpc** from the Monero C++ source, instead of writing it by hand.

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

The first line runs the generator on `tests/fake_monero`, a tiny checkout with one example of each C++ construct it handles (inherited maps, `if`/`else` branches, hand-written `serialize` calls, positional params, defaults, comments, and so on), plus unit tests for the parser. With `MONERO_REPO` set it also runs on a real checkout, and if that checkout is at the commit in `schemas/source.json`, it checks that `schemas/` is up to date.

## Using it in CI

Regenerate the schemas and fail if anything changed:

```
python3 node_schema.py   "$MONERO_REPO" > schemas/daemon.openrpc.json
python3 wallet_schema.py "$MONERO_REPO" > schemas/wallet.openrpc.json
git diff --exit-code schemas/
```

## Warnings and limitations

- Two warnings are expected on current sources. The maps for `get_output_distribution`'s `distribution` and for `block_complete_entry` contain hand-written C++. The script still describes both, but flags them so someone checks them by hand.
- The parser is regex-based and follows Monero's header style. Anything it can't read produces a warning rather than being silently dropped.
- No request field is required at the wire level, even when a handler can't do anything useful without it. Which params really matter, descriptions and examples still need people.
- Behavior inside handlers isn't read, apart from counting positional params. That includes restrictions enforced in code (e.g. the wallet's `set_log_level` in restricted mode) and which error codes each method returns.
- The ZMQ interface and the P2P (levin) messages aren't covered: ZMQ uses a different serialization system, and P2P isn't JSON-RPC.
- The upcoming serialization rewrite ([#8867](https://github.com/monero-project/monero/pull/8867)) changes some of the macros this reads; the generator will need updates when it lands.
