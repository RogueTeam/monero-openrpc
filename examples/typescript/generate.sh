#!/bin/sh
# Regenerate src/monerod.ts and src/walletrpc.ts from ../../schemas with the
# official OpenRPC generator. Needs Node.js (npx downloads the generator on first use).
set -eu
cd "$(dirname "$0")"
ROOT=$(cd ../.. && pwd)
GENERATOR=${GENERATOR:-"npx --yes @open-rpc/generator@2.1.1"}
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

gen() { # gen SCHEMA MODULE
    cat > "$TMP/$2.json" <<EOF
{"openrpcDocument": "$ROOT/schemas/$1.openrpc.json", "outDir": "$TMP/$2",
 "components": [{"type": "client", "name": "$2", "language": "typescript"}]}
EOF
    (cd "$TMP" && $GENERATOR generate -c "$2.json" > /dev/null)
    cp "$TMP/$2/client/typescript/src/index.ts" "src/$2.ts"
}

gen daemon monerod
gen wallet walletrpc

# Generator bug: get_transfers has a param named "in", which it emits as a function
# parameter name, and "in" is a reserved word in TypeScript. Rename it in the type.
# (The JSON name comes from the embedded schema, so the request still says "in".)
sed -i 's/(in?: /(in_?: /' src/walletrpc.ts
