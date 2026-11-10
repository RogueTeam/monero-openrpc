// Package jsonrpc is a minimal client for monerod and monero-wallet-rpc.
//
// It sends JSON-RPC 2.0 calls to /json_rpc (Call) and plain JSON requests to the
// other endpoints such as /get_height (Post). Request and response values are the
// types generated from the schema, for example monerod.GetInfoResponse.
//
// It doesn't implement HTTP digest authentication (--rpc-login); put a proxy in
// front or set HTTPClient.Transport if you need it.
package jsonrpc

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
)

// Client talks to one monerod or monero-wallet-rpc instance.
type Client struct {
	BaseURL    string // e.g. "http://127.0.0.1:18081"
	HTTPClient *http.Client
}

// New returns a Client for baseURL using http.DefaultClient.
func New(baseURL string) *Client {
	return &Client{BaseURL: strings.TrimRight(baseURL, "/"), HTTPClient: http.DefaultClient}
}

// Error is a JSON-RPC error returned by the server.
type Error struct {
	Code    int64  `json:"code"`
	Message string `json:"message"`
}

func (e *Error) Error() string { return fmt.Sprintf("json-rpc error %d: %s", e.Code, e.Message) }

type request struct {
	JSONRPC string      `json:"jsonrpc"`
	ID      string      `json:"id"`
	Method  string      `json:"method"`
	Params  interface{} `json:"params,omitempty"`
}

type response struct {
	Result json.RawMessage `json:"result"`
	Error  *Error          `json:"error"`
}

// Call invokes a JSON-RPC method. params is a request struct for by-name methods
// (e.g. &monerod.GetBlockRequest{...}), a slice for by-position methods such as
// on_get_block_hash, or nil. result is a pointer to the response type.
func (c *Client) Call(ctx context.Context, method string, params, result interface{}) error {
	var resp response
	if err := c.post(ctx, "/json_rpc", request{"2.0", "0", method, params}, &resp); err != nil {
		return err
	}
	if resp.Error != nil {
		return resp.Error
	}
	if result == nil || len(resp.Result) == 0 {
		return nil
	}
	return json.Unmarshal(resp.Result, result)
}

// Post sends a request to a plain JSON endpoint such as "/get_height".
func (c *Client) Post(ctx context.Context, path string, req, result interface{}) error {
	return c.post(ctx, path, req, result)
}

func (c *Client) post(ctx context.Context, path string, body, out interface{}) error {
	payload := []byte("{}")
	if body != nil {
		var err error
		if payload, err = json.Marshal(body); err != nil {
			return err
		}
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, c.BaseURL+path, bytes.NewReader(payload))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	httpClient := c.HTTPClient
	if httpClient == nil {
		httpClient = http.DefaultClient
	}
	res, err := httpClient.Do(req)
	if err != nil {
		return err
	}
	defer res.Body.Close()
	data, err := io.ReadAll(res.Body)
	if err != nil {
		return err
	}
	if res.StatusCode != http.StatusOK {
		return fmt.Errorf("%s: HTTP %d: %s", path, res.StatusCode, bytes.TrimSpace(data))
	}
	if out == nil {
		return nil
	}
	return json.Unmarshal(data, out)
}
