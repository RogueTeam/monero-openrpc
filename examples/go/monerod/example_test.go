package monerod_test

import (
	"context"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/http/httptest"

	"monero-rpc-examples/jsonrpc"
	"monero-rpc-examples/monerod"
)

// Calling get_info. Against a real node, use jsonrpc.New("http://127.0.0.1:18081");
// here a stand-in server answers so the example runs as a test.
func Example() {
	node := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		io.WriteString(w, `{"jsonrpc":"2.0","id":"0","result":{"status":"OK","height":3195144,
			"top_block_hash":"0a7a5f2b6d4ed7d0b3c0e8f2d2a0f2b1e6c4d9a8b7c6d5e4f3a2b1c0d9e8f7a6",
			"nettype":"mainnet","synchronized":true}}`)
	}))
	defer node.Close()

	client := jsonrpc.New(node.URL)
	var info monerod.GetInfoResponse
	if err := client.Call(context.Background(), "get_info", &monerod.GetInfoRequest{}, &info); err != nil {
		log.Fatal(err)
	}
	fmt.Println(info.Height, info.Nettype, info.Synchronized)
	// Output: 3195144 mainnet true
}
