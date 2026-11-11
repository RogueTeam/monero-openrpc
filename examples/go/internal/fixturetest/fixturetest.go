// Package fixturetest checks generated Go types against the docs examples in
// examples/fixtures (see tools/make_fixtures.py).
package fixturetest

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math"
	"math/big"
	"os"
	"reflect"
	"sort"
	"testing"
)

type fixture struct {
	Cases []struct {
		Name      string          `json:"name"`
		Kind      string          `json:"kind"`
		Type      *string         `json:"type"`
		Status    string          `json:"status"`
		Notes     []string        `json:"notes"`
		JSON      json.RawMessage `json:"json"`
		KnownJSON json.RawMessage `json:"known_json"`
	} `json:"cases"`
}

func normalize(t *testing.T, b []byte) interface{} {
	t.Helper()
	var v interface{}
	d := json.NewDecoder(bytes.NewReader(b))
	d.UseNumber()
	if err := d.Decode(&v); err != nil {
		t.Fatal(err)
	}
	return v
}

// diff returns where got and want differ ("" if they don't). Integers are compared
// exactly, other numbers with a small relative tolerance (formatting may differ).
func diff(got, want interface{}, path string) string {
	switch g := got.(type) {
	case map[string]interface{}:
		w, ok := want.(map[string]interface{})
		if !ok {
			return fmt.Sprintf("%s: object vs %T", path, want)
		}
		keys := map[string]bool{}
		for k := range g {
			keys[k] = true
		}
		for k := range w {
			keys[k] = true
		}
		sorted := make([]string, 0, len(keys))
		for k := range keys {
			sorted = append(sorted, k)
		}
		sort.Strings(sorted)
		for _, k := range sorted {
			gv, gok := g[k]
			wv, wok := w[k]
			switch {
			case !wok:
				return path + "/" + k + ": written by the generated type but not in the example"
			case !gok:
				return path + "/" + k + ": in the example but lost by the generated type"
			}
			if d := diff(gv, wv, path+"/"+k); d != "" {
				return d
			}
		}
	case []interface{}:
		w, ok := want.([]interface{})
		if !ok || len(w) != len(g) {
			return path + ": array length or type differs"
		}
		for i := range g {
			if d := diff(g[i], w[i], fmt.Sprintf("%s/%d", path, i)); d != "" {
				return d
			}
		}
	case json.Number:
		w, ok := want.(json.Number)
		if !ok {
			return fmt.Sprintf("%s: number vs %T", path, want)
		}
		gi, gok := new(big.Int).SetString(g.String(), 10)
		wi, wok := new(big.Int).SetString(w.String(), 10)
		if gok && wok {
			if gi.Cmp(wi) != 0 {
				return path + ": " + g.String() + " != " + w.String()
			}
			return ""
		}
		gf, _ := g.Float64()
		wf, _ := w.Float64()
		if math.Abs(gf-wf) > 1e-12*math.Max(1, math.Abs(wf)) {
			return path + ": " + g.String() + " != " + w.String()
		}
	default:
		if !reflect.DeepEqual(got, want) {
			return fmt.Sprintf("%s: %v != %v", path, got, want)
		}
	}
	return ""
}

// Run decodes every typed case in the fixture file into the type newOf returns.
//   - "ok" cases must decode and re-encode to exactly known_json: the example with
//     only the fields the schema defines. That catches wrong JSON names and types,
//     which plain decoding would silently ignore.
//   - "stale" cases (old examples missing newer fields) must decode.
//   - "docs-mistake" cases must fail to decode or change when re-encoded, so a fixed
//     docs example shows up as a failure until the fixtures are regenerated.
func Run(t *testing.T, path string, newOf func(typeName string) interface{}) {
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	var f fixture
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatal(err)
	}
	checked := 0
	for _, c := range f.Cases {
		if c.Type == nil || (c.Kind != "request" && c.Kind != "response") {
			continue
		}
		c := c
		t.Run(c.Name+"/"+c.Kind, func(t *testing.T) {
			v := newOf(*c.Type)
			if v == nil {
				t.Fatalf("no generated Go type for %s", *c.Type)
			}
			decodeErr := json.Unmarshal(c.JSON, v)
			var back []byte
			if decodeErr == nil {
				var err error
				if back, err = json.Marshal(v); err != nil {
					t.Fatal(err)
				}
			}
			switch c.Status {
			case "ok":
				if decodeErr != nil {
					t.Fatalf("decode into %s: %v", *c.Type, decodeErr)
				}
				want := c.KnownJSON
				if reflect.TypeOf(v).Elem().Kind() == reflect.Map {
					want = c.JSON // empty-object schema: quicktype makes a map, which keeps every key
				}
				if d := diff(normalize(t, back), normalize(t, want), ""); d != "" {
					t.Errorf("re-encoded through %s, the example differs at %s", *c.Type, d)
				}
			case "stale":
				if decodeErr != nil {
					t.Fatalf("decode into %s: %v", *c.Type, decodeErr)
				}
			case "docs-mistake":
				if decodeErr == nil && diff(normalize(t, back), normalize(t, c.JSON), "") == "" {
					t.Errorf("docs example no longer shows the mistake %v; regenerate the fixtures", c.Notes)
				}
			default:
				t.Fatalf("unknown status %q", c.Status)
			}
		})
		checked++
	}
	if checked == 0 {
		t.Fatal("no typed cases in " + path)
	}
}
