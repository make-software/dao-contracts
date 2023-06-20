prepare:
    rustup target add wasm32-unknown-unknown
    sudo apt install wabt
    cargo install cargo-odra --locked

build-dao-contracts:
    cargo odra build -b casper

test-dao-contracts: build-dao-contracts
    cargo odra test -b casper -s

test:
    cargo odra test
    cargo odra test -b casper

clippy:
	cargo clippy --all-targets -- -D warnings -A clippy::bool-assert-comparison
	cd dao-client && cargo clippy --all-targets -- -D warnings -A clippy::bool-assert-comparison

check-lint: clippy
	cargo fmt -- --check
	cd dao-client && cargo fmt -- --check

lint: clippy
	cargo fmt
	cd dao-client && cargo fmt

clean:
    cargo odra clean

rebuild-docs:
	rm -rf docs
	cargo doc --workspace --lib --no-deps
	cp -r target/doc docs
	echo "<meta http-equiv=\"refresh\" content=\"0; url=dao\">" > docs/index.html

test-admin:
	cargo odra test -b casper -s -- --test test_admin

test-bid-escrow: build-dao-contracts
	cargo odra test -b casper -- --test test_bid_escrow

test-slashing: build-dao-contracts
	cargo odra test -b casper -- --test test_slashing

test-variables: build-dao-contracts
	cargo odra test -b casper -- --test test_variables

test-kyc: build-dao-contracts
	cargo odra test -b casper -- --test test_kyc

test-ownership: build-dao-contracts
	cargo odra test -b casper -- --test test_ownership

test-va: build-dao-contracts
	cargo odra test -b casper -- --test test_va

test-voting: build-dao-contracts
	cargo odra test -b casper -- --test test_voting

test-rate-provider: build-dao-contracts
	cargo odra test -b casper -- --test test_rate_provider

