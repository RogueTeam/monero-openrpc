package walletrpc

type AutoRefreshResponse map[string]interface{}

type ChangeWalletPasswordResponse map[string]interface{}

type CloseWalletResponse map[string]interface{}

type CreateWalletResponse map[string]interface{}

type DeleteAddressBookEntryResponse map[string]interface{}

type EditAddressBookEntryResponse map[string]interface{}

type ExportMultisigRequest map[string]interface{}

type FreezeResponse map[string]interface{}

type GetAccountTagsRequest map[string]interface{}

type GetDefaultFeePriorityRequest map[string]interface{}

type GetWalletInfoRequest map[string]interface{}

type IsMultisigRequest map[string]interface{}

type LabelAccountResponse map[string]interface{}

type LabelAddressResponse map[string]interface{}

type OpenWalletResponse map[string]interface{}

type RescanBlockchainResponse map[string]interface{}

type RescanSpentRequest map[string]interface{}

type RescanSpentResponse map[string]interface{}

type ScanTxResponse map[string]interface{}

type SetupBackgroundSyncResponse map[string]interface{}

type SetAccountTagDescriptionResponse map[string]interface{}

type SetAttributeResponse map[string]interface{}

type SetDaemonResponse map[string]interface{}

type SetSubaddrLookaheadResponse map[string]interface{}

type SetTxNotesResponse map[string]interface{}

type StartBackgroundSyncRequest map[string]interface{}

type StartBackgroundSyncResponse map[string]interface{}

type StopBackgroundSyncResponse map[string]interface{}

type StopWalletRequest map[string]interface{}

type StopWalletResponse map[string]interface{}

type StoreRequest map[string]interface{}

type StoreResponse map[string]interface{}

type TagAccountsResponse map[string]interface{}

type ThawResponse map[string]interface{}

type UntagAccountsResponse map[string]interface{}

type GetHeightRequest map[string]interface{}

type GetVersionRequest map[string]interface{}

type SetLogLevelResponse map[string]interface{}

type StartMiningResponse map[string]interface{}

type StopMiningRequest map[string]interface{}

type StopMiningResponse map[string]interface{}

type AddAddressBookEntryRequest struct {
	Address     *string `json:"address,omitempty"`
	Description *string `json:"description,omitempty"`
}

type AddAddressBookEntryResponse struct {
	Index int64 `json:"index"`
}

type AutoRefreshRequest struct {
	Enable *bool `json:"enable,omitempty"`
	// seconds
	Period *int64 `json:"period,omitempty"`
}

type ChangeWalletPasswordRequest struct {
	NewPassword *string `json:"new_password,omitempty"`
	OldPassword *string `json:"old_password,omitempty"`
}

type CheckReserveProofRequest struct {
	Address   *string `json:"address,omitempty"`
	Message   *string `json:"message,omitempty"`
	Signature *string `json:"signature,omitempty"`
}

type CheckReserveProofResponse struct {
	Good  bool  `json:"good"`
	Spent int64 `json:"spent"`
	Total int64 `json:"total"`
}

type CheckSpendProofRequest struct {
	Message   *string `json:"message,omitempty"`
	Signature *string `json:"signature,omitempty"`
	Txid      *string `json:"txid,omitempty"`
}

type CheckSpendProofResponse struct {
	Good bool `json:"good"`
}

type CheckTxKeyRequest struct {
	Address *string `json:"address,omitempty"`
	TxKey   *string `json:"tx_key,omitempty"`
	Txid    *string `json:"txid,omitempty"`
}

type CheckTxKeyResponse struct {
	Confirmations int64 `json:"confirmations"`
	InPool        bool  `json:"in_pool"`
	Received      int64 `json:"received"`
}

type CheckTxProofRequest struct {
	Address   *string `json:"address,omitempty"`
	Message   *string `json:"message,omitempty"`
	Signature *string `json:"signature,omitempty"`
	Txid      *string `json:"txid,omitempty"`
}

type CheckTxProofResponse struct {
	Confirmations int64 `json:"confirmations"`
	Good          bool  `json:"good"`
	InPool        bool  `json:"in_pool"`
	Received      int64 `json:"received"`
}

type CloseWalletRequest struct {
	AutosaveCurrent *bool `json:"autosave_current,omitempty"`
}

type CreateAccountRequest struct {
	Label *string `json:"label,omitempty"`
}

type CreateAccountResponse struct {
	AccountIndex int64 `json:"account_index"`
	// the 0-th address for convenience
	Address string `json:"address"`
}

type CreateAddressRequest struct {
	AccountIndex *int64  `json:"account_index,omitempty"`
	Count        *int64  `json:"count,omitempty"`
	Label        *string `json:"label,omitempty"`
}

type CreateAddressResponse struct {
	Address        string   `json:"address"`
	AddressIndex   int64    `json:"address_index"`
	AddressIndices []int64  `json:"address_indices,omitempty"`
	Addresses      []string `json:"addresses,omitempty"`
}

