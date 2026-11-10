#!/bin/sh
# Regenerate the Go types in monerod/ and walletrpc/ from ../../schemas.
# Needs Python 3, Go (gofmt) and Node.js (npx downloads quicktype on first use).
# Set QUICKTYPE to use a quicktype that's already installed.
set -eu
cd "$(dirname "$0")"
ROOT=../..
QUICKTYPE=${QUICKTYPE:-"npx --yes quicktype@26.0.0"}
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

gen() { # gen SCHEMA PACKAGE
    python3 "$ROOT/tools/to_jsonschema.py" "$ROOT/schemas/$1.openrpc.json" > "$TMP/$1.schema.json"
    $QUICKTYPE --src-lang schema --lang go --package "$2" --just-types-and-package \
        -o "$2/types.go" "$TMP/$1.schema.json#/definitions/"
    gofmt -w "$2/types.go"
}

gen daemon monerod
gen wallet walletrpc
