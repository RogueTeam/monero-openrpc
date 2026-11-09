#!/usr/bin/env python3
"""Print the OpenRPC schema of monerod's RPC, generated from a Monero checkout.

  python3 node_schema.py /path/to/monero > daemon.openrpc.json
"""
from rpc_schema import Target, main

MONEROD = Target(
    title="monerod RPC",
    url="http://127.0.0.1:18081/json_rpc",
    version_macro="CORE_RPC_VERSION",
    defs=["src/rpc/core_rpc_server_commands_defs.h",
          "src/cryptonote_protocol/cryptonote_protocol_defs.h",
          "src/rpc/rpc_handler.h"],
    server="src/rpc/core_rpc_server.h",
    impl="src/rpc/core_rpc_server.cpp",
    errors="src/rpc/core_rpc_server_error_codes.h",
)

if __name__ == "__main__":
    main(MONEROD)
