ROOT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: format
format:
	forge fmt --root $(ROOT_DIR)

.PHONY: format_check
format_check:
	forge fmt --check --root $(ROOT_DIR)

.PHONY: test
test:
	forge test --root $(ROOT_DIR)

setup:
	node ./script/circom-verifier-setup.js
