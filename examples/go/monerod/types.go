package monerod

type GetbansRequest map[string]interface{}

type GetminerdataRequest map[string]interface{}

type GetAlternateChainsRequest map[string]interface{}

type GetConnectionsRequest map[string]interface{}

type GetHeightRequest map[string]interface{}

type GetLimitRequest map[string]interface{}

type GetNetStatsRequest map[string]interface{}

type GetVersionRequest map[string]interface{}

type MiningStatusRequest map[string]interface{}

type SaveBcRequest map[string]interface{}

type StopDaemonRequest map[string]interface{}

type StopMiningRequest map[string]interface{}

type AddAuxPowRequest struct {
	AuxPow            []AddAuxPowAuxPow `json:"aux_pow,omitempty"`
	BlocktemplateBlob *string           `json:"blocktemplate_blob,omitempty"`
}

type AddAuxPowAuxPow struct {
	Hash string `json:"hash"`
	ID   string `json:"id"`
}

type AddAuxPowResponse struct {
	AuxPow            []AddAuxPowAuxPow `json:"aux_pow,omitempty"`
	BlockhashingBlob  string            `json:"blockhashing_blob"`
	BlocktemplateBlob string            `json:"blocktemplate_blob"`
	MerkleRoot        string            `json:"merkle_root"`
	MerkleTreeDepth   int64             `json:"merkle_tree_depth"`
	Status            string            `json:"status"`
}

type BannedRequest struct {
	Address *string `json:"address,omitempty"`
}

type BannedResponse struct {
	Banned  bool   `json:"banned"`
	Seconds int64  `json:"seconds"`
	Status  string `json:"status"`
}

type CalcpowRequest struct {
	BlockBlob    *string `json:"block_blob,omitempty"`
	Height       *int64  `json:"height,omitempty"`
	MajorVersion *int64  `json:"major_version,omitempty"`
	SeedHash     *string `json:"seed_hash,omitempty"`
}

type FlushCacheRequest struct {
	BadBlocks *bool `json:"bad_blocks,omitempty"`
}

type FlushCacheResponse struct {
	Status string `json:"status"`
}

type FlushTransactionPoolRequest struct {
	Txids []string `json:"txids,omitempty"`
}

type FlushTransactionPoolResponse struct {
	Status string `json:"status"`
}

type GenerateblocksRequest struct {
	AmountOfBlocks *int64  `json:"amount_of_blocks,omitempty"`
	PrevBlock      *string `json:"prev_block,omitempty"`
	StartingNonce  *int64  `json:"starting_nonce,omitempty"`
	WalletAddress  *string `json:"wallet_address,omitempty"`
}

type GenerateblocksResponse struct {
	Blocks []string `json:"blocks,omitempty"`
	Height int64    `json:"height"`
	Status string   `json:"status"`
}

type GetbansResponse struct {
	Bans   []GetbansBan `json:"bans,omitempty"`
	Status string       `json:"status"`
}

type GetbansBan struct {
	Host    string `json:"host"`
	IP      int64  `json:"ip"`
	Seconds int64  `json:"seconds"`
}

type GetblockcountResponse struct {
	Count  int64  `json:"count"`
	Status string `json:"status"`
}

type GetblocktemplateRequest struct {
	ExtraNonce *string `json:"extra_nonce,omitempty"`
	PrevBlock  *string `json:"prev_block,omitempty"`
	// max 255 bytes
	ReserveSize   *int64  `json:"reserve_size,omitempty"`
	WalletAddress *string `json:"wallet_address,omitempty"`
}

type GetblocktemplateResponse struct {
	BlockhashingBlob  string `json:"blockhashing_blob"`
	BlocktemplateBlob string `json:"blocktemplate_blob"`
	CumulativeWeight  *int64 `json:"cumulative_weight,omitempty"`
	Difficulty        int64  `json:"difficulty"`
	DifficultyTop64   int64  `json:"difficulty_top64"`
	ExpectedReward    int64  `json:"expected_reward"`
	Height            int64  `json:"height"`
	NextSeedHash      string `json:"next_seed_hash"`
	PrevHash          string `json:"prev_hash"`
	ReservedOffset    int64  `json:"reserved_offset"`
	SeedHash          string `json:"seed_hash"`
	SeedHeight        int64  `json:"seed_height"`
	Status            string `json:"status"`
	WideDifficulty    string `json:"wide_difficulty"`
}

type GetminerdataResponse struct {
	AlreadyGeneratedCoins int64                                `json:"already_generated_coins"`
	Difficulty            string                               `json:"difficulty"`
	Height                int64                                `json:"height"`
	MajorVersion          int64                                `json:"major_version"`
	MedianWeight          int64                                `json:"median_weight"`
	PrevID                string                               `json:"prev_id"`
	SeedHash              string                               `json:"seed_hash"`
	Status                string                               `json:"status"`
	TxBacklog             []GetminerdataResponseTxBacklogEntry `json:"tx_backlog,omitempty"`
}

type GetminerdataResponseTxBacklogEntry struct {
	Fee    int64  `json:"fee"`
	ID     string `json:"id"`
	Weight int64  `json:"weight"`
}

