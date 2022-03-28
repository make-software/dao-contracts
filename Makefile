OUTPUT_DIR = target/wasm32-unknown-unknown/release
CARGO_BUILD = cargo build --release --target wasm32-unknown-unknown --quiet --features=wasm --no-default-features
CARGO_TEST = cargo test --features=test-support --no-default-features

prepare:
	rustup target add wasm32-unknown-unknown

build-proxy-getter:
	$(CARGO_BUILD) -p casper-dao-utils --bin getter_proxy
	@wasm-strip $(OUTPUT_DIR)/getter_proxy.wasm 2>/dev/null | true
	
build-dao-contracts:
	$(CARGO_BUILD) -p casper-dao-contracts
	@wasm-strip $(OUTPUT_DIR)/reputation_contract.wasm 2>/dev/null | true
	@wasm-strip $(OUTPUT_DIR)/variable_repository_contract.wasm 2>/dev/null | true
	@wasm-strip $(OUTPUT_DIR)/erc_20.wasm 2>/dev/null | true

build-erc20:
	$(CARGO_BUILD) -p casper-dao-erc20

test-dao-contracts: build-proxy-getter build-dao-contracts
	mkdir -p dao-contracts/wasm
	cp $(OUTPUT_DIR)/*.wasm dao-contracts/wasm
	$(CARGO_TEST) -p casper-dao-contracts $$TEST_NAME --tests

test-erc20: build-proxy-getter build-erc20 
	mkdir -p dao-erc20/wasm
	cp $(OUTPUT_DIR)/*.wasm dao-erc20/wasm
	$(CARGO_TEST) -p casper-dao-erc20 $$TEST_NAME --tests

test-dao-macros:
	cargo test -p casper-dao-macros

build-all: build-dao-contracts build-erc20

test: build-all test-dao-macros test-dao-contracts test-erc20

clippy:
	cargo clippy --all-targets -- -D warnings -A clippy::bool-assert-comparison

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
	
docs:
	cargo doc --features test-support --no-deps --open