type CreateWalletRequest struct {
	Filename *string `json:"filename,omitempty"`
	Language *string `json:"language,omitempty"`
	Password *string `json:"password,omitempty"`
	Polyseed *bool   `json:"polyseed,omitempty"`
}

type DeleteAddressBookEntryRequest struct {
	Index *int64 `json:"index,omitempty"`
}

type DescribeTransferRequest struct {
	MultisigTxset *string `json:"multisig_txset,omitempty"`
	UnsignedTxset *string `json:"unsigned_txset,omitempty"`
}

type DescribeTransferResponse struct {
	Desc    []DescribeTransferTransferDescription `json:"desc,omitempty"`
	Summary DescribeTransferTxsetSummary          `json:"summary"`
}

type DescribeTransferTransferDescription struct {
	AmountIn      int64                       `json:"amount_in"`
	AmountOut     int64                       `json:"amount_out"`
	ChangeAddress string                      `json:"change_address"`
	ChangeAmount  int64                       `json:"change_amount"`
	DummyOutputs  int64                       `json:"dummy_outputs"`
	Extra         string                      `json:"extra"`
	Fee           int64                       `json:"fee"`
	PaymentID     string                      `json:"payment_id"`
	Recipients    []DescribeTransferRecipient `json:"recipients,omitempty"`
	RingSize      int64                       `json:"ring_size"`
	Sources       []DescribeTransferSource    `json:"sources,omitempty"`
	UnlockTime    int64                       `json:"unlock_time"`
	Weight        *int64                      `json:"weight,omitempty"`
}

type DescribeTransferRecipient struct {
	Address string `json:"address"`
	Amount  int64  `json:"amount"`
}

type DescribeTransferSource struct {
	Amount      int64  `json:"amount"`
	GlobalIndex int64  `json:"global_index"`
	Pubkey      string `json:"pubkey"`
	Rct         bool   `json:"rct"`
}

type DescribeTransferTxsetSummary struct {
	AmountIn      int64                       `json:"amount_in"`
	AmountOut     int64                       `json:"amount_out"`
	ChangeAddress string                      `json:"change_address"`
	ChangeAmount  int64                       `json:"change_amount"`
	Fee           int64                       `json:"fee"`
	Recipients    []DescribeTransferRecipient `json:"recipients,omitempty"`
}

type EditAddressBookEntryRequest struct {
	Address        *string `json:"address,omitempty"`
	Description    *string `json:"description,omitempty"`
	Index          *int64  `json:"index,omitempty"`
	SetAddress     *bool   `json:"set_address,omitempty"`
	SetDescription *bool   `json:"set_description,omitempty"`
}

type EstimateTxSizeAndWeightRequest struct {
	NInputs  *int64 `json:"n_inputs,omitempty"`
	NOutputs *int64 `json:"n_outputs,omitempty"`
	Rct      *bool  `json:"rct,omitempty"`
	RingSize *int64 `json:"ring_size,omitempty"`
}

type EstimateTxSizeAndWeightResponse struct {
	Size   int64 `json:"size"`
	Weight int64 `json:"weight"`
}

type ExchangeMultisigKeysRequest struct {
	ForceUpdateUseWithCaution *bool    `json:"force_update_use_with_caution,omitempty"`
	MultisigInfo              []string `json:"multisig_info,omitempty"`
	Password                  *string  `json:"password,omitempty"`
}

type ExchangeMultisigKeysResponse struct {
	Address      string `json:"address"`
	MultisigInfo string `json:"multisig_info"`
}

type ExportKeyImagesRequest struct {
	All *bool `json:"all,omitempty"`
}

type ExportKeyImagesResponse struct {
	Offset          int64                           `json:"offset"`
	SignedKeyImages []ExportKeyImagesSignedKeyImage `json:"signed_key_images,omitempty"`
}

type ExportKeyImagesSignedKeyImage struct {
	KeyImage  string `json:"key_image"`
	Signature string `json:"signature"`
}

type ExportMultisigResponse struct {
	Info string `json:"info"`
}

type ExportOutputsRequest struct {
	All   *bool  `json:"all,omitempty"`
	Count *int64 `json:"count,omitempty"`
	Start *int64 `json:"start,omitempty"`
}

type ExportOutputsResponse struct {
	OutputsDataHex string `json:"outputs_data_hex"`
}

type FreezeRequest struct {
	KeyImage *string `json:"key_image,omitempty"`
}

type FrozenRequest struct {
	KeyImage *string `json:"key_image,omitempty"`
}

type FrozenResponse struct {
	Frozen bool `json:"frozen"`
}

type GenerateFromKeysRequest struct {
	Address         *string `json:"address,omitempty"`
	AutosaveCurrent *bool   `json:"autosave_current,omitempty"`
	Filename        *string `json:"filename,omitempty"`
	Language        *string `json:"language,omitempty"`
	Password        *string `json:"password,omitempty"`
	RestoreHeight   *int64  `json:"restore_height,omitempty"`
	Spendkey        *string `json:"spendkey,omitempty"`
	Viewkey         *string `json:"viewkey,omitempty"`
}