type GetAlternateChainsResponse struct {
	Chains []GetAlternateChainsChainInfo `json:"chains,omitempty"`
	Status string                        `json:"status"`
}

type GetAlternateChainsChainInfo struct {
	BlockHash            string   `json:"block_hash"`
	BlockHashes          []string `json:"block_hashes,omitempty"`
	Difficulty           int64    `json:"difficulty"`
	DifficultyTop64      int64    `json:"difficulty_top64"`
	Height               int64    `json:"height"`
	Length               int64    `json:"length"`
	MainChainParentBlock string   `json:"main_chain_parent_block"`
	WideDifficulty       string   `json:"wide_difficulty"`
}

type GetAltBlocksHashesRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetAltBlocksHashesResponse struct {
	BlksHashes []string `json:"blks_hashes,omitempty"`
	Credits    int64    `json:"credits"`
	Status     string   `json:"status"`
	TopHash    string   `json:"top_hash"`
}

type GetBaseFeeEstimateRequest struct {
	Client      *string `json:"client,omitempty"`
	GraceBlocks *int64  `json:"grace_blocks,omitempty"`
}

type GetBaseFeeEstimateResponse struct {
	Credits          int64   `json:"credits"`
	Fee              int64   `json:"fee"`
	Fees             []int64 `json:"fees,omitempty"`
	QuantizationMask *int64  `json:"quantization_mask,omitempty"`
	Status           string  `json:"status"`
	TopHash          string  `json:"top_hash"`
}

type GetBlockRequest struct {
	Client      *string `json:"client,omitempty"`
	FillPowHash *bool   `json:"fill_pow_hash,omitempty"`
	Hash        *string `json:"hash,omitempty"`
	Height      *int64  `json:"height,omitempty"`
}

type GetBlockResponse struct {
	Blob        string              `json:"blob"`
	BlockHeader BlockHeaderResponse `json:"block_header"`
	Credits     int64               `json:"credits"`
	JSON        string              `json:"json"`
	MinerTxHash string              `json:"miner_tx_hash"`
	Status      string              `json:"status"`
	TopHash     string              `json:"top_hash"`
	TxHashes    []string            `json:"tx_hashes,omitempty"`
}

type BlockHeaderResponse struct {
	BlockSize                 int64  `json:"block_size"`
	BlockWeight               *int64 `json:"block_weight,omitempty"`
	CumulativeDifficulty      int64  `json:"cumulative_difficulty"`
	CumulativeDifficultyTop64 int64  `json:"cumulative_difficulty_top64"`
	Depth                     int64  `json:"depth"`
	Difficulty                int64  `json:"difficulty"`
	DifficultyTop64           int64  `json:"difficulty_top64"`
	Hash                      string `json:"hash"`
	Height                    int64  `json:"height"`
	LongTermWeight            *int64 `json:"long_term_weight,omitempty"`
	MajorVersion              int64  `json:"major_version"`
	MinerTxHash               string `json:"miner_tx_hash"`
	MinorVersion              int64  `json:"minor_version"`
	Nonce                     int64  `json:"nonce"`
	NumTxes                   int64  `json:"num_txes"`
	OrphanStatus              bool   `json:"orphan_status"`
	PowHash                   string `json:"pow_hash"`
	PrevHash                  string `json:"prev_hash"`
	Reward                    int64  `json:"reward"`
	Timestamp                 int64  `json:"timestamp"`
	WideCumulativeDifficulty  string `json:"wide_cumulative_difficulty"`
	WideDifficulty            string `json:"wide_difficulty"`
}

type GetBlocksByHeightRequest struct {
	Client  *string `json:"client,omitempty"`
	Heights []int64 `json:"heights,omitempty"`
}

type GetBlocksByHeightResponse struct {
	Blocks  []BlockCompleteEntry `json:"blocks,omitempty"`
	Credits int64                `json:"credits"`
	Status  string               `json:"status"`
	TopHash string               `json:"top_hash"`
}

type BlockCompleteEntry struct {
	Block       string `json:"block"`
	BlockWeight *int64 `json:"block_weight,omitempty"`
	Pruned      *bool  `json:"pruned,omitempty"`
	Txs         []Tx   `json:"txs,omitempty"`
}

type TxBlobEntry struct {
	Blob         string `json:"blob"`
	PrunableHash string `json:"prunable_hash"`
}

type GetBlocksFastRequest struct {
	// first 10 blocks id goes sequential, next goes in pow(2,n) offset, like 2, 4, 8, 16, 32,
	// 64 and so on, and the last one is always genesis block
	BlockIDS          *string `json:"block_ids,omitempty"`
	BlockIDSExclusive *bool   `json:"block_ids_exclusive,omitempty"`
	Client            *string `json:"client,omitempty"`
	MaxBlockCount     *int64  `json:"max_block_count,omitempty"`
	NoMinerTx         *bool   `json:"no_miner_tx,omitempty"`
	PoolInfoSince     *int64  `json:"pool_info_since,omitempty"`
	Prune             *bool   `json:"prune,omitempty"`
	RequestedInfo     *int64  `json:"requested_info,omitempty"`
	StartHeight       *int64  `json:"start_height,omitempty"`
}

