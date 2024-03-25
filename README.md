# MVPR DAO for Casper

Reusable smart contracts for building DAOs on top of Casper.

Repository contains following modules:
- `dao` provides smart contracts implementation,
- `dao-macros` makes writing code easier,

## General Documentation

- [Usage](docs-high-level/usage.md)
- [Architecture](docs-high-level/architecture.md)
- [Adding a new contract](docs-high-level/adding_new_contract.md)
- [BDD Testing with Gherkin](docs-high-level/gherkin.md)

## Technical documentation
To generate `rustdoc` execute the following:
```bash
just rebuild-docs
```

Live version: https://make-software.github.io/dao-contracts.

## Quickstart

### Prerequisites

- `Rust` toolchain installed (see [rustup.rs](https://rustup.rs/)),
- `cargo odra` installed `cargo install --version 0.0.10 --force --locked cargo-odra`
- `wasmstrip` tool installed (see [wabt](https://github.com/WebAssembly/wabt)).
- `just` tool installed (see [just](https://github.com/casey/just)).
- `wabt` installed (see [wabt](https://github.com/WebAssembly/wabt)).

To prepare your environment execute:

```bash
just prepare
```

### Build contracts
To build `WASM` files execute:

```bash
just build-all
```
Contracts will be located in the `wasm/` folder.

### Run tests

To run integration tests execute:

```bash
just test
```