type GenerateFromKeysResponse struct {
	Address string `json:"address"`
	Info    string `json:"info"`
}

type GetAccountsRequest struct {
	// allow regular expression filters if set to true
	Regexp         *bool `json:"regexp,omitempty"`
	StrictBalances *bool `json:"strict_balances,omitempty"`
	// all accounts if empty, otherwise those accounts with this tag
	Tag *string `json:"tag,omitempty"`
}

type GetAccountsResponse struct {
	SubaddressAccounts   []GetAccountsSubaddressAccountInfo `json:"subaddress_accounts,omitempty"`
	TotalBalance         int64                              `json:"total_balance"`
	TotalUnlockedBalance int64                              `json:"total_unlocked_balance"`
}

type GetAccountsSubaddressAccountInfo struct {
	AccountIndex    int64  `json:"account_index"`
	Balance         int64  `json:"balance"`
	BaseAddress     string `json:"base_address"`
	Label           string `json:"label"`
	Tag             string `json:"tag"`
	UnlockedBalance int64  `json:"unlocked_balance"`
}

type GetAccountTagsResponse struct {
	AccountTags []GetAccountTagsAccountTagInfo `json:"account_tags,omitempty"`
}

type GetAccountTagsAccountTagInfo struct {
	Accounts []int64 `json:"accounts,omitempty"`
	Label    string  `json:"label"`
	Tag      string  `json:"tag"`
}

type GetAddressRequest struct {
	AccountIndex *int64  `json:"account_index,omitempty"`
	AddressIndex []int64 `json:"address_index,omitempty"`
	AllAccounts  *bool   `json:"all_accounts,omitempty"`
}

type GetAddressResponse struct {
	// to remain compatible with older RPC format
	Address   string                  `json:"address"`
	Addresses []GetAddressAddressInfo `json:"addresses,omitempty"`
}

type GetAddressAddressInfo struct {
	AccountIndex int64  `json:"account_index"`
	Address      string `json:"address"`
	AddressIndex int64  `json:"address_index"`
	Label        string `json:"label"`
	Used         bool   `json:"used"`
}

type GetAddressBookEntryRequest struct {
	Entries []int64 `json:"entries,omitempty"`
}

type GetAddressBookEntryResponse struct {
	Entries []GetAddressBookEntryEntry `json:"entries,omitempty"`
}

type GetAddressBookEntryEntry struct {
	Address     string `json:"address"`
	Description string `json:"description"`
	Index       int64  `json:"index"`
}

type GetAddressIndexRequest struct {
	Address *string `json:"address,omitempty"`
}

type GetAddressIndexResponse struct {
	Index SubaddressIndex `json:"index"`
}

type SubaddressIndex struct {
	Major int64 `json:"major"`
	Minor int64 `json:"minor"`
}

type GetAttributeRequest struct {
	Key *string `json:"key,omitempty"`
}

type GetAttributeResponse struct {
	Value string `json:"value"`
}

type GetBalanceRequest struct {
	AccountIndex   *int64  `json:"account_index,omitempty"`
	AddressIndices []int64 `json:"address_indices,omitempty"`
	AllAccounts    *bool   `json:"all_accounts,omitempty"`
	Strict         *bool   `json:"strict,omitempty"`
}

type GetBalanceResponse struct {
	Balance              int64                         `json:"balance"`
	BlocksToUnlock       int64                         `json:"blocks_to_unlock"`
	MultisigImportNeeded bool                          `json:"multisig_import_needed"`
	PerSubaddress        []GetBalancePerSubaddressInfo `json:"per_subaddress,omitempty"`
	TimeToUnlock         int64                         `json:"time_to_unlock"`
	UnlockedBalance      int64                         `json:"unlocked_balance"`
}

type GetBalancePerSubaddressInfo struct {
	AccountIndex      int64  `json:"account_index"`
	Address           string `json:"address"`
	AddressIndex      int64  `json:"address_index"`
	Balance           int64  `json:"balance"`
	BlocksToUnlock    int64  `json:"blocks_to_unlock"`
	Label             string `json:"label"`
	NumUnspentOutputs int64  `json:"num_unspent_outputs"`
	TimeToUnlock      int64  `json:"time_to_unlock"`
	UnlockedBalance   int64  `json:"unlocked_balance"`
}

type GetBulkPaymentsRequest struct {
	MinBlockHeight *int64   `json:"min_block_height,omitempty"`
	PaymentIDS     []string `json:"payment_ids,omitempty"`
}

type GetBulkPaymentsResponse struct {
	Payments []PaymentDetails `json:"payments,omitempty"`
}

