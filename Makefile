# make schemas MONERO_REPO=/path/to/monero   regenerate schemas/
# make examples                              regenerate the code in examples/ from schemas/
# make test                                  run every test (needs Go, Rust, Node.js, pydantic)
MONERO_REPO ?= ../monero
PYTHON ?= python3

.PHONY: schemas examples test test-generator test-go test-python test-rust test-typescript

schemas:
	$(PYTHON) node_schema.py $(MONERO_REPO) > schemas/daemon.openrpc.json
	$(PYTHON) wallet_schema.py $(MONERO_REPO) > schemas/wallet.openrpc.json

examples:
	examples/go/generate.sh
	examples/python/generate.sh
	examples/rust/generate.sh
	examples/typescript/generate.sh

test: test-generator test-go test-python test-rust test-typescript

test-generator:
	$(PYTHON) -m unittest discover -s tests

test-go:
	cd examples/go && go vet ./... && go test ./...

test-python:
	cd examples/python && $(PYTHON) -m unittest test_examples

test-rust:
	cd examples/rust && cargo test

test-typescript:
	cd examples/typescript && npm install --no-audit --no-fund && npm test
