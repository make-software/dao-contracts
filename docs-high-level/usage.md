# Usage

## Prerequisites

To start using the DAO contracts you need to have:
- `Rust` toolchain installed (see [rustup.rs](https://rustup.rs/)),
- `wasmstrip` tool installed (see [wabt](https://github.com/WebAssembly/wabt)).
- `cargo odra` installed `cargo install cargo-odra`
- `just` tool installed (see [just](https://github.com/casey/just)).

- The `wasmstrip` tool is used to reduce the size of the compiled contracts.
It is not required, but it is recommended if you want to deploy the contracts
to the Casper network.

As the contracts are compiled to `WASM` files,
you need to have `wasm32-unknown-unknown` target installed. To install it execute:

```bash
rustup target add wasm32-unknown-unknown
```

or use the recipe in the `Justfile`:

```bash
just prepare
```

## Building contracts
To build the contracts located in the `dao-contracts` folder, execute `cargo odra build`:

```bash
cargo odra build -b casper
```
or use the recipe in the `Makefile`:

```bash
just build-dao-contracts
```
It will also run wasm-strip on the compiled wasm files.

## Running tests

We can run the tests for the contracts:

```bash
cargo odra test -b casper
```

Or use the MockVM for faster tests:
```bash
cargo odra test
```

Of course, all of the above can be done with the `Jusfile`:

```bash
just test
```
You can also pass additional information, to filter the tests:

```bash
cargo odra test -- --test test_bid_escrow
```