type PaymentDetails struct {
	Address      string          `json:"address"`
	Amount       int64           `json:"amount"`
	BlockHeight  int64           `json:"block_height"`
	Locked       bool            `json:"locked"`
	PaymentID    string          `json:"payment_id"`
	SubaddrIndex SubaddressIndex `json:"subaddr_index"`
	TxHash       string          `json:"tx_hash"`
	UnlockTime   int64           `json:"unlock_time"`
}

type GetDefaultFeePriorityResponse struct {
	Priority int64 `json:"priority"`
}

type GetLanguagesRequest struct {
	Polyseed *bool `json:"polyseed,omitempty"`
}

type GetLanguagesResponse struct {
	Languages      []string `json:"languages,omitempty"`
	LanguagesLocal []string `json:"languages_local,omitempty"`
}

type GetMultisigKeyExchangeBoosterRequest struct {
	MultisigInfo []string `json:"multisig_info,omitempty"`
	NumSigners   *int64   `json:"num_signers,omitempty"`
	Password     *string  `json:"password,omitempty"`
	Threshold    *int64   `json:"threshold,omitempty"`
}

type GetMultisigKeyExchangeBoosterResponse struct {
	MultisigInfo string `json:"multisig_info"`
}

type GetPaymentsRequest struct {
	PaymentID *string `json:"payment_id,omitempty"`
}

type GetPaymentsResponse struct {
	Payments []PaymentDetails `json:"payments,omitempty"`
}

type GetReserveProofRequest struct {
	// ignored when `all` is true
	AccountIndex *int64 `json:"account_index,omitempty"`
	All          *bool  `json:"all,omitempty"`
	// ignored when `all` is true
	Amount  *int64  `json:"amount,omitempty"`
	Message *string `json:"message,omitempty"`
}

type GetReserveProofResponse struct {
	Signature string `json:"signature"`
}

type GetSpendProofRequest struct {
	Message *string `json:"message,omitempty"`
	Txid    *string `json:"txid,omitempty"`
}

type GetSpendProofResponse struct {
	Signature string `json:"signature"`
}

type GetTransfersRequest struct {
	AccountIndex   *int64  `json:"account_index,omitempty"`
	AllAccounts    *bool   `json:"all_accounts,omitempty"`
	Failed         *bool   `json:"failed,omitempty"`
	FilterByHeight *bool   `json:"filter_by_height,omitempty"`
	In             *bool   `json:"in,omitempty"`
	MaxHeight      *int64  `json:"max_height,omitempty"`
	MinHeight      *int64  `json:"min_height,omitempty"`
	Out            *bool   `json:"out,omitempty"`
	Pending        *bool   `json:"pending,omitempty"`
	Pool           *bool   `json:"pool,omitempty"`
	RefreshPool    *bool   `json:"refresh_pool,omitempty"`
	SubaddrIndices []int64 `json:"subaddr_indices,omitempty"`
}

type GetTransfersResponse struct {
	Failed  []TransferEntry `json:"failed,omitempty"`
	In      []TransferEntry `json:"in,omitempty"`
	Out     []TransferEntry `json:"out,omitempty"`
	Pending []TransferEntry `json:"pending,omitempty"`
	Pool    []TransferEntry `json:"pool,omitempty"`
}

type TransferEntry struct {
	Address                         string                `json:"address"`
	Amount                          int64                 `json:"amount"`
	Amounts                         []int64               `json:"amounts,omitempty"`
	ChangeAmount                    *int64                `json:"change_amount,omitempty"`
	Confirmations                   *int64                `json:"confirmations,omitempty"`
	Destinations                    []TransferDestination `json:"destinations,omitempty"`
	DoubleSpendSeen                 bool                  `json:"double_spend_seen"`
	Fee                             int64                 `json:"fee"`
	Height                          int64                 `json:"height"`
	Locked                          bool                  `json:"locked"`
	Note                            string                `json:"note"`
	PaymentID                       string                `json:"payment_id"`
	SubaddrIndex                    SubaddressIndex       `json:"subaddr_index"`
	SubaddrIndices                  []SubaddressIndex     `json:"subaddr_indices,omitempty"`
	SuggestedConfirmationsThreshold *int64                `json:"suggested_confirmations_threshold,omitempty"`
	Timestamp                       int64                 `json:"timestamp"`
	Txid                            string                `json:"txid"`
	Type                            string                `json:"type"`
	UnlockTime                      int64                 `json:"unlock_time"`
}

type TransferDestination struct {
	Address string `json:"address"`
	Amount  int64  `json:"amount"`
}

type GetTransferByTxidRequest struct {
	AccountIndex *int64  `json:"account_index,omitempty"`
	AllAccounts  *bool   `json:"all_accounts,omitempty"`
	Txid         *string `json:"txid,omitempty"`
}

type GetTransferByTxidResponse struct {
	Transfer  TransferEntry   `json:"transfer"`
	Transfers []TransferEntry `json:"transfers,omitempty"`
}

type GetTxKeyRequest struct {
	Txid *string `json:"txid,omitempty"`
}

