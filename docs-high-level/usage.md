# Usage

## Prerequisites

To start using the DAO contracts you need to have:
- `Rust` toolchain installed (see [rustup.rs](https://rustup.rs/)),
- `wasmstrip` tool installed (see [wabt](https://github.com/WebAssembly/wabt)).

The `wasmstrip` tool is used to reduce the size of the compiled contracts.
It is not required, but it is recommended if you want to deploy the contracts
to the Casper network.

As the contracts are compiled to `WASM` files,
you need to have `wasm32-unknown-unknown` target installed. To install it execute:

```bash
rustup target add wasm32-unknown-unknown
```
or use the recipe in the `Makefile`:

```bash
make prepare
```

## Building contracts
To build the contracts located in the `dao-contracts` folder, execute cargo build:

```bash
cargo build --release --target wasm32-unknown-unknown --quiet --features=wasm --no-default-features -p casper-dao-utils --bin getter_proxy
```
or use the recipe in the `Makefile`:

```bash
make build-dao-contracts
```
It will also run wasm-strip on the compiled wasm files.

## Running tests

The tests are run on the local instance of the CasperVM. To be able to communicate with deployed
contracts, we need an additional one, called `getter_proxy`, located in the `casper-dao-utils` module.
To build it, execute:

```bash
cargo build --release --target wasm32-unknown-unknown --quiet --features=wasm --no-default-features -p casper-dao-utils --bin getter_proxy
```
or use the recipe in the `Makefile`:

```bash
make build-getter-proxy
```

Next, we need to copy wasm files to the `wasm` folder, so that the tests can find them:

```bash
cp target/wasm32-unknown-unknown/release/*.wasm dao-contracts/wasm/
```

With `getter_proxy` built and wasm files in the proper directory, we can run the tests for the contract:

```bash
cargo test --features=test-support --no-default-features --release -p casper-dao-contracts --tests
```

Of course, all of the above can be done with the `Makefile`:

```bash
make test
```
You can also pass additional information as environment variable, to filter the tests:

```bash
TEST_NAME=bid_escrow make test
```

## Deploying contracts to the network

[//]: # (TODO)