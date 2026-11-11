use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type GetbansRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetminerdataRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetAlternateChainsRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetConnectionsRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetHeightRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetLimitRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetNetStatsRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetVersionRequest = HashMap<String, Option<serde_json::Value>>;
pub type MiningStatusRequest = HashMap<String, Option<serde_json::Value>>;
pub type SaveBcRequest = HashMap<String, Option<serde_json::Value>>;
pub type StopDaemonRequest = HashMap<String, Option<serde_json::Value>>;
pub type StopMiningRequest = HashMap<String, Option<serde_json::Value>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddAuxPowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aux_pow: Option<Vec<AddAuxPowAuxPow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocktemplate_blob: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddAuxPowAuxPow {
    pub hash: String,

    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddAuxPowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aux_pow: Option<Vec<AddAuxPowAuxPow>>,

    pub blockhashing_blob: String,

    pub blocktemplate_blob: String,

    pub merkle_root: String,

    pub merkle_tree_depth: u64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BannedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BannedResponse {
    pub banned: bool,

    pub seconds: i64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalcpowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_blob: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_version: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlushCacheRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_blocks: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlushCacheResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlushTransactionPoolRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlushTransactionPoolResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateblocksRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_of_blocks: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_block: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_nonce: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateblocksResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<String>>,

    pub height: u64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetbansResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bans: Option<Vec<GetbansBan>>,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetbansBan {
    pub host: String,

    pub ip: i64,

    pub seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetblockcountResponse {
    pub count: u64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetblocktemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_nonce: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_block: Option<String>,

    /// max 255 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetblocktemplateResponse {
    pub blockhashing_blob: String,

    pub blocktemplate_blob: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative_weight: Option<u64>,

    pub difficulty: u64,

    pub difficulty_top64: u64,

    pub expected_reward: u64,

    pub height: u64,

    pub next_seed_hash: String,

    pub prev_hash: String,

    pub reserved_offset: u64,

    pub seed_hash: String,

    pub seed_height: u64,

    pub status: String,

    pub wide_difficulty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetminerdataResponse {
    pub already_generated_coins: u64,

    pub difficulty: String,

    pub height: u64,

    pub major_version: i32,

    pub median_weight: u64,

    pub prev_id: String,

    pub seed_hash: String,

    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_backlog: Option<Vec<GetminerdataResponseTxBacklogEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetminerdataResponseTxBacklogEntry {
    pub fee: u64,

    pub id: String,

    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAlternateChainsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chains: Option<Vec<GetAlternateChainsChainInfo>>,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAlternateChainsChainInfo {
    pub block_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hashes: Option<Vec<String>>,

    pub difficulty: u64,

    pub difficulty_top64: u64,

    pub height: u64,

    pub length: u64,

    pub main_chain_parent_block: String,

    pub wide_difficulty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAltBlocksHashesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAltBlocksHashesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blks_hashes: Option<Vec<String>>,

    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBaseFeeEstimateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_blocks: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBaseFeeEstimateResponse {
    pub credits: u64,

    pub fee: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantization_mask: Option<u64>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_pow_hash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockResponse {
    pub blob: String,

    pub block_header: BlockHeaderResponse,

    pub credits: u64,

    pub json: String,

    pub miner_tx_hash: String,

    pub status: String,

    pub top_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hashes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeaderResponse {
    pub block_size: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_weight: Option<u64>,

    pub cumulative_difficulty: u64,

    pub cumulative_difficulty_top64: u64,

    pub depth: u64,

    pub difficulty: u64,

    pub difficulty_top64: u64,

    pub hash: String,

    pub height: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_term_weight: Option<u64>,

    pub major_version: i32,

    pub miner_tx_hash: String,

    pub minor_version: i32,

    pub nonce: i64,

    pub num_txes: u64,

    pub orphan_status: bool,

    pub pow_hash: String,

    pub prev_hash: String,

    pub reward: u64,

    pub timestamp: u64,

    pub wide_cumulative_difficulty: String,

    pub wide_difficulty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksByHeightRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heights: Option<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksByHeightResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<BlockCompleteEntry>>,

    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockCompleteEntry {
    pub block: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_weight: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruned: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<Vec<Tx>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tx {
    PurpleString(String),

    TxBlobEntry(TxBlobEntry),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxBlobEntry {
    pub blob: String,

    pub prunable_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksFastRequest {
    /// first 10 blocks id goes sequential, next goes in pow(2,n) offset, like 2, 4, 8, 16, 32,
    /// 64 and so on, and the last one is always genesis block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids_exclusive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_block_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_miner_tx: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_info_since: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_info: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksFastResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_pool_txs: Option<Vec<GetBlocksFastPoolTxInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<BlockCompleteEntry>>,

    pub credits: u64,

    pub current_height: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_time: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_indices: Option<Vec<GetBlocksFastBlockOutputIndices>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_info_extent: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_added_pool_txids: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_pool_txids: Option<String>,

    pub start_height: u64,

    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_block_hash: Option<String>,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksFastPoolTxInfo {
    pub double_spend_seen: bool,

    pub tx_blob: String,

    pub tx_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksFastBlockOutputIndices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<GetBlocksFastTxOutputIndices>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlocksFastTxOutputIndices {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockHeadersRangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_pow_hash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockHeadersRangeResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<BlockHeaderResponse>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockHeaderByHashRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_pow_hash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockHeaderByHashResponse {
    pub block_header: BlockHeaderResponse,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_headers: Option<Vec<BlockHeaderResponse>>,

    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockHeaderByHeightRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_pow_hash: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBlockHeaderByHeightResponse {
    pub block_header: BlockHeaderResponse,

    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCoinbaseTxSumRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCoinbaseTxSumResponse {
    pub credits: u64,

    pub emission_amount: u64,

    pub emission_amount_top64: u64,

    pub fee_amount: u64,

    pub fee_amount_top64: u64,

    pub status: String,

    pub top_hash: String,

    pub wide_emission_amount: String,

    pub wide_fee_amount: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetConnectionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionInfo>>,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub address: String,

    pub address_type: i32,

    pub avg_download: u64,

    pub avg_upload: u64,

    pub connection_id: String,

    pub current_download: u64,

    pub current_upload: u64,

    pub height: u64,

    pub host: String,

    pub incoming: bool,

    pub ip: String,

    pub live_time: u64,

    pub local_ip: bool,

    pub localhost: bool,

    pub peer_id: String,

    pub port: String,

    pub pruning_seed: i64,

    pub recv_count: u64,

    pub recv_idle_time: u64,

    pub rpc_credits_per_hash: i64,

    pub rpc_port: i32,

    pub send_count: u64,

    pub send_idle_time: u64,

    pub state: String,

    pub support_flags: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHashesFastRequest {
    /// first 10 blocks id goes sequential, next goes in pow(2,n) offset, like 2, 4, 8, 16, 32,
    /// 64 and so on, and the last one is always genesis block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHashesFastResponse {
    pub credits: u64,

    pub current_height: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_block_ids: Option<String>,

    pub start_height: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeightResponse {
    pub hash: String,

    pub height: u64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInfoResponse {
    pub adjusted_time: u64,

    pub alt_blocks_count: u64,

    pub block_size_limit: u64,

    pub block_size_median: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_weight_limit: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_weight_median: Option<u64>,

    pub busy_syncing: bool,

    pub credits: u64,

    pub cumulative_difficulty: u64,

    pub cumulative_difficulty_top64: u64,

    pub database_size: u64,

    pub difficulty: u64,

    pub difficulty_top64: u64,

    pub free_space: u64,

    pub grey_peerlist_size: u64,

    pub height: u64,

    pub incoming_connections_count: u64,

    pub mainnet: bool,

    pub nettype: String,

    pub offline: bool,

    pub outgoing_connections_count: u64,

    pub restricted: bool,

    pub rpc_connections_count: u64,

    pub stagenet: bool,

    pub start_time: u64,

    pub status: String,

    pub synchronized: bool,

    pub target: u64,

    pub target_height: u64,

    pub testnet: bool,

    pub top_block_hash: String,

    pub top_hash: String,

    pub tx_count: u64,

    pub tx_pool_size: u64,

    pub update_available: bool,

    pub version: String,

    pub white_peerlist_size: u64,

    pub wide_cumulative_difficulty: String,

    pub wide_difficulty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLastBlockHeaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_pow_hash: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLastBlockHeaderResponse {
    pub block_header: BlockHeaderResponse,

    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLimitResponse {
    pub limit_down: u64,

    pub limit_up: u64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetStatsResponse {
    pub start_time: u64,

    pub status: String,

    pub total_bytes_in: u64,

    pub total_bytes_out: u64,

    pub total_packets_in: u64,

    pub total_packets_out: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_txid: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<GetOutputsOut>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<Vec<GetOutputsOutkey>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsOutkey {
    pub height: u64,

    pub key: String,

    pub mask: String,

    pub txid: String,

    pub unlocked: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsBinRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_txid: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<GetOutputsOut>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsBinResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<Vec<GetOutputsBinOutkey>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputsBinOutkey {
    pub height: u64,

    pub key: String,

    pub mask: String,

    pub txid: String,

    pub unlocked: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputDistributionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cumulative: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputDistributionResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributions: Option<Vec<GetOutputDistributionDistribution>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputDistributionDistribution {
    pub amount: u64,

    pub base: u64,

    pub binary: bool,

    pub compress: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed_data: Option<String>,

    pub distribution: Option<Distribution>,

    pub start_height: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Distribution {
    IntegerArray(Vec<i64>),

    PurpleString(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputHistogramRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_cutoff: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputHistogramResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histogram: Option<Vec<GetOutputHistogramEntry>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOutputHistogramEntry {
    pub amount: u64,

    pub recent_instances: u64,

    pub total_instances: u64,

    pub unlocked_instances: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPeerListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_only: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPeerListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gray_list: Option<Vec<Peer>>,

    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_list: Option<Vec<Peer>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub host: String,

    pub id: u64,

    pub ip: i64,

    pub last_seen: u64,

    pub port: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pruning_seed: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_credits_per_hash: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_port: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPublicNodesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gray: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_blocked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub white: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPublicNodesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gray: Option<Vec<PublicNode>>,

    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub white: Option<Vec<PublicNode>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicNode {
    pub host: String,

    pub last_seen: u64,

    pub rpc_credits_per_hash: i64,

    pub rpc_port: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_as_json: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs_hashes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionsResponse {
    pub credits: u64,

    /// not found transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missed_tx: Option<Vec<String>>,

    pub status: String,

    pub top_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<Vec<GetTransactionsEntry>>,

    /// transactions blobs as hex (old compat)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs_as_hex: Option<Vec<String>>,

    /// transactions decoded as json (old compat)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs_as_json: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionsEntry {
    pub as_hex: String,

    pub as_json: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,

    pub double_spend_seen: bool,

    pub in_pool: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_indices: Option<Vec<u64>>,

    pub prunable_as_hex: String,

    pub prunable_hash: String,

    pub pruned_as_hex: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_timestamp: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relayed: Option<bool>,

    pub tx_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_key_images: Option<Vec<SpentKeyImageInfo>>,

    pub status: String,

    pub top_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<TxInfo>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpentKeyImageInfo {
    pub id_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs_hashes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxInfo {
    pub blob_size: u64,

    pub do_not_relay: bool,

    pub double_spend_seen: bool,

    pub fee: u64,

    pub id_hash: String,

    pub kept_by_block: bool,

    pub last_failed_height: u64,

    pub last_failed_id_hash: String,

    pub last_relayed_time: u64,

    pub max_used_block_height: u64,

    pub max_used_block_id_hash: String,

    pub receive_time: u64,

    pub relayed: bool,

    pub tx_blob: String,

    /// TODO - expose this data directly
    pub tx_json: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolBacklogRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolBacklogResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlog: Option<String>,

    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolHashesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolHashesResponse {
    pub credits: u64,

    pub status: String,

    pub top_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hashes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolHashesBinRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolHashesBinResponse {
    pub credits: u64,

    pub status: String,

    pub top_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hashes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolStatsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionPoolStatsResponse {
    pub credits: u64,

    pub pool_stats: TxpoolStats,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxpoolStats {
    pub bytes_max: i64,

    pub bytes_med: i64,

    pub bytes_min: i64,

    pub bytes_total: u64,

    pub fee_total: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histo: Option<Vec<TxpoolHisto>>,

    #[serde(rename = "histo_98pc")]
    pub histo_98_pc: u64,

    #[serde(rename = "num_10m")]
    pub num_10_m: i64,

    pub num_double_spends: i64,

    pub num_failing: i64,

    pub num_not_relayed: i64,

    pub oldest: u64,

    pub txs_total: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TxpoolHisto {
    pub bytes: u64,

    pub txs: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxidsLooseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_matching_bits: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxidsLooseResponse {
    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxGlobalOutputsIndexesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxGlobalOutputsIndexesResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_indexes: Option<Vec<u64>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_forks: Option<Vec<GetVersionHfEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nettype: Option<String>,

    pub release: bool,

    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_height: Option<u64>,

    pub version: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionHfEntry {
    pub height: u64,

    pub hf_version: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardForkInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HardForkInfoResponse {
    pub credits: u64,

    pub earliest_height: u64,

    pub enabled: bool,

    pub state: i64,

    pub status: String,

    pub threshold: i64,

    pub top_hash: String,

    pub version: i32,

    pub votes: i64,

    pub voting: i32,

    pub window: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InPeersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_peers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InPeersResponse {
    pub in_peers: i64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsKeyImageSpentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_images: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsKeyImageSpentResponse {
    pub credits: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_status: Option<Vec<i32>>,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningStatusResponse {
    pub active: bool,

    pub address: String,

    pub bg_idle_threshold: i32,

    pub bg_ignore_battery: bool,

    pub bg_min_idle_seconds: i32,

    pub bg_target: i32,

    pub block_reward: u64,

    pub block_target: i64,

    pub difficulty: u64,

    pub difficulty_top64: u64,

    pub is_background_mining_enabled: bool,

    pub pow_algorithm: String,

    pub speed: u64,

    pub status: String,

    pub threads_count: i64,

    pub wide_difficulty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutPeersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_peers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutPeersResponse {
    pub out_peers: i64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopBlocksRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_txs: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nblocks: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopBlocksResponse {
    pub height: u64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PruneBlockchainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PruneBlockchainResponse {
    pub pruned: bool,

    pub pruning_seed: i64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelayTxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelayTxResponse {
    pub credits: u64,

    pub status: String,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveBcResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendRawTxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_relay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_sanity_checks: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_as_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendRawTxResponse {
    pub credits: u64,

    pub double_spend: bool,

    pub fee_too_low: bool,

    pub invalid_input: bool,

    pub invalid_output: bool,

    pub low_mixin: bool,

    pub nonzero_unlock_time: bool,

    pub not_relayed: bool,

    pub overspend: bool,

    pub reason: String,

    pub sanity_check_failed: bool,

    pub status: String,

    pub too_big: bool,

    pub too_few_outputs: bool,

    pub top_hash: String,

    pub tx_extra_too_big: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetbansRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bans: Option<Vec<SetbansBan>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetbansBan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetbansResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLimitRequest {
    /// all limits (for get and set) are kB/s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_down: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_up: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLimitResponse {
    pub limit_down: i64,

    pub limit_up: i64,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogCategoriesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogCategoriesResponse {
    pub categories: String,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogHashRateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogHashRateResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogLevelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogLevelResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartMiningRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_background_mining: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_battery: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub miner_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartMiningResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopDaemonResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopMiningResponse {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitblockResponse {
    pub block_id: String,

    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncInfoResponse {
    pub credits: u64,

    pub height: u64,

    pub next_needed_pruning_seed: i64,

    pub overview: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<SyncInfoPeer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<SyncInfoSpan>>,

    pub status: String,

    pub target_height: u64,

    pub top_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncInfoPeer {
    pub info: ConnectionInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncInfoSpan {
    pub connection_id: String,

    pub nblocks: u64,

    pub rate: i64,

    pub remote_address: String,

    pub size: u64,

    pub speed: i64,

    pub start_block_height: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub auto_uri: String,

    pub hash: String,

    pub path: String,

    pub status: String,

    pub update: bool,

    pub user_uri: String,

    pub version: String,
}