type GetTxKeyResponse struct {
	TxKey string `json:"tx_key"`
}

type GetTxNotesRequest struct {
	Txids []string `json:"txids,omitempty"`
}

type GetTxNotesResponse struct {
	Notes []string `json:"notes,omitempty"`
}

type GetTxProofRequest struct {
	Address *string `json:"address,omitempty"`
	Message *string `json:"message,omitempty"`
	Txid    *string `json:"txid,omitempty"`
}

type GetTxProofResponse struct {
	Signature string `json:"signature"`
}

type GetWalletInfoResponse struct {
	Address           string `json:"address"`
	DaemonAddress     string `json:"daemon_address"`
	DaemonBlockHeight int64  `json:"daemon_block_height"`
	DaemonProxy       string `json:"daemon_proxy"`
	DaemonRPCVersion  int64  `json:"daemon_rpc_version"`
	DaemonSSL         bool   `json:"daemon_ssl"`
	Description       string `json:"description"`
	Filename          string `json:"filename"`
	NetworkType       string `json:"network_type"`
	// Legacy | Multisig | Polyseed
	SeedType          string `json:"seed_type"`
	WalletBlockHeight int64  `json:"wallet_block_height"`
	// Normal | Multisig | Watch-Only | Background
	WalletType string `json:"wallet_type"`
}

type ImportKeyImagesRequest struct {
	Offset          *int64                          `json:"offset,omitempty"`
	SignedKeyImages []ImportKeyImagesSignedKeyImage `json:"signed_key_images,omitempty"`
}

type ImportKeyImagesSignedKeyImage struct {
	KeyImage  *string `json:"key_image,omitempty"`
	Signature *string `json:"signature,omitempty"`
}

type ImportKeyImagesResponse struct {
	Height  int64 `json:"height"`
	Spent   int64 `json:"spent"`
	Unspent int64 `json:"unspent"`
}

type ImportMultisigRequest struct {
	Info               []string `json:"info,omitempty"`
	RefreshAfterImport *bool    `json:"refresh_after_import,omitempty"`
}

type ImportMultisigResponse struct {
	NOutputs int64 `json:"n_outputs"`
}

type ImportOutputsRequest struct {
	OutputsDataHex *string `json:"outputs_data_hex,omitempty"`
}

type ImportOutputsResponse struct {
	NumImported int64 `json:"num_imported"`
}

type IncomingTransfersRequest struct {
	AccountIndex   *int64  `json:"account_index,omitempty"`
	AllAccounts    *bool   `json:"all_accounts,omitempty"`
	SubaddrIndices []int64 `json:"subaddr_indices,omitempty"`
	TransferType   *string `json:"transfer_type,omitempty"`
}

type IncomingTransfersResponse struct {
	Transfers []TransferDetails `json:"transfers,omitempty"`
}

type TransferDetails struct {
	Amount      int64  `json:"amount"`
	BlockHeight int64  `json:"block_height"`
	Frozen      bool   `json:"frozen"`
	GlobalIndex int64  `json:"global_index"`
	KeyImage    string `json:"key_image"`
	// owned output public key found
	Pubkey       string          `json:"pubkey"`
	Spent        bool            `json:"spent"`
	SubaddrIndex SubaddressIndex `json:"subaddr_index"`
	TxHash       string          `json:"tx_hash"`
	Unlocked     bool            `json:"unlocked"`
}

type IsMultisigResponse struct {
	KexIsDone bool  `json:"kex_is_done"`
	Multisig  bool  `json:"multisig"`
	Ready     bool  `json:"ready"`
	Threshold int64 `json:"threshold"`
	Total     int64 `json:"total"`
}

type LabelAccountRequest struct {
	AccountIndex *int64  `json:"account_index,omitempty"`
	Label        *string `json:"label,omitempty"`
}

type LabelAddressRequest struct {
	Index *SubaddressIndex `json:"index,omitempty"`
	Label *string          `json:"label,omitempty"`
}

type MakeIntegratedAddressRequest struct {
	PaymentID       *string `json:"payment_id,omitempty"`
	StandardAddress *string `json:"standard_address,omitempty"`
}

type MakeIntegratedAddressResponse struct {
	IntegratedAddress string `json:"integrated_address"`
	PaymentID         string `json:"payment_id"`
}

type MakeMultisigRequest struct {
	MultisigInfo []string `json:"multisig_info,omitempty"`
	Password     *string  `json:"password,omitempty"`
	Threshold    *int64   `json:"threshold,omitempty"`
}

type MakeMultisigResponse struct {
	Address      string `json:"address"`
	MultisigInfo string `json:"multisig_info"`
}

type MakeURIRequest struct {
	Address       *string `json:"address,omitempty"`
	Amount        *int64  `json:"amount,omitempty"`
	PaymentID     *string `json:"payment_id,omitempty"`
	RecipientName *string `json:"recipient_name,omitempty"`
	TxDescription *string `json:"tx_description,omitempty"`
}

