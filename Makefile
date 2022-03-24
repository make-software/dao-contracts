OUTPUT_DIR = target/wasm32-unknown-unknown/release
CARGO_BUILD = cargo build --release --target wasm32-unknown-unknown \
				--quiet --features=wasm --no-default-features
CARGO_TEST = CARGO_TARGET_DIR=$(shell pwd)/target cargo test --features=test-support --no-default-features

prepare:
	rustup target add wasm32-unknown-unknown

build-proxy-getter:
	@$(CARGO_BUILD) \
		-p casper-dao-utils \
		--bin getter_proxy
	@wasm-strip $(OUTPUT_DIR)/getter_proxy.wasm 2>/dev/null | true
	
build-dao-contracts:
	$(CARGO_BUILD) -p casper-dao-contracts
	wasm-strip $(OUTPUT_DIR)/reputation_contract.wasm 2>/dev/null | true
	wasm-strip $(OUTPUT_DIR)/variable_repository_contract.wasm 2>/dev/null | true
	wasm-strip $(OUTPUT_DIR)/erc_20.wasm 2>/dev/null | true

test-dao-contarcts: build-contracts prepare-proxy-getter
	cargo test -p tests $$TEST_NAME

build-erc20:
	@$(CARGO_BUILD) -p casper-dao-erc20 

test-erc20: build-proxy-getter build-erc20 
	$(CARGO_TEST) -p casper-dao-erc20 $$TEST_NAME --test test-erc20

clippy:
	cargo clippy --all-targets -- -D warnings -A clippy::bool-assert-comparison

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
	rm -rf tests/wasm/*.wasm
	rm -rf dao-erc20/wasm/*.wasm

docs:
	cargo doc --features test-support --no-deps --open