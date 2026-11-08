#!/usr/bin/env python3
"""Print the OpenRPC schema of monero-wallet-rpc, generated from a Monero checkout.

  python3 wallet_schema.py /path/to/monero > wallet.openrpc.json
"""
from rpc_schema import Target, main

WALLET_RPC = Target(
    title="monero-wallet-rpc",
    url="http://127.0.0.1:18082/json_rpc",
    version_macro="WALLET_RPC_VERSION",
    defs=["src/wallet/wallet_rpc_server_commands_defs.h",
          "src/rpc/core_rpc_server_commands_defs.h",
          "src/cryptonote_protocol/cryptonote_protocol_defs.h",
          "src/cryptonote_basic/subaddress_index.h"],
    server="src/wallet/wallet_rpc_server.h",
)

if __name__ == "__main__":
    main(WALLET_RPC)
