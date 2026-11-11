//! Rust types for monerod and monero-wallet-rpc, generated from `../../schemas`
//! by `generate.sh` (quicktype + `tools/fix_uint64.py`), plus the small JSON-RPC
//! envelope both servers use on `/json_rpc`.

pub mod jsonrpc;

/// monerod (daemon) RPC types. Generated: don't edit, run `generate.sh`.
#[allow(clippy::all)]
pub mod monerod;

/// monero-wallet-rpc types. Generated: don't edit, run `generate.sh`.
#[allow(clippy::all)]
pub mod walletrpc;
