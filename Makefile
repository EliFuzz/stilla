MAKEFILE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))

CARGO ?= cargo

.PHONY: build
build: clean
	$(CARGO) build $(CARGO_WORKSPACE_ARGS)

.PHONY: lint
lint:
	$(CARGO) fmt --all
	$(CARGO) clippy --fix $(CARGO_WORKSPACE_ARGS) --all-targets --allow-dirty --allow-staged -- -D warnings

.PHONY: clean
clean:
	$(CARGO) clean