type MakeURIResponse struct {
	URI string `json:"uri"`
}

type OpenWalletRequest struct {
	AutosaveCurrent *bool   `json:"autosave_current,omitempty"`
	Filename        *string `json:"filename,omitempty"`
	Password        *string `json:"password,omitempty"`
}

type ParseURIRequest struct {
	URI *string `json:"uri,omitempty"`
}

type ParseURIResponse struct {
	UnknownParameters []string `json:"unknown_parameters,omitempty"`
	URI               URISpec  `json:"uri"`
}

type URISpec struct {
	Address       string `json:"address"`
	Amount        int64  `json:"amount"`
	PaymentID     string `json:"payment_id"`
	RecipientName string `json:"recipient_name"`
	TxDescription string `json:"tx_description"`
}

type PrepareMultisigRequest struct {
	EnableMultisigExperimental *bool `json:"enable_multisig_experimental,omitempty"`
}

type PrepareMultisigResponse struct {
	MultisigInfo string `json:"multisig_info"`
}

type QueryKeyRequest struct {
	KeyType *string `json:"key_type,omitempty"`
}

type QueryKeyResponse struct {
	Key                 string `json:"key"`
	PolyseedBirthday    int64  `json:"polyseed_birthday"`
	PolyseedIsEncrypted bool   `json:"polyseed_is_encrypted"`
}

type RefreshRequest struct {
	StartHeight *int64 `json:"start_height,omitempty"`
}

type RefreshResponse struct {
	BlocksFetched int64 `json:"blocks_fetched"`
	ReceivedMoney bool  `json:"received_money"`
}

type RescanBlockchainRequest struct {
	Hard          *bool `json:"hard,omitempty"`
	KeepKeyImages *bool `json:"keep_key_images,omitempty"`
}

type RestoreDeterministicWalletRequest struct {
	AutosaveCurrent            *bool   `json:"autosave_current,omitempty"`
	EnableMultisigExperimental *bool   `json:"enable_multisig_experimental,omitempty"`
	Filename                   *string `json:"filename,omitempty"`
	Language                   *string `json:"language,omitempty"`
	Password                   *string `json:"password,omitempty"`
	RestoreHeight              *int64  `json:"restore_height,omitempty"`
	Seed                       *string `json:"seed,omitempty"`
	SeedOffset                 *string `json:"seed_offset,omitempty"`
}

type RestoreDeterministicWalletResponse struct {
	Address       string `json:"address"`
	Info          string `json:"info"`
	Seed          string `json:"seed"`
	WasDeprecated bool   `json:"was_deprecated"`
}

type ScanTxRequest struct {
	Txids []string `json:"txids,omitempty"`
}

type SetupBackgroundSyncRequest struct {
	BackgroundCachePassword *string `json:"background_cache_password,omitempty"`
	BackgroundSyncType      *string `json:"background_sync_type,omitempty"`
	WalletPassword          *string `json:"wallet_password,omitempty"`
}

type SetAccountTagDescriptionRequest struct {
	Description *string `json:"description,omitempty"`
	Tag         *string `json:"tag,omitempty"`
}

type SetAttributeRequest struct {
	Key   *string `json:"key,omitempty"`
	Value *string `json:"value,omitempty"`
}

type SetDaemonRequest struct {
	Address                *string  `json:"address,omitempty"`
	Password               *string  `json:"password,omitempty"`
	Proxy                  *string  `json:"proxy,omitempty"`
	SSLAllowAnyCERT        *bool    `json:"ssl_allow_any_cert,omitempty"`
	SSLAllowedFingerprints []string `json:"ssl_allowed_fingerprints,omitempty"`
	SSLCAFile              *string  `json:"ssl_ca_file,omitempty"`
	SSLCertificatePath     *string  `json:"ssl_certificate_path,omitempty"`
	SSLPrivateKeyPath      *string  `json:"ssl_private_key_path,omitempty"`
	// disabled, enabled, autodetect
	SSLSupport *string `json:"ssl_support,omitempty"`
	Trusted    *bool   `json:"trusted,omitempty"`
	Username   *string `json:"username,omitempty"`
}

type SetSubaddrLookaheadRequest struct {
	MajorIdx *int64  `json:"major_idx,omitempty"`
	MinorIdx *int64  `json:"minor_idx,omitempty"`
	Password *string `json:"password,omitempty"`
}

type SetTxNotesRequest struct {
	Notes []string `json:"notes,omitempty"`
	Txids []string `json:"txids,omitempty"`
}

type SignRequest struct {
	AccountIndex  *int64  `json:"account_index,omitempty"`
	AddressIndex  *int64  `json:"address_index,omitempty"`
	Data          *string `json:"data,omitempty"`
	SignatureType *string `json:"signature_type,omitempty"`
}

type SignResponse struct {
	Signature string `json:"signature"`
}

type SignMultisigRequest struct {
	TxDataHex *string `json:"tx_data_hex,omitempty"`
}

