BINARYEN_VERSION := "version_116"
BINARYEN_CHECKSUM := "c55b74f3109cdae97490faf089b0286d3bba926bb6ea5ed00c8c784fc53718fd"

prepare:
    rustup target add wasm32-unknown-unknown
    cargo install --version 0.0.9-fixed --force --locked cargo-odra
    wget https://github.com/WebAssembly/binaryen/releases/download/{{BINARYEN_VERSION}}/binaryen-{{BINARYEN_VERSION}}-x86_64-linux.tar.gz || { echo "Download failed"; exit 1; }
    sha256sum binaryen-{{BINARYEN_VERSION}}-x86_64-linux.tar.gz | grep {{BINARYEN_CHECKSUM}} || { echo "Checksum verification failed"; exit 1; }
    tar -xzf binaryen-{{BINARYEN_VERSION}}-x86_64-linux.tar.gz || { echo "Extraction failed"; exit 1; }
    sudo cp binaryen-{{BINARYEN_VERSION}}/bin/wasm-opt /usr/local/bin/wasm-opt

build-dao-contracts:
    cargo odra build -b casper
    for file in wasm/*.wasm; do \
        echo "Processing $file"; \
        wasm-opt "$file" --signext-lowering -o "$file"; \
    done

test: build-dao-contracts
    cargo odra test
    cargo odra test -b casper -s

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
	cargo odra test -b casper -s -- --test test_bid_escrow

test-slashing: build-dao-contracts
	cargo odra test -b casper -s -- --test test_slashing

test-variables: build-dao-contracts
	cargo odra test -b casper -s -- --test test_variables

test-kyc: build-dao-contracts
	cargo odra test -b casper -s -- --test test_kyc

test-ownership: build-dao-contracts
	cargo odra test -b casper -s -- --test test_ownership

test-va: build-dao-contracts
	cargo odra test -b casper -s -- --test test_va

test-voting: build-dao-contracts
	cargo odra test -b casper -s -- --test test_voting

test-rate-provider: build-dao-contracts
	cargo odra test -b casper -s -- --test test_rate_provider

