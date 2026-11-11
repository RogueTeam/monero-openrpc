#!/bin/sh
# Regenerate the pydantic models from ../../schemas.
# Needs: pip install -r requirements-dev.txt
set -eu
cd "$(dirname "$0")"
ROOT=../..
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

gen() { # gen SCHEMA MODULE
    python3 "$ROOT/tools/to_jsonschema.py" "$ROOT/schemas/$1.openrpc.json" > "$TMP/$1.schema.json"
    datamodel-codegen --input "$TMP/$1.schema.json" --input-file-type jsonschema \
        --output "$2.py" --output-model-type pydantic_v2.BaseModel \
        --target-python-version 3.10 --field-constraints --disable-timestamp \
        --formatters black isort
}

gen daemon monerod_models
gen wallet walletrpc_models
