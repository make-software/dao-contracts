prepare:
	rustup target add wasm32-unknown-unknown

prepare-proxy-getter:
	cargo build --release --target wasm32-unknown-unknown -p casper-dao-utils --bin getter_proxy
	wasm-strip target/wasm32-unknown-unknown/release/getter_proxy.wasm 2>/dev/null | true
	cp target/wasm32-unknown-unknown/release/getter_proxy.wasm tests/wasm
	
build-contracts:
	cargo build --release --target wasm32-unknown-unknown -p casper-dao-contracts
	cargo build --release --target wasm32-unknown-unknown -p erc20 --features=as-wasm --no-default-features
	wasm-strip target/wasm32-unknown-unknown/release/reputation_contract.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/variable_repository_contract.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/erc_20.wasm 2>/dev/null | true

test: build-contracts prepare-proxy-getter
	cp target/wasm32-unknown-unknown/release/getter_proxy.wasm tests/wasm
	cp target/wasm32-unknown-unknown/release/reputation_contract.wasm tests/wasm
	cp target/wasm32-unknown-unknown/release/variable_repository_contract.wasm tests/wasm
	cp target/wasm32-unknown-unknown/release/erc_20.wasm erc20/wasm
	cargo test -p tests $$TEST_NAME
	
build-erc20:
	cargo build --release --target wasm32-unknown-unknown -p casper-dao-erc20

test-erc20: prepare-proxy-getter
	cp target/wasm32-unknown-unknown/release/getter_proxy.wasm erc20/wasm
	cp target/wasm32-unknown-unknown/release/erc_20.wasm dao-erc20/wasm
	cargo test -p casper-dao-erc20 $$TEST_NAME --features=test-support --no-default-features

clippy:
	cargo clippy --all-targets -- -D warnings -A clippy::bool-assert-comparison

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
	rm -rf tests/wasm/*.wasm

docs:
	cargo doc --features test-support --no-deps --open