type GetBlocksFastResponse struct {
	AddedPoolTxs            []GetBlocksFastPoolTxInfo         `json:"added_pool_txs,omitempty"`
	Blocks                  []BlockCompleteEntry              `json:"blocks,omitempty"`
	Credits                 int64                             `json:"credits"`
	CurrentHeight           int64                             `json:"current_height"`
	DaemonTime              *int64                            `json:"daemon_time,omitempty"`
	OutputIndices           []GetBlocksFastBlockOutputIndices `json:"output_indices,omitempty"`
	PoolInfoExtent          *int64                            `json:"pool_info_extent,omitempty"`
	RemainingAddedPoolTxids *string                           `json:"remaining_added_pool_txids,omitempty"`
	RemovedPoolTxids        *string                           `json:"removed_pool_txids,omitempty"`
	StartHeight             int64                             `json:"start_height"`
	Status                  string                            `json:"status"`
	TopBlockHash            *string                           `json:"top_block_hash,omitempty"`
	TopHash                 string                            `json:"top_hash"`
}

type GetBlocksFastPoolTxInfo struct {
	DoubleSpendSeen bool   `json:"double_spend_seen"`
	TxBlob          string `json:"tx_blob"`
	TxHash          string `json:"tx_hash"`
}

type GetBlocksFastBlockOutputIndices struct {
	Indices []GetBlocksFastTxOutputIndices `json:"indices,omitempty"`
}

type GetBlocksFastTxOutputIndices struct {
	Indices []int64 `json:"indices,omitempty"`
}

type GetBlockHeadersRangeRequest struct {
	Client      *string `json:"client,omitempty"`
	EndHeight   *int64  `json:"end_height,omitempty"`
	FillPowHash *bool   `json:"fill_pow_hash,omitempty"`
	StartHeight *int64  `json:"start_height,omitempty"`
}

type GetBlockHeadersRangeResponse struct {
	Credits int64                 `json:"credits"`
	Headers []BlockHeaderResponse `json:"headers,omitempty"`
	Status  string                `json:"status"`
	TopHash string                `json:"top_hash"`
}

type GetBlockHeaderByHashRequest struct {
	Client      *string  `json:"client,omitempty"`
	FillPowHash *bool    `json:"fill_pow_hash,omitempty"`
	Hash        *string  `json:"hash,omitempty"`
	Hashes      []string `json:"hashes,omitempty"`
}

type GetBlockHeaderByHashResponse struct {
	BlockHeader  BlockHeaderResponse   `json:"block_header"`
	BlockHeaders []BlockHeaderResponse `json:"block_headers,omitempty"`
	Credits      int64                 `json:"credits"`
	Status       string                `json:"status"`
	TopHash      string                `json:"top_hash"`
}

type GetBlockHeaderByHeightRequest struct {
	Client      *string `json:"client,omitempty"`
	FillPowHash *bool   `json:"fill_pow_hash,omitempty"`
	Height      *int64  `json:"height,omitempty"`
}

type GetBlockHeaderByHeightResponse struct {
	BlockHeader BlockHeaderResponse `json:"block_header"`
	Credits     int64               `json:"credits"`
	Status      string              `json:"status"`
	TopHash     string              `json:"top_hash"`
}

type GetCoinbaseTxSumRequest struct {
	Client *string `json:"client,omitempty"`
	Count  *int64  `json:"count,omitempty"`
	Height *int64  `json:"height,omitempty"`
}

type GetCoinbaseTxSumResponse struct {
	Credits             int64  `json:"credits"`
	EmissionAmount      int64  `json:"emission_amount"`
	EmissionAmountTop64 int64  `json:"emission_amount_top64"`
	FeeAmount           int64  `json:"fee_amount"`
	FeeAmountTop64      int64  `json:"fee_amount_top64"`
	Status              string `json:"status"`
	TopHash             string `json:"top_hash"`
	WideEmissionAmount  string `json:"wide_emission_amount"`
	WideFeeAmount       string `json:"wide_fee_amount"`
}

type GetConnectionsResponse struct {
	Connections []ConnectionInfo `json:"connections,omitempty"`
	Status      string           `json:"status"`
}

type ConnectionInfo struct {
	Address           string `json:"address"`
	AddressType       int64  `json:"address_type"`
	AvgDownload       int64  `json:"avg_download"`
	AvgUpload         int64  `json:"avg_upload"`
	ConnectionID      string `json:"connection_id"`
	CurrentDownload   int64  `json:"current_download"`
	CurrentUpload     int64  `json:"current_upload"`
	Height            int64  `json:"height"`
	Host              string `json:"host"`
	Incoming          bool   `json:"incoming"`
	IP                string `json:"ip"`
	LiveTime          int64  `json:"live_time"`
	LocalIP           bool   `json:"local_ip"`
	Localhost         bool   `json:"localhost"`
	PeerID            string `json:"peer_id"`
	Port              string `json:"port"`
	PruningSeed       int64  `json:"pruning_seed"`
	RecvCount         int64  `json:"recv_count"`
	RecvIdleTime      int64  `json:"recv_idle_time"`
	RPCCreditsPerHash int64  `json:"rpc_credits_per_hash"`
	RPCPort           int64  `json:"rpc_port"`
	SendCount         int64  `json:"send_count"`
	SendIdleTime      int64  `json:"send_idle_time"`
	State             string `json:"state"`
	SupportFlags      int64  `json:"support_flags"`
}

