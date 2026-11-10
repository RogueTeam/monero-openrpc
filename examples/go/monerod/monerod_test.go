package monerod

import (
	"encoding/json"
	"testing"
)

// Optional fields are pointers and are left out of the JSON when nil, so a request
// only carries the params you set.
func TestOptionalParamsOmitted(t *testing.T) {
	height := int64(912345)
	b, err := json.Marshal(GetBlockRequest{Height: &height})
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != `{"height":912345}` {
		t.Errorf("got %s, want {\"height\":912345}", b)
	}
}