type SignMultisigResponse struct {
	TxDataHex  string   `json:"tx_data_hex"`
	TxHashList []string `json:"tx_hash_list,omitempty"`
}

type SignTransferRequest struct {
	ExportRaw     *bool   `json:"export_raw,omitempty"`
	GetTxKeys     *bool   `json:"get_tx_keys,omitempty"`
	UnsignedTxset *string `json:"unsigned_txset,omitempty"`
}

type SignTransferResponse struct {
	SignedTxset string   `json:"signed_txset"`
	TxHashList  []string `json:"tx_hash_list,omitempty"`
	TxKeyList   []string `json:"tx_key_list,omitempty"`
	TxRawList   []string `json:"tx_raw_list,omitempty"`
}

type SplitIntegratedAddressRequest struct {
	IntegratedAddress *string `json:"integrated_address,omitempty"`
}

type SplitIntegratedAddressResponse struct {
	IsSubaddress    bool   `json:"is_subaddress"`
	PaymentID       string `json:"payment_id"`
	StandardAddress string `json:"standard_address"`
}

type StopBackgroundSyncRequest struct {
	Seed           *string `json:"seed,omitempty"`
	SeedOffset     *string `json:"seed_offset,omitempty"`
	WalletPassword *string `json:"wallet_password,omitempty"`
}

type SubmitMultisigRequest struct {
	TxDataHex *string `json:"tx_data_hex,omitempty"`
}

type SubmitMultisigResponse struct {
	TxHashList []string `json:"tx_hash_list,omitempty"`
}

type SubmitTransferRequest struct {
	TxDataHex *string `json:"tx_data_hex,omitempty"`
}

type SubmitTransferResponse struct {
	TxHashList []string `json:"tx_hash_list,omitempty"`
}

type SweepAllRequest struct {
	AccountIndex      *int64  `json:"account_index,omitempty"`
	Address           *string `json:"address,omitempty"`
	BelowAmount       *int64  `json:"below_amount,omitempty"`
	DoNotRelay        *bool   `json:"do_not_relay,omitempty"`
	GetTxHex          *bool   `json:"get_tx_hex,omitempty"`
	GetTxKeys         *bool   `json:"get_tx_keys,omitempty"`
	GetTxMetadata     *bool   `json:"get_tx_metadata,omitempty"`
	Outputs           *int64  `json:"outputs,omitempty"`
	PaymentID         *string `json:"payment_id,omitempty"`
	Priority          *int64  `json:"priority,omitempty"`
	RingSize          *int64  `json:"ring_size,omitempty"`
	SubaddrIndices    []int64 `json:"subaddr_indices,omitempty"`
	SubaddrIndicesAll *bool   `json:"subaddr_indices_all,omitempty"`
	UnlockTime        *int64  `json:"unlock_time,omitempty"`
}

type SweepDustRequest struct {
	DoNotRelay    *bool `json:"do_not_relay,omitempty"`
	GetTxHex      *bool `json:"get_tx_hex,omitempty"`
	GetTxKeys     *bool `json:"get_tx_keys,omitempty"`
	GetTxMetadata *bool `json:"get_tx_metadata,omitempty"`
}

type SweepSingleRequest struct {
	Address       *string `json:"address,omitempty"`
	DoNotRelay    *bool   `json:"do_not_relay,omitempty"`
	GetTxHex      *bool   `json:"get_tx_hex,omitempty"`
	GetTxKey      *bool   `json:"get_tx_key,omitempty"`
	GetTxMetadata *bool   `json:"get_tx_metadata,omitempty"`
	KeyImage      *string `json:"key_image,omitempty"`
	Outputs       *int64  `json:"outputs,omitempty"`
	PaymentID     *string `json:"payment_id,omitempty"`
	Priority      *int64  `json:"priority,omitempty"`
	RingSize      *int64  `json:"ring_size,omitempty"`
	UnlockTime    *int64  `json:"unlock_time,omitempty"`
}

type TagAccountsRequest struct {
	Accounts []int64 `json:"accounts,omitempty"`
	Tag      *string `json:"tag,omitempty"`
}

type ThawRequest struct {
	KeyImage *string `json:"key_image,omitempty"`
}

type TransferRequest struct {
	AccountIndex           *int64                `json:"account_index,omitempty"`
	Destinations           []TransferDestination `json:"destinations,omitempty"`
	DoNotRelay             *bool                 `json:"do_not_relay,omitempty"`
	GetTxHex               *bool                 `json:"get_tx_hex,omitempty"`
	GetTxKey               *bool                 `json:"get_tx_key,omitempty"`
	GetTxMetadata          *bool                 `json:"get_tx_metadata,omitempty"`
	PaymentID              *string               `json:"payment_id,omitempty"`
	Priority               *int64                `json:"priority,omitempty"`
	RingSize               *int64                `json:"ring_size,omitempty"`
	SubaddrIndices         []int64               `json:"subaddr_indices,omitempty"`
	SubtractFeeFromOutputs []int64               `json:"subtract_fee_from_outputs,omitempty"`
	UnlockTime             *int64                `json:"unlock_time,omitempty"`
}