type GetHashesFastRequest struct {
	// first 10 blocks id goes sequential, next goes in pow(2,n) offset, like 2, 4, 8, 16, 32,
	// 64 and so on, and the last one is always genesis block
	BlockIDS    *string `json:"block_ids,omitempty"`
	Client      *string `json:"client,omitempty"`
	StartHeight *int64  `json:"start_height,omitempty"`
}

type GetHashesFastResponse struct {
	Credits       int64   `json:"credits"`
	CurrentHeight int64   `json:"current_height"`
	MBlockIDS     *string `json:"m_block_ids,omitempty"`
	StartHeight   int64   `json:"start_height"`
	Status        string  `json:"status"`
	TopHash       string  `json:"top_hash"`
}

type GetHeightResponse struct {
	Hash   string `json:"hash"`
	Height int64  `json:"height"`
	Status string `json:"status"`
}

type GetInfoRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetInfoResponse struct {
	AdjustedTime              int64  `json:"adjusted_time"`
	AltBlocksCount            int64  `json:"alt_blocks_count"`
	BlockSizeLimit            int64  `json:"block_size_limit"`
	BlockSizeMedian           int64  `json:"block_size_median"`
	BlockWeightLimit          *int64 `json:"block_weight_limit,omitempty"`
	BlockWeightMedian         *int64 `json:"block_weight_median,omitempty"`
	BusySyncing               bool   `json:"busy_syncing"`
	Credits                   int64  `json:"credits"`
	CumulativeDifficulty      int64  `json:"cumulative_difficulty"`
	CumulativeDifficultyTop64 int64  `json:"cumulative_difficulty_top64"`
	DatabaseSize              int64  `json:"database_size"`
	Difficulty                int64  `json:"difficulty"`
	DifficultyTop64           int64  `json:"difficulty_top64"`
	FreeSpace                 int64  `json:"free_space"`
	GreyPeerlistSize          int64  `json:"grey_peerlist_size"`
	Height                    int64  `json:"height"`
	IncomingConnectionsCount  int64  `json:"incoming_connections_count"`
	Mainnet                   bool   `json:"mainnet"`
	Nettype                   string `json:"nettype"`
	Offline                   bool   `json:"offline"`
	OutgoingConnectionsCount  int64  `json:"outgoing_connections_count"`
	Restricted                bool   `json:"restricted"`
	RPCConnectionsCount       int64  `json:"rpc_connections_count"`
	Stagenet                  bool   `json:"stagenet"`
	StartTime                 int64  `json:"start_time"`
	Status                    string `json:"status"`
	Synchronized              bool   `json:"synchronized"`
	Target                    int64  `json:"target"`
	TargetHeight              int64  `json:"target_height"`
	Testnet                   bool   `json:"testnet"`
	TopBlockHash              string `json:"top_block_hash"`
	TopHash                   string `json:"top_hash"`
	TxCount                   int64  `json:"tx_count"`
	TxPoolSize                int64  `json:"tx_pool_size"`
	UpdateAvailable           bool   `json:"update_available"`
	Version                   string `json:"version"`
	WhitePeerlistSize         int64  `json:"white_peerlist_size"`
	WideCumulativeDifficulty  string `json:"wide_cumulative_difficulty"`
	WideDifficulty            string `json:"wide_difficulty"`
}

type GetLastBlockHeaderRequest struct {
	Client      *string `json:"client,omitempty"`
	FillPowHash *bool   `json:"fill_pow_hash,omitempty"`
}

type GetLastBlockHeaderResponse struct {
	BlockHeader BlockHeaderResponse `json:"block_header"`
	Credits     int64               `json:"credits"`
	Status      string              `json:"status"`
	TopHash     string              `json:"top_hash"`
}

type GetLimitResponse struct {
	LimitDown int64  `json:"limit_down"`
	LimitUp   int64  `json:"limit_up"`
	Status    string `json:"status"`
}

type GetNetStatsResponse struct {
	StartTime       int64  `json:"start_time"`
	Status          string `json:"status"`
	TotalBytesIn    int64  `json:"total_bytes_in"`
	TotalBytesOut   int64  `json:"total_bytes_out"`
	TotalPacketsIn  int64  `json:"total_packets_in"`
	TotalPacketsOut int64  `json:"total_packets_out"`
}

type GetOutputsRequest struct {
	Client  *string         `json:"client,omitempty"`
	GetTxid *bool           `json:"get_txid,omitempty"`
	Outputs []GetOutputsOut `json:"outputs,omitempty"`
}

