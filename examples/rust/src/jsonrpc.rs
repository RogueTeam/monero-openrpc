//! The JSON-RPC 2.0 envelope monerod and monero-wallet-rpc use on `/json_rpc`.
//!
//! There's no HTTP client here, so you can use whichever one you like: POST the
//! string from [`request`] to `http://host:port/json_rpc` and hand the response body
//! to [`parse_response`]. Endpoints listed under `x-uri-endpoints` in the schema
//! (`/get_height`, `/get_transactions`, ...) take the request type as the whole
//! body instead, with no envelope: `serde_json::to_string(&req)`.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

#[derive(Serialize)]
struct Request<'a, P> {
    jsonrpc: &'a str,
    id: &'a str,
    method: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<P>,
}

/// Builds a JSON-RPC request body. `params` is a request struct (params by name),
/// an array or tuple (by position, e.g. `[912345u64]` for `on_get_block_hash`),
/// or `None::<()>` for methods without params.
pub fn request<P: Serialize>(method: &str, params: Option<P>) -> serde_json::Result<String> {
    serde_json::to_string(&Request { jsonrpc: "2.0", id: "0", method, params })
}

/// A JSON-RPC error object, e.g. `{"code": -2, "message": "..."}`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Error {
    pub code: i64,
    pub message: String,
}

/// Why a call failed: the server returned an error, or the body didn't parse.
#[derive(Debug)]
pub enum CallError {
    Rpc(Error),
    Json(serde_json::Error),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallError::Rpc(e) => write!(f, "rpc error {}: {}", e.code, e.message),
            CallError::Json(e) => write!(f, "bad response: {e}"),
        }
    }
}

impl std::error::Error for CallError {}

#[derive(Deserialize)]
struct Response<R> {
    result: Option<R>,
    error: Option<Error>,
}

/// Parses a JSON-RPC response body into the method's response type.
pub fn parse_response<R: DeserializeOwned>(body: &str) -> Result<R, CallError> {
    let r: Response<R> = serde_json::from_str(body).map_err(CallError::Json)?;
    match (r.error, r.result) {
        (Some(e), _) => Err(CallError::Rpc(e)),
        (None, Some(result)) => Ok(result),
        (None, None) => Err(CallError::Json(serde::de::Error::missing_field("result"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{monerod, walletrpc};
    use serde_json::{json, Value};

    fn parsed(s: &str) -> Value {
        serde_json::from_str(s).unwrap()
    }

    #[test]
    fn request_by_name_omits_unset_optional_params() {
        let req = monerod::GetBlockRequest {
            height: Some(912345),
            ..serde_json::from_str("{}").unwrap() // every request param is optional
        };
        let body = request("get_block", Some(req)).unwrap();
        assert_eq!(
            parsed(&body),
            json!({"jsonrpc": "2.0", "id": "0", "method": "get_block", "params": {"height": 912345}})
        );
    }

    #[test]
    fn request_by_position() {
        let body = request("on_get_block_hash", Some([912345u64])).unwrap();
        assert_eq!(parsed(&body)["params"], json!([912345]));
    }

    #[test]
    fn request_without_params() {
        let body = request("get_block_count", None::<()>).unwrap();
        assert_eq!(parsed(&body), json!({"jsonrpc": "2.0", "id": "0", "method": "get_block_count"}));
    }

    #[test]
    fn reserved_word_param() {
        // `in` is a Rust keyword; quicktype renames the field but keeps the JSON name.
        let req = walletrpc::GetTransfersRequest {
            get_transfers_request_in: Some(true),
            ..serde_json::from_str("{}").unwrap()
        };
        assert_eq!(serde_json::to_value(&req).unwrap(), json!({"in": true}));
    }

    #[test]
    fn parse_result() {
        let body = r#"{"id":"0","jsonrpc":"2.0","result":{"count":3195144,"status":"OK","untrusted":false}}"#;
        let r: monerod::GetblockcountResponse = parse_response(body).unwrap();
        assert_eq!(r.count, 3195144);
    }

    #[test]
    fn parse_error() {
        let body = r#"{"error":{"code":-2,"message":"Requested block height: 99999999 greater than current top block height: 3195143"},"id":"0","jsonrpc":"2.0"}"#;
        match parse_response::<monerod::GetblockcountResponse>(body) {
            Err(CallError::Rpc(e)) => assert_eq!(e.code, -2),
            other => panic!("expected an rpc error, got {other:?}"),
        }
    }

    #[test]
    fn parse_missing_result() {
        let body = r#"{"id":"0","jsonrpc":"2.0"}"#;
        assert!(matches!(
            parse_response::<monerod::GetblockcountResponse>(body),
            Err(CallError::Json(_))
        ));
    }

    #[test]
    fn uint64_above_int64() {
        // already_generated_coins passed 2^63-1 long ago; i64 (quicktype's default) can't hold it.
        let body = r#"{"already_generated_coins":18446744073709551615,"difficulty":"0x1",
            "height":1,"major_version":16,"median_weight":300000,"prev_id":"","seed_hash":"",
            "status":"OK"}"#;
        let r: monerod::GetminerdataResponse = serde_json::from_str(body).unwrap();
        assert_eq!(r.already_generated_coins, u64::MAX);
    }
}