type TransferSplitRequest struct {
	AccountIndex   *int64                `json:"account_index,omitempty"`
	Destinations   []TransferDestination `json:"destinations,omitempty"`
	DoNotRelay     *bool                 `json:"do_not_relay,omitempty"`
	GetTxHex       *bool                 `json:"get_tx_hex,omitempty"`
	GetTxKeys      *bool                 `json:"get_tx_keys,omitempty"`
	GetTxMetadata  *bool                 `json:"get_tx_metadata,omitempty"`
	PaymentID      *string               `json:"payment_id,omitempty"`
	Priority       *int64                `json:"priority,omitempty"`
	RingSize       *int64                `json:"ring_size,omitempty"`
	SubaddrIndices []int64               `json:"subaddr_indices,omitempty"`
	UnlockTime     *int64                `json:"unlock_time,omitempty"`
}

type UntagAccountsRequest struct {
	Accounts []int64 `json:"accounts,omitempty"`
}

type ValidateAddressRequest struct {
	Address        *string `json:"address,omitempty"`
	AllowOpenalias *bool   `json:"allow_openalias,omitempty"`
	AnyNetType     *bool   `json:"any_net_type,omitempty"`
}

type ValidateAddressResponse struct {
	Integrated       bool   `json:"integrated"`
	Nettype          string `json:"nettype"`
	OpenaliasAddress string `json:"openalias_address"`
	Subaddress       bool   `json:"subaddress"`
	Valid            bool   `json:"valid"`
}

type VerifyRequest struct {
	Address   *string `json:"address,omitempty"`
	Data      *string `json:"data,omitempty"`
	Signature *string `json:"signature,omitempty"`
}

type VerifyResponse struct {
	Good          bool   `json:"good"`
	Old           bool   `json:"old"`
	SignatureType string `json:"signature_type"`
	Version       int64  `json:"version"`
}

type WalletExistsRequest struct {
	Filename *string `json:"filename,omitempty"`
}

type WalletExistsResponse struct {
	KeysFileExists   bool `json:"keys_file_exists"`
	WalletFileExists bool `json:"wallet_file_exists"`
}

type SingleTransferResponse struct {
	Amount         int64        `json:"amount"`
	AmountsByDest  *AmountsList `json:"amounts_by_dest,omitempty"`
	Fee            int64        `json:"fee"`
	MultisigTxset  string       `json:"multisig_txset"`
	SpentKeyImages KeyImageList `json:"spent_key_images"`
	TxBlob         string       `json:"tx_blob"`
	TxHash         string       `json:"tx_hash"`
	TxKey          string       `json:"tx_key"`
	TxMetadata     string       `json:"tx_metadata"`
	UnsignedTxset  string       `json:"unsigned_txset"`
	Weight         int64        `json:"weight"`
}

type AmountsList struct {
	Amounts []int64 `json:"amounts,omitempty"`
}

type KeyImageList struct {
	KeyImages []string `json:"key_images,omitempty"`
}

type SplitTransferResponse struct {
	AmountList         []int64        `json:"amount_list,omitempty"`
	AmountsByDestList  []AmountsList  `json:"amounts_by_dest_list,omitempty"`
	FeeList            []int64        `json:"fee_list,omitempty"`
	MultisigTxset      string         `json:"multisig_txset"`
	SpentKeyImagesList []KeyImageList `json:"spent_key_images_list,omitempty"`
	TxBlobList         []string       `json:"tx_blob_list,omitempty"`
	TxHashList         []string       `json:"tx_hash_list,omitempty"`
	TxKeyList          []string       `json:"tx_key_list,omitempty"`
	TxMetadataList     []string       `json:"tx_metadata_list,omitempty"`
	UnsignedTxset      string         `json:"unsigned_txset"`
	WeightList         []int64        `json:"weight_list,omitempty"`
}

type GetHeightResponse struct {
	Height int64 `json:"height"`
}

type GetVersionResponse struct {
	Release bool  `json:"release"`
	Version int64 `json:"version"`
}

type RelayTxRequest struct {
	Hex *string `json:"hex,omitempty"`
}

type RelayTxResponse struct {
	TxHash string `json:"tx_hash"`
}

type SetLogCategoriesRequest struct {
	Categories *string `json:"categories,omitempty"`
}

type SetLogCategoriesResponse struct {
	Categories string `json:"categories"`
}

type SetLogLevelRequest struct {
	Level *int64 `json:"level,omitempty"`
}

type StartMiningRequest struct {
	DoBackgroundMining *bool  `json:"do_background_mining,omitempty"`
	IgnoreBattery      *bool  `json:"ignore_battery,omitempty"`
	ThreadsCount       *int64 `json:"threads_count,omitempty"`
}