type GetOutputsOut struct {
	Amount *int64 `json:"amount,omitempty"`
	Index  *int64 `json:"index,omitempty"`
}

type GetOutputsResponse struct {
	Credits int64              `json:"credits"`
	Outs    []GetOutputsOutkey `json:"outs,omitempty"`
	Status  string             `json:"status"`
	TopHash string             `json:"top_hash"`
}

type GetOutputsOutkey struct {
	Height   int64  `json:"height"`
	Key      string `json:"key"`
	Mask     string `json:"mask"`
	Txid     string `json:"txid"`
	Unlocked bool   `json:"unlocked"`
}

type GetOutputsBinRequest struct {
	Client  *string         `json:"client,omitempty"`
	GetTxid *bool           `json:"get_txid,omitempty"`
	Outputs []GetOutputsOut `json:"outputs,omitempty"`
}

type GetOutputsBinResponse struct {
	Credits int64                 `json:"credits"`
	Outs    []GetOutputsBinOutkey `json:"outs,omitempty"`
	Status  string                `json:"status"`
	TopHash string                `json:"top_hash"`
}

type GetOutputsBinOutkey struct {
	Height   int64  `json:"height"`
	Key      string `json:"key"`
	Mask     string `json:"mask"`
	Txid     string `json:"txid"`
	Unlocked bool   `json:"unlocked"`
}

type GetOutputDistributionRequest struct {
	Amounts    []int64 `json:"amounts,omitempty"`
	Binary     *bool   `json:"binary,omitempty"`
	Client     *string `json:"client,omitempty"`
	Compress   *bool   `json:"compress,omitempty"`
	Cumulative *bool   `json:"cumulative,omitempty"`
	FromHeight *int64  `json:"from_height,omitempty"`
	ToHeight   *int64  `json:"to_height,omitempty"`
}

type GetOutputDistributionResponse struct {
	Credits       int64                               `json:"credits"`
	Distributions []GetOutputDistributionDistribution `json:"distributions,omitempty"`
	Status        string                              `json:"status"`
	TopHash       string                              `json:"top_hash"`
}

type GetOutputDistributionDistribution struct {
	Amount         int64         `json:"amount"`
	Base           int64         `json:"base"`
	Binary         bool          `json:"binary"`
	Compress       bool          `json:"compress"`
	CompressedData *string       `json:"compressed_data,omitempty"`
	Distribution   *Distribution `json:"distribution"`
	StartHeight    int64         `json:"start_height"`
}

type GetOutputHistogramRequest struct {
	Amounts      []int64 `json:"amounts,omitempty"`
	Client       *string `json:"client,omitempty"`
	MaxCount     *int64  `json:"max_count,omitempty"`
	MinCount     *int64  `json:"min_count,omitempty"`
	RecentCutoff *int64  `json:"recent_cutoff,omitempty"`
	Unlocked     *bool   `json:"unlocked,omitempty"`
}

type GetOutputHistogramResponse struct {
	Credits   int64                     `json:"credits"`
	Histogram []GetOutputHistogramEntry `json:"histogram,omitempty"`
	Status    string                    `json:"status"`
	TopHash   string                    `json:"top_hash"`
}

type GetOutputHistogramEntry struct {
	Amount            int64 `json:"amount"`
	RecentInstances   int64 `json:"recent_instances"`
	TotalInstances    int64 `json:"total_instances"`
	UnlockedInstances int64 `json:"unlocked_instances"`
}

type GetPeerListRequest struct {
	IncludeBlocked *bool `json:"include_blocked,omitempty"`
	PublicOnly     *bool `json:"public_only,omitempty"`
}

type GetPeerListResponse struct {
	GrayList  []Peer `json:"gray_list,omitempty"`
	Status    string `json:"status"`
	WhiteList []Peer `json:"white_list,omitempty"`
}

type Peer struct {
	Host              string `json:"host"`
	ID                int64  `json:"id"`
	IP                int64  `json:"ip"`
	LastSeen          int64  `json:"last_seen"`
	Port              int64  `json:"port"`
	PruningSeed       *int64 `json:"pruning_seed,omitempty"`
	RPCCreditsPerHash *int64 `json:"rpc_credits_per_hash,omitempty"`
	RPCPort           *int64 `json:"rpc_port,omitempty"`
}

type GetPublicNodesRequest struct {
	Gray           *bool `json:"gray,omitempty"`
	IncludeBlocked *bool `json:"include_blocked,omitempty"`
	White          *bool `json:"white,omitempty"`
}

type GetPublicNodesResponse struct {
	Gray   []PublicNode `json:"gray,omitempty"`
	Status string       `json:"status"`
	White  []PublicNode `json:"white,omitempty"`
}

type PublicNode struct {
	Host              string `json:"host"`
	LastSeen          int64  `json:"last_seen"`
	RPCCreditsPerHash int64  `json:"rpc_credits_per_hash"`
	RPCPort           int64  `json:"rpc_port"`
}

