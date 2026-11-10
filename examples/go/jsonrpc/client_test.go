package jsonrpc

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"reflect"
	"testing"
)

// server returns a test server that records the last request and answers with reply.
func server(t *testing.T, status int, reply string, seen *map[string]interface{}, path *string) *httptest.Server {
	t.Helper()
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost || r.Header.Get("Content-Type") != "application/json" {
			t.Errorf("got %s with Content-Type %q, want a JSON POST", r.Method, r.Header.Get("Content-Type"))
		}
		body, _ := io.ReadAll(r.Body)
		*seen = map[string]interface{}{}
		if err := json.Unmarshal(body, seen); err != nil {
			t.Errorf("request body isn't a JSON object: %s", body)
		}
		*path = r.URL.Path
		w.WriteHeader(status)
		_, _ = io.WriteString(w, reply)
	}))
	t.Cleanup(srv.Close)
	return srv
}

type heightRequest struct {
	Height uint64 `json:"height"`
}

type headerResponse struct {
	Status string `json:"status"`
	Height uint64 `json:"height"`
}

func TestCallByName(t *testing.T) {
	var seen map[string]interface{}
	var path string
	srv := server(t, 200, `{"jsonrpc":"2.0","id":"0","result":{"status":"OK","height":3000000}}`, &seen, &path)

	var res headerResponse
	if err := New(srv.URL).Call(context.Background(), "get_block_header_by_height", &heightRequest{3000000}, &res); err != nil {
		t.Fatal(err)
	}
	if path != "/json_rpc" {
		t.Errorf("path = %q, want /json_rpc", path)
	}
	want := map[string]interface{}{"jsonrpc": "2.0", "id": "0", "method": "get_block_header_by_height",
		"params": map[string]interface{}{"height": float64(3000000)}}
	if !reflect.DeepEqual(seen, want) {
		t.Errorf("sent %v, want %v", seen, want)
	}
	if res != (headerResponse{"OK", 3000000}) {
		t.Errorf("result = %+v", res)
	}
}

func TestCallByPosition(t *testing.T) {
	var seen map[string]interface{}
	var path string
	srv := server(t, 200, `{"jsonrpc":"2.0","id":"0","result":"e22cf75f39ae720e8b71b3d120a5ac03f0db50bba6379e2850975b4859190bc6"}`, &seen, &path)

	var hash string
	if err := New(srv.URL).Call(context.Background(), "on_get_block_hash", []uint64{912345}, &hash); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(seen["params"], []interface{}{float64(912345)}) {
		t.Errorf("params = %v, want [912345]", seen["params"])
	}
	if len(hash) != 64 {
		t.Errorf("hash = %q", hash)
	}
}

func TestCallWithoutParams(t *testing.T) {
	var seen map[string]interface{}
	var path string
	srv := server(t, 200, `{"jsonrpc":"2.0","id":"0","result":{"count":993163,"status":"OK"}}`, &seen, &path)

	var res struct {
		Count uint64 `json:"count"`
	}
	if err := New(srv.URL).Call(context.Background(), "get_block_count", nil, &res); err != nil {
		t.Fatal(err)
	}
	if _, ok := seen["params"]; ok {
		t.Errorf("params sent for a method without params: %v", seen["params"])
	}
	if res.Count != 993163 {
		t.Errorf("count = %d", res.Count)
	}
}

func TestCallError(t *testing.T) {
	var seen map[string]interface{}
	var path string
	srv := server(t, 200, `{"jsonrpc":"2.0","id":"0","error":{"code":-7,"message":"Block not accepted"}}`, &seen, &path)

	err := New(srv.URL).Call(context.Background(), "submit_block", []string{"0707e6bdfedc05"}, nil)
	var rpcErr *Error
	if !errors.As(err, &rpcErr) || rpcErr.Code != -7 || rpcErr.Message != "Block not accepted" {
		t.Fatalf("err = %v, want json-rpc error -7", err)
	}
}

func TestHTTPError(t *testing.T) {
	var seen map[string]interface{}
	var path string
	srv := server(t, 401, `Unauthorized`, &seen, &path)

	if err := New(srv.URL).Call(context.Background(), "get_info", nil, nil); err == nil {
		t.Fatal("expected an error for HTTP 401")
	}
}

func TestPost(t *testing.T) {
	var seen map[string]interface{}
	var path string
	srv := server(t, 200, `{"hash":"7e23a28cfa6df925d5b63940baf60b83c0cbb65da95f49b19e7cf0ce7dd709ce","height":2287217,"status":"OK","untrusted":false}`, &seen, &path)

	var res struct {
		Height uint64 `json:"height"`
		Hash   string `json:"hash"`
	}
	if err := New(srv.URL+"/").Post(context.Background(), "/get_height", nil, &res); err != nil {
		t.Fatal(err)
	}
	if path != "/get_height" || len(seen) != 0 {
		t.Errorf("sent %v to %q, want {} to /get_height", seen, path)
	}
	if res.Height != 2287217 {
		t.Errorf("height = %d", res.Height)
	}
}
