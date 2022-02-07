prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cargo build --release --target wasm32-unknown-unknown -p casper-dao-contracts
	wasm-strip target/wasm32-unknown-unknown/release/reputation_contract.wasm 2>/dev/null | true
	cargo build --release --target wasm32-unknown-unknown -p variable-repository
	wasm-strip target/wasm32-unknown-unknown/release/variable_repository.wasm 2>/dev/null | true

test: build-contracts
	cp target/wasm32-unknown-unknown/release/reputation_contract.wasm tests/wasm
	cp target/wasm32-unknown-unknown/release/variable_repository.wasm tests/wasm
	cargo test -p tests -- --nocapture
clippy:
	cargo clippy --all-targets -- -D warnings -A clippy::bool-assert-comparison

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
	rm -rf tests/wasm/*.wasm