type GetTransactionsRequest struct {
	Client       *string  `json:"client,omitempty"`
	DecodeAsJSON *bool    `json:"decode_as_json,omitempty"`
	Prune        *bool    `json:"prune,omitempty"`
	Split        *bool    `json:"split,omitempty"`
	TxsHashes    []string `json:"txs_hashes,omitempty"`
}

type GetTransactionsResponse struct {
	Credits int64 `json:"credits"`
	// not found transactions
	MissedTx []string               `json:"missed_tx,omitempty"`
	Status   string                 `json:"status"`
	TopHash  string                 `json:"top_hash"`
	Txs      []GetTransactionsEntry `json:"txs,omitempty"`
	// transactions blobs as hex (old compat)
	TxsAsHex []string `json:"txs_as_hex,omitempty"`
	// transactions decoded as json (old compat)
	TxsAsJSON []string `json:"txs_as_json,omitempty"`
}

type GetTransactionsEntry struct {
	AsHex             string  `json:"as_hex"`
	AsJSON            string  `json:"as_json"`
	BlockHeight       *int64  `json:"block_height,omitempty"`
	BlockTimestamp    *int64  `json:"block_timestamp,omitempty"`
	Confirmations     *int64  `json:"confirmations,omitempty"`
	DoubleSpendSeen   bool    `json:"double_spend_seen"`
	InPool            bool    `json:"in_pool"`
	OutputIndices     []int64 `json:"output_indices,omitempty"`
	PrunableAsHex     string  `json:"prunable_as_hex"`
	PrunableHash      string  `json:"prunable_hash"`
	PrunedAsHex       string  `json:"pruned_as_hex"`
	ReceivedTimestamp *int64  `json:"received_timestamp,omitempty"`
	Relayed           *bool   `json:"relayed,omitempty"`
	TxHash            string  `json:"tx_hash"`
}

type GetTransactionPoolRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetTransactionPoolResponse struct {
	Credits        int64               `json:"credits"`
	SpentKeyImages []SpentKeyImageInfo `json:"spent_key_images,omitempty"`
	Status         string              `json:"status"`
	TopHash        string              `json:"top_hash"`
	Transactions   []TxInfo            `json:"transactions,omitempty"`
}

type SpentKeyImageInfo struct {
	IDHash    string   `json:"id_hash"`
	TxsHashes []string `json:"txs_hashes,omitempty"`
}

type TxInfo struct {
	BlobSize           int64  `json:"blob_size"`
	DoNotRelay         bool   `json:"do_not_relay"`
	DoubleSpendSeen    bool   `json:"double_spend_seen"`
	Fee                int64  `json:"fee"`
	IDHash             string `json:"id_hash"`
	KeptByBlock        bool   `json:"kept_by_block"`
	LastFailedHeight   int64  `json:"last_failed_height"`
	LastFailedIDHash   string `json:"last_failed_id_hash"`
	LastRelayedTime    int64  `json:"last_relayed_time"`
	MaxUsedBlockHeight int64  `json:"max_used_block_height"`
	MaxUsedBlockIDHash string `json:"max_used_block_id_hash"`
	ReceiveTime        int64  `json:"receive_time"`
	Relayed            bool   `json:"relayed"`
	TxBlob             string `json:"tx_blob"`
	// TODO - expose this data directly
	TxJSON string `json:"tx_json"`
	Weight *int64 `json:"weight,omitempty"`
}

type GetTransactionPoolBacklogRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetTransactionPoolBacklogResponse struct {
	Backlog *string `json:"backlog,omitempty"`
	Credits int64   `json:"credits"`
	Status  string  `json:"status"`
	TopHash string  `json:"top_hash"`
}

type GetTransactionPoolHashesRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetTransactionPoolHashesResponse struct {
	Credits  int64    `json:"credits"`
	Status   string   `json:"status"`
	TopHash  string   `json:"top_hash"`
	TxHashes []string `json:"tx_hashes,omitempty"`
}

type GetTransactionPoolHashesBinRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetTransactionPoolHashesBinResponse struct {
	Credits  int64   `json:"credits"`
	Status   string  `json:"status"`
	TopHash  string  `json:"top_hash"`
	TxHashes *string `json:"tx_hashes,omitempty"`
}

type GetTransactionPoolStatsRequest struct {
	Client *string `json:"client,omitempty"`
}

type GetTransactionPoolStatsResponse struct {
	Credits   int64       `json:"credits"`
	PoolStats TxpoolStats `json:"pool_stats"`
	Status    string      `json:"status"`
	TopHash   string      `json:"top_hash"`
}

type TxpoolStats struct {
	BytesMax        int64         `json:"bytes_max"`
	BytesMed        int64         `json:"bytes_med"`
	BytesMin        int64         `json:"bytes_min"`
	BytesTotal      int64         `json:"bytes_total"`
	FeeTotal        int64         `json:"fee_total"`
	Histo           []TxpoolHisto `json:"histo,omitempty"`
	Histo98PC       int64         `json:"histo_98pc"`
	Num10M          int64         `json:"num_10m"`
	NumDoubleSpends int64         `json:"num_double_spends"`
	NumFailing      int64         `json:"num_failing"`
	NumNotRelayed   int64         `json:"num_not_relayed"`
	Oldest          int64         `json:"oldest"`
	TxsTotal        int64         `json:"txs_total"`
}

