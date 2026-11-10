#!/usr/bin/env python3
"""Convert the component schemas of a generated OpenRPC file into a plain JSON Schema,
with every schema under "definitions" keyed by its title (GetInfoResponse, ...).

  python3 tools/to_jsonschema.py schemas/daemon.openrpc.json > daemon.schema.json

Code generators that read JSON Schema (quicktype, datamodel-code-generator, ...) use
the definition names for their types. Pointed at the OpenRPC file directly, they would
name types after the C++ structs (COMMANDRPCGETBANSRequestT).
"""
import json
import sys


def convert(openrpc):
    comps = openrpc["components"]["schemas"]
    title = {k: v["title"] for k, v in comps.items()}

    def rewrite(node):
        if isinstance(node, dict):
            node = {k: rewrite(v) for k, v in node.items()}
            if "$ref" in node:
                node["$ref"] = "#/definitions/" + title[node["$ref"].rsplit("/", 1)[-1]]
            return node
        if isinstance(node, list):
            return [rewrite(v) for v in node]
        return node

    return {"$schema": "http://json-schema.org/draft-07/schema#",
            "title": openrpc["info"]["title"],
            "definitions": {title[k]: rewrite(v) for k, v in comps.items()}}


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    json.dump(convert(json.load(open(sys.argv[1]))), sys.stdout, indent=1)
    print()
