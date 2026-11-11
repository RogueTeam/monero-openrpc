use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type AutoRefreshResponse = HashMap<String, Option<serde_json::Value>>;
pub type ChangeWalletPasswordResponse = HashMap<String, Option<serde_json::Value>>;
pub type CloseWalletResponse = HashMap<String, Option<serde_json::Value>>;
pub type CreateWalletResponse = HashMap<String, Option<serde_json::Value>>;
pub type DeleteAddressBookEntryResponse = HashMap<String, Option<serde_json::Value>>;
pub type EditAddressBookEntryResponse = HashMap<String, Option<serde_json::Value>>;
pub type ExportMultisigRequest = HashMap<String, Option<serde_json::Value>>;
pub type FreezeResponse = HashMap<String, Option<serde_json::Value>>;
pub type GetAccountTagsRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetDefaultFeePriorityRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetWalletInfoRequest = HashMap<String, Option<serde_json::Value>>;
pub type IsMultisigRequest = HashMap<String, Option<serde_json::Value>>;
pub type LabelAccountResponse = HashMap<String, Option<serde_json::Value>>;
pub type LabelAddressResponse = HashMap<String, Option<serde_json::Value>>;
pub type OpenWalletResponse = HashMap<String, Option<serde_json::Value>>;
pub type RescanBlockchainResponse = HashMap<String, Option<serde_json::Value>>;
pub type RescanSpentRequest = HashMap<String, Option<serde_json::Value>>;
pub type RescanSpentResponse = HashMap<String, Option<serde_json::Value>>;
pub type ScanTxResponse = HashMap<String, Option<serde_json::Value>>;
pub type SetupBackgroundSyncResponse = HashMap<String, Option<serde_json::Value>>;
pub type SetAccountTagDescriptionResponse = HashMap<String, Option<serde_json::Value>>;
pub type SetAttributeResponse = HashMap<String, Option<serde_json::Value>>;
pub type SetDaemonResponse = HashMap<String, Option<serde_json::Value>>;
pub type SetSubaddrLookaheadResponse = HashMap<String, Option<serde_json::Value>>;
pub type SetTxNotesResponse = HashMap<String, Option<serde_json::Value>>;
pub type StartBackgroundSyncRequest = HashMap<String, Option<serde_json::Value>>;
pub type StartBackgroundSyncResponse = HashMap<String, Option<serde_json::Value>>;
pub type StopBackgroundSyncResponse = HashMap<String, Option<serde_json::Value>>;
pub type StopWalletRequest = HashMap<String, Option<serde_json::Value>>;
pub type StopWalletResponse = HashMap<String, Option<serde_json::Value>>;
pub type StoreRequest = HashMap<String, Option<serde_json::Value>>;
pub type StoreResponse = HashMap<String, Option<serde_json::Value>>;
pub type TagAccountsResponse = HashMap<String, Option<serde_json::Value>>;
pub type ThawResponse = HashMap<String, Option<serde_json::Value>>;
pub type UntagAccountsResponse = HashMap<String, Option<serde_json::Value>>;
pub type GetHeightRequest = HashMap<String, Option<serde_json::Value>>;
pub type GetVersionRequest = HashMap<String, Option<serde_json::Value>>;
pub type SetLogLevelResponse = HashMap<String, Option<serde_json::Value>>;
pub type StartMiningResponse = HashMap<String, Option<serde_json::Value>>;
pub type StopMiningRequest = HashMap<String, Option<serde_json::Value>>;
pub type StopMiningResponse = HashMap<String, Option<serde_json::Value>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddAddressBookEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddAddressBookEntryResponse {
    pub index: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoRefreshRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    /// seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeWalletPasswordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckReserveProofRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckReserveProofResponse {
    pub good: bool,

    pub spent: u64,

    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSpendProofRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSpendProofResponse {
    pub good: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckTxKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckTxKeyResponse {
    pub confirmations: u64,

    pub in_pool: bool,

    pub received: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckTxProofRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckTxProofResponse {
    pub confirmations: u64,

    pub good: bool,

    pub in_pool: bool,

    pub received: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseWalletRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autosave_current: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub account_index: i64,

    /// the 0-th address for convenience
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAddressResponse {
    pub address: String,

    pub address_index: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_indices: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyseed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteAddressBookEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisig_txset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsigned_txset: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeTransferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Vec<DescribeTransferTransferDescription>>,

    pub summary: DescribeTransferTxsetSummary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeTransferTransferDescription {
    pub amount_in: u64,

    pub amount_out: u64,

    pub change_address: String,

    pub change_amount: u64,

    pub dummy_outputs: i64,

    pub extra: String,

    pub fee: u64,

    pub payment_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<DescribeTransferRecipient>>,

    pub ring_size: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<DescribeTransferSource>>,

    pub unlock_time: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeTransferRecipient {
    pub address: String,

    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeTransferSource {
    pub amount: u64,

    pub global_index: u64,

    pub pubkey: String,

    pub rct: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeTransferTxsetSummary {
    pub amount_in: u64,

    pub amount_out: u64,

    pub change_address: String,

    pub change_amount: u64,

    pub fee: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<DescribeTransferRecipient>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditAddressBookEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_address: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_description: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimateTxSizeAndWeightRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_inputs: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_outputs: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rct: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimateTxSizeAndWeightResponse {
    pub size: u64,

    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeMultisigKeysRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update_use_with_caution: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisig_info: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeMultisigKeysResponse {
    pub address: String,

    pub multisig_info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportKeyImagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportKeyImagesResponse {
    pub offset: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_key_images: Option<Vec<ExportKeyImagesSignedKeyImage>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportKeyImagesSignedKeyImage {
    pub key_image: String,

    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportMultisigResponse {
    pub info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportOutputsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportOutputsResponse {
    pub outputs_data_hex: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreezeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_image: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrozenRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_image: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrozenResponse {
    pub frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateFromKeysRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autosave_current: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spendkey: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewkey: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateFromKeysResponse {
    pub address: String,

    pub info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAccountsRequest {
    /// allow regular expression filters if set to true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_balances: Option<bool>,

    /// all accounts if empty, otherwise those accounts with this tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAccountsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddress_accounts: Option<Vec<GetAccountsSubaddressAccountInfo>>,

    pub total_balance: u64,

    pub total_unlocked_balance: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAccountsSubaddressAccountInfo {
    pub account_index: i64,

    pub balance: u64,

    pub base_address: String,

    pub label: String,

    pub tag: String,

    pub unlocked_balance: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAccountTagsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tags: Option<Vec<GetAccountTagsAccountTagInfo>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAccountTagsAccountTagInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<i64>>,

    pub label: String,

    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_index: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_accounts: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressResponse {
    /// to remain compatible with older RPC format
    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GetAddressAddressInfo>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressAddressInfo {
    pub account_index: i64,

    pub address: String,

    pub address_index: i64,

    pub label: String,

    pub used: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressBookEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressBookEntryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<GetAddressBookEntryEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressBookEntryEntry {
    pub address: String,

    pub description: String,

    pub index: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressIndexRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAddressIndexResponse {
    pub index: SubaddressIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubaddressIndex {
    pub major: i64,

    pub minor: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributeResponse {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBalanceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_indices: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_accounts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBalanceResponse {
    pub balance: u64,

    pub blocks_to_unlock: u64,

    pub multisig_import_needed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_subaddress: Option<Vec<GetBalancePerSubaddressInfo>>,

    pub time_to_unlock: u64,

    pub unlocked_balance: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBalancePerSubaddressInfo {
    pub account_index: i64,

    pub address: String,

    pub address_index: i64,

    pub balance: u64,

    pub blocks_to_unlock: u64,

    pub label: String,

    pub num_unspent_outputs: u64,

    pub time_to_unlock: u64,

    pub unlocked_balance: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBulkPaymentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_block_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBulkPaymentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<PaymentDetails>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentDetails {
    pub address: String,

    pub amount: u64,

    pub block_height: u64,

    pub locked: bool,

    pub payment_id: String,

    pub subaddr_index: SubaddressIndex,

    pub tx_hash: String,

    pub unlock_time: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDefaultFeePriorityResponse {
    pub priority: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLanguagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyseed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLanguagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages_local: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMultisigKeyExchangeBoosterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisig_info: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_signers: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMultisigKeyExchangeBoosterResponse {
    pub multisig_info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPaymentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPaymentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<PaymentDetails>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReserveProofRequest {
    /// ignored when `all` is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,

    /// ignored when `all` is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReserveProofResponse {
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpendProofRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpendProofResponse {
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransfersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_accounts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_height: Option<bool>,

    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_transfers_request_in: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_pool: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransfersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<TransferEntry>>,

    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_transfers_response_in: Option<Vec<TransferEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out: Option<Vec<TransferEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<Vec<TransferEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<Vec<TransferEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferEntry {
    pub address: String,

    pub amount: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<TransferDestination>>,

    pub double_spend_seen: bool,

    pub fee: u64,

    pub height: u64,

    pub locked: bool,

    pub note: String,

    pub payment_id: String,

    pub subaddr_index: SubaddressIndex,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices: Option<Vec<SubaddressIndex>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_confirmations_threshold: Option<u64>,

    pub timestamp: u64,

    pub txid: String,

    #[serde(rename = "type")]
    pub transfer_entry_type: String,

    pub unlock_time: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferDestination {
    pub address: String,

    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransferByTxidRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_accounts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTransferByTxidResponse {
    pub transfer: TransferEntry,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Vec<TransferEntry>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxKeyResponse {
    pub tx_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxNotesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxNotesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxProofRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTxProofResponse {
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWalletInfoResponse {
    pub address: String,

    pub daemon_address: String,

    pub daemon_block_height: u64,

    pub daemon_proxy: String,

    pub daemon_rpc_version: i64,

    pub daemon_ssl: bool,

    pub description: String,

    pub filename: String,

    pub network_type: String,

    /// Legacy | Multisig | Polyseed
    pub seed_type: String,

    pub wallet_block_height: u64,

    /// Normal | Multisig | Watch-Only | Background
    pub wallet_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportKeyImagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_key_images: Option<Vec<ImportKeyImagesSignedKeyImage>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportKeyImagesSignedKeyImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportKeyImagesResponse {
    pub height: u64,

    pub spent: u64,

    pub unspent: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportMultisigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_after_import: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportMultisigResponse {
    pub n_outputs: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportOutputsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs_data_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportOutputsResponse {
    pub num_imported: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomingTransfersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_accounts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomingTransfersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Vec<TransferDetails>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferDetails {
    pub amount: u64,

    pub block_height: u64,

    pub frozen: bool,

    pub global_index: u64,

    pub key_image: String,

    /// owned output public key found
    pub pubkey: String,

    pub spent: bool,

    pub subaddr_index: SubaddressIndex,

    pub tx_hash: String,

    pub unlocked: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsMultisigResponse {
    pub kex_is_done: bool,

    pub multisig: bool,

    pub ready: bool,

    pub threshold: i64,

    pub total: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<SubaddressIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeIntegratedAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeIntegratedAddressResponse {
    pub integrated_address: String,

    pub payment_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeMultisigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisig_info: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeMultisigResponse {
    pub address: String,

    pub multisig_info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeUriRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MakeUriResponse {
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenWalletRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autosave_current: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseUriRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseUriResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_parameters: Option<Vec<String>>,

    pub uri: UriSpec,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UriSpec {
    pub address: String,

    pub amount: u64,

    pub payment_id: String,

    pub recipient_name: String,

    pub tx_description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepareMultisigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multisig_experimental: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepareMultisigResponse {
    pub multisig_info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryKeyResponse {
    pub key: String,

    pub polyseed_birthday: u64,

    pub polyseed_is_encrypted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub blocks_fetched: u64,

    pub received_money: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RescanBlockchainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_key_images: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreDeterministicWalletRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autosave_current: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_multisig_experimental: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_height: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_offset: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreDeterministicWalletResponse {
    pub address: String,

    pub info: String,

    pub seed: String,

    pub was_deprecated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanTxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetupBackgroundSyncRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_cache_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_sync_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAccountTagDescriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDaemonRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_allow_any_cert: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_allowed_fingerprints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca_file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_private_key_path: Option<String>,

    /// disabled, enabled, autodetect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_support: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSubaddrLookaheadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_idx: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_idx: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTxNotesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignResponse {
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignMultisigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_data_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignMultisigResponse {
    pub tx_data_hex: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_raw: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_keys: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsigned_txset: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignTransferResponse {
    pub signed_txset: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_key_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_raw_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitIntegratedAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrated_address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitIntegratedAddressResponse {
    pub is_subaddress: bool,

    pub payment_id: String,

    pub standard_address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopBackgroundSyncRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_offset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitMultisigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_data_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitMultisigResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_data_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitTransferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SweepAllRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub below_amount: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_relay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_hex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_keys: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_metadata: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices_all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SweepDustRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_relay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_hex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_keys: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SweepSingleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_relay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_hex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_key: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_metadata: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThawRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_image: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<TransferDestination>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_relay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_hex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_key: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_metadata: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtract_fee_from_outputs: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferSplitRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<TransferDestination>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_relay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_hex: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_keys: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_tx_metadata: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaddr_indices: Option<Vec<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntagAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidateAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_openalias: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_net_type: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidateAddressResponse {
    pub integrated: bool,

    pub nettype: String,

    pub openalias_address: String,

    pub subaddress: bool,

    pub valid: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub good: bool,

    pub old: bool,

    pub signature_type: String,

    pub version: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletExistsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletExistsResponse {
    pub keys_file_exists: bool,

    pub wallet_file_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleTransferResponse {
    pub amount: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts_by_dest: Option<AmountsList>,

    pub fee: u64,

    pub multisig_txset: String,

    pub spent_key_images: KeyImageList,

    pub tx_blob: String,

    pub tx_hash: String,

    pub tx_key: String,

    pub tx_metadata: String,

    pub unsigned_txset: String,

    pub weight: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmountsList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyImageList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_images: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplitTransferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_list: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts_by_dest_list: Option<Vec<AmountsList>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_list: Option<Vec<u64>>,

    pub multisig_txset: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_key_images_list: Option<Vec<KeyImageList>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_blob_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_key_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_metadata_list: Option<Vec<String>>,

    pub unsigned_txset: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_list: Option<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeightResponse {
    pub height: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionResponse {
    pub release: bool,

    pub version: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelayTxRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelayTxResponse {
    pub tx_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogCategoriesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogCategoriesResponse {
    pub categories: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLogLevelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartMiningRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_background_mining: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_battery: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_count: Option<u64>,
}
