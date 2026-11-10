package walletrpc

import (
	"encoding/json"
	"testing"
)

// The wallet's get_transfers has a param named "in", a Go-safe name is generated
// for it, but the JSON name must stay "in".
func TestReservedWordParam(t *testing.T) {
	yes := true
	b, err := json.Marshal(GetTransfersRequest{In: &yes})
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != `{"in":true}` {
		t.Errorf("got %s, want {\"in\":true}", b)
	}
}
