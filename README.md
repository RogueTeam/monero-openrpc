# monero-rpc-schema

Generate an [OpenRPC](https://open-rpc.org/) schema for **monerod** from the Monero C++ source, instead of writing it by hand.

```
python3 node_schema.py /path/to/monero > schemas/daemon.openrpc.json
```

The argument is a Monero source checkout ([monero-project/monero](https://github.com/monero-project/monero)). The schema goes to stdout and warnings go to stderr. Python 3.8+, standard library only.

## Why

Monero has no machine-readable RPC spec, and the hand-written docs drift because nothing forces anyone to update them. See [monero-project/monero#9186](https://github.com/monero-project/monero/issues/9186). Generating the spec from the source means it can't fall behind the code.

## How it works

- **Fields** come from the epee `KV_SERIALIZE` maps in `src/rpc/core_rpc_server_commands_defs.h` (plus the protocol defs and `src/rpc/rpc_handler.h`), which define the wire format. `KV_SERIALIZE_N` sets the wire name, `KV_SERIALIZE_OPT` makes a field optional, `KV_SERIALIZE_PARENT` pulls in base fields, and `*_POD_AS_BLOB` fields are raw bytes, not hex (`contentEncoding: binary`).
- **Methods** come from the `MAP_JON_RPC*` routes in `src/rpc/core_rpc_server.h`.
- Anything the parser can't read produces a warning rather than being silently dropped.
