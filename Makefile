prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release --target wasm32-unknown-unknown -p reputation-contract
	wasm-strip target/wasm32-unknown-unknown/release/reputation_contract.wasm 2>/dev/null | true

test: build-contract
	cp target/wasm32-unknown-unknown/release/reputation_contract.wasm tests/wasm
	cargo test -p tests

clippy:
	cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
	rm -rf tests/wasm/*.wasm
