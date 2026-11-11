#!/bin/sh
# Regenerate src/monerod.rs, src/walletrpc.rs and tests/registry/mod.rs from ../../schemas.
# Needs Python 3, Rust (rustfmt) and Node.js (npx downloads quicktype on first use).
# Set QUICKTYPE to use a quicktype that's already installed.
set -eu
cd "$(dirname "$0")"
ROOT=../..
QUICKTYPE=${QUICKTYPE:-"npx --yes quicktype@26.0.0"}
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

gen() { # gen SCHEMA MODULE
    python3 "$ROOT/tools/to_jsonschema.py" "$ROOT/schemas/$1.openrpc.json" > "$TMP/$1.schema.json"
    $QUICKTYPE --src-lang schema --lang rust --visibility public --skip-serializing-none \
        --derive-partial-eq --no-leading-comments \
        -o "src/$2.rs" "$TMP/$1.schema.json#/definitions/"
    python3 "$ROOT/tools/fix_uint64.py" rust "$TMP/$1.schema.json" "src/$2.rs"
}

gen daemon monerod
gen wallet walletrpc
python3 registry.py src/monerod.rs src/walletrpc.rs > tests/registry/mod.rs
rustfmt --edition 2021 src/monerod.rs src/walletrpc.rs tests/registry/mod.rs
