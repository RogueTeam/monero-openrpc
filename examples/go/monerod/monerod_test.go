package monerod

import (
	"encoding/json"
	"math"
	"testing"

	"monero-rpc-examples/internal/fixturetest"
)

// Every request/response example in the official docs decodes into the generated types.
func TestDocsExamples(t *testing.T) {
	fixturetest.Run(t, "../../fixtures/daemon.json", newOf)
}

// uint64 fields must hold values above 2^63-1 (quicktype alone would generate int64).
func TestUint64AboveInt64(t *testing.T) {
	var r GetminerdataResponse
	err := json.Unmarshal([]byte(`{"status":"OK","major_version":16,"height":3000000,
		"prev_id":"","seed_hash":"","difficulty":"0x1","median_weight":300000,
		"already_generated_coins":18446744073709551615}`), &r)
	if err != nil {
		t.Fatal(err)
	}
	if r.AlreadyGeneratedCoins != math.MaxUint64 {
		t.Errorf("already_generated_coins = %d, want %d", r.AlreadyGeneratedCoins, uint64(math.MaxUint64))
	}
}

// Optional fields are pointers and are left out of the JSON when nil, so a request
// only carries the params you set.
func TestOptionalParamsOmitted(t *testing.T) {
	height := uint64(912345)
	b, err := json.Marshal(GetBlockRequest{Height: &height})
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != `{"height":912345}` {
		t.Errorf("got %s, want {\"height\":912345}", b)
	}
}