type TxpoolHisto struct {
	Bytes int64 `json:"bytes"`
	Txs   int64 `json:"txs"`
}

type GetTxidsLooseRequest struct {
	NumMatchingBits *int64  `json:"num_matching_bits,omitempty"`
	TxidTemplate    *string `json:"txid_template,omitempty"`
}

type GetTxidsLooseResponse struct {
	Status string   `json:"status"`
	Txids  []string `json:"txids,omitempty"`
}

type GetTxGlobalOutputsIndexesRequest struct {
	Client *string `json:"client,omitempty"`
	Txid   *string `json:"txid,omitempty"`
}

type GetTxGlobalOutputsIndexesResponse struct {
	Credits  int64   `json:"credits"`
	OIndexes []int64 `json:"o_indexes,omitempty"`
	Status   string  `json:"status"`
	TopHash  string  `json:"top_hash"`
}

type GetVersionResponse struct {
	CurrentHeight *int64              `json:"current_height,omitempty"`
	HardForks     []GetVersionHFEntry `json:"hard_forks,omitempty"`
	Nettype       *string             `json:"nettype,omitempty"`
	Release       bool                `json:"release"`
	Status        string              `json:"status"`
	TargetHeight  *int64              `json:"target_height,omitempty"`
	Version       int64               `json:"version"`
}

type GetVersionHFEntry struct {
	Height    int64 `json:"height"`
	HFVersion int64 `json:"hf_version"`
}

type HardForkInfoRequest struct {
	Client  *string `json:"client,omitempty"`
	Version *int64  `json:"version,omitempty"`
}

type HardForkInfoResponse struct {
	Credits        int64  `json:"credits"`
	EarliestHeight int64  `json:"earliest_height"`
	Enabled        bool   `json:"enabled"`
	State          int64  `json:"state"`
	Status         string `json:"status"`
	Threshold      int64  `json:"threshold"`
	TopHash        string `json:"top_hash"`
	Version        int64  `json:"version"`
	Votes          int64  `json:"votes"`
	Voting         int64  `json:"voting"`
	Window         int64  `json:"window"`
}

type InPeersRequest struct {
	InPeers *int64 `json:"in_peers,omitempty"`
	Set     *bool  `json:"set,omitempty"`
}

type InPeersResponse struct {
	InPeers int64  `json:"in_peers"`
	Status  string `json:"status"`
}

type IsKeyImageSpentRequest struct {
	Client    *string  `json:"client,omitempty"`
	KeyImages []string `json:"key_images,omitempty"`
}

type IsKeyImageSpentResponse struct {
	Credits     int64   `json:"credits"`
	SpentStatus []int64 `json:"spent_status,omitempty"`
	Status      string  `json:"status"`
	TopHash     string  `json:"top_hash"`
}

type MiningStatusResponse struct {
	Active                    bool   `json:"active"`
	Address                   string `json:"address"`
	BgIdleThreshold           int64  `json:"bg_idle_threshold"`
	BgIgnoreBattery           bool   `json:"bg_ignore_battery"`
	BgMinIdleSeconds          int64  `json:"bg_min_idle_seconds"`
	BgTarget                  int64  `json:"bg_target"`
	BlockReward               int64  `json:"block_reward"`
	BlockTarget               int64  `json:"block_target"`
	Difficulty                int64  `json:"difficulty"`
	DifficultyTop64           int64  `json:"difficulty_top64"`
	IsBackgroundMiningEnabled bool   `json:"is_background_mining_enabled"`
	PowAlgorithm              string `json:"pow_algorithm"`
	Speed                     int64  `json:"speed"`
	Status                    string `json:"status"`
	ThreadsCount              int64  `json:"threads_count"`
	WideDifficulty            string `json:"wide_difficulty"`
}

type OutPeersRequest struct {
	OutPeers *int64 `json:"out_peers,omitempty"`
	Set      *bool  `json:"set,omitempty"`
}

type OutPeersResponse struct {
	OutPeers int64  `json:"out_peers"`
	Status   string `json:"status"`
}

type PopBlocksRequest struct {
	KeepTxs *bool  `json:"keep_txs,omitempty"`
	Nblocks *int64 `json:"nblocks,omitempty"`
}

type PopBlocksResponse struct {
	Height int64  `json:"height"`
	Status string `json:"status"`
}

type PruneBlockchainRequest struct {
	Check *bool `json:"check,omitempty"`
}

type PruneBlockchainResponse struct {
	Pruned      bool   `json:"pruned"`
	PruningSeed int64  `json:"pruning_seed"`
	Status      string `json:"status"`
}

type RelayTxRequest struct {
	Client *string  `json:"client,omitempty"`
	Txids  []string `json:"txids,omitempty"`
}

type RelayTxResponse struct {
	Credits int64  `json:"credits"`
	Status  string `json:"status"`
	TopHash string `json:"top_hash"`
}

type SaveBcResponse struct {
	Status string `json:"status"`
}

type SendRawTxRequest struct {
	Client         *string `json:"client,omitempty"`
	DoNotRelay     *bool   `json:"do_not_relay,omitempty"`
	DoSanityChecks *bool   `json:"do_sanity_checks,omitempty"`
	TxAsHex        *string `json:"tx_as_hex,omitempty"`
}

type SendRawTxResponse struct {
	Credits           int64  `json:"credits"`
	DoubleSpend       bool   `json:"double_spend"`
	FeeTooLow         bool   `json:"fee_too_low"`
	InvalidInput      bool   `json:"invalid_input"`
	InvalidOutput     bool   `json:"invalid_output"`
	LowMixin          bool   `json:"low_mixin"`
	NonzeroUnlockTime bool   `json:"nonzero_unlock_time"`
	NotRelayed        bool   `json:"not_relayed"`
	Overspend         bool   `json:"overspend"`
	Reason            string `json:"reason"`
	SanityCheckFailed bool   `json:"sanity_check_failed"`
	Status            string `json:"status"`
	TooBig            bool   `json:"too_big"`
	TooFewOutputs     bool   `json:"too_few_outputs"`
	TopHash           string `json:"top_hash"`
	TxExtraTooBig     bool   `json:"tx_extra_too_big"`
}

type SetbansRequest struct {
	Bans []SetbansBan `json:"bans,omitempty"`
}

type SetbansBan struct {
	Ban     *bool   `json:"ban,omitempty"`
	Host    *string `json:"host,omitempty"`
	IP      *int64  `json:"ip,omitempty"`
	Seconds *int64  `json:"seconds,omitempty"`
}

type SetbansResponse struct {
	Status string `json:"status"`
}

type SetLimitRequest struct {
	// all limits (for get and set) are kB/s
	LimitDown *int64 `json:"limit_down,omitempty"`
	LimitUp   *int64 `json:"limit_up,omitempty"`
}

type SetLimitResponse struct {
	LimitDown int64  `json:"limit_down"`
	LimitUp   int64  `json:"limit_up"`
	Status    string `json:"status"`
}

type SetLogCategoriesRequest struct {
	Categories *string `json:"categories,omitempty"`
}

type SetLogCategoriesResponse struct {
	Categories string `json:"categories"`
	Status     string `json:"status"`
}

type SetLogHashRateRequest struct {
	Visible *bool `json:"visible,omitempty"`
}

type SetLogHashRateResponse struct {
	Status string `json:"status"`
}

type SetLogLevelRequest struct {
	Level *int64 `json:"level,omitempty"`
}

type SetLogLevelResponse struct {
	Status string `json:"status"`
}

type StartMiningRequest struct {
	DoBackgroundMining *bool   `json:"do_background_mining,omitempty"`
	IgnoreBattery      *bool   `json:"ignore_battery,omitempty"`
	MinerAddress       *string `json:"miner_address,omitempty"`
	ThreadsCount       *int64  `json:"threads_count,omitempty"`
}

type StartMiningResponse struct {
	Status string `json:"status"`
}

type StopDaemonResponse struct {
	Status string `json:"status"`
}

type StopMiningResponse struct {
	Status string `json:"status"`
}

type SubmitblockResponse struct {
	BlockID string `json:"block_id"`
	Status  string `json:"status"`
}

type SyncInfoRequest struct {
	Client *string `json:"client,omitempty"`
}

type SyncInfoResponse struct {
	Credits               int64          `json:"credits"`
	Height                int64          `json:"height"`
	NextNeededPruningSeed int64          `json:"next_needed_pruning_seed"`
	Overview              string         `json:"overview"`
	Peers                 []SyncInfoPeer `json:"peers,omitempty"`
	Spans                 []SyncInfoSpan `json:"spans,omitempty"`
	Status                string         `json:"status"`
	TargetHeight          int64          `json:"target_height"`
	TopHash               string         `json:"top_hash"`
}

type SyncInfoPeer struct {
	Info ConnectionInfo `json:"info"`
}

type SyncInfoSpan struct {
	ConnectionID     string `json:"connection_id"`
	Nblocks          int64  `json:"nblocks"`
	Rate             int64  `json:"rate"`
	RemoteAddress    string `json:"remote_address"`
	Size             int64  `json:"size"`
	Speed            int64  `json:"speed"`
	StartBlockHeight int64  `json:"start_block_height"`
}

type UpdateRequest struct {
	Command *string `json:"command,omitempty"`
	Path    *string `json:"path,omitempty"`
}

type UpdateResponse struct {
	AutoURI string `json:"auto_uri"`
	Hash    string `json:"hash"`
	Path    string `json:"path"`
	Status  string `json:"status"`
	Update  bool   `json:"update"`
	UserURI string `json:"user_uri"`
	Version string `json:"version"`
}

type Tx struct {
	String      *string
	TxBlobEntry *TxBlobEntry
}

type Distribution struct {
	IntegerArray []int64
	String       *string
}
