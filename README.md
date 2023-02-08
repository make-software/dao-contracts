# MVPR DAO for Casper

Reusable smart contracts for building DAOs on top of Casper.

Repository contains following modules:
- `dao-contracts` and `dao-modules` provides smart contracts implementation,
- `dao-erc20` and `dao-erc721` allows to use those standards,
- `dao-utils` and `dao-macros` makes writing code easier,
- `client` implements a JavaScript client for smart contracts interactions.

## General Documentation

- [Usage](docs-high-level/usage.md)
- [Architecture](docs-high-level/architecture.md)
- [Adding a new contract](docs-high-level/adding_new_contract.md)
- [BDD Testing with Gherkin](docs-high-level/gherkin.md)

## Technical documentation
To generate `rustdoc` execute the following:
```bash
make build-docs
```

Live version: https://make-software.github.io/dao-contracts.

## Quickstart

### Prerequisites

- `Rust` toolchain installed (see [rustup.rs](https://rustup.rs/)),
- `wasmstrip` tool installed (see [wabt](https://github.com/WebAssembly/wabt)).

To prepare your environment execute:

```bash
make prepare
```

### Build contracts
To build `WASM` files execute:

```bash
make build-all
```
Contracts will be located in the `dao-contracts/wasm/` folder.

### Run tests

To run integration tests execute:

```bash
make test
```

### Run e2e tests

#### E2E Tests Prerequisites

- `docker` installed (see [Get Docker](https://docs.docker.com/get-docker/)),
- `docker-compose` installed (see [Docker Compose](https://docs.docker.com/compose/install/)),
- `nodejs` installed (see [nodejs.org](https://nodejs.org/en/download/)).

#### Tests execution
To run e2e tests execute:

```bash
make run-e2e-tests
```

The above command starts Docker container with running Casper network in it
(see [nctl](https://docs.casperlabs.io/dapp-dev-guide/building-dapps/setup-nctl/) for more info).
Then it executes [e2e-reputation.ts](client/e2e/e2e-reputation.ts) script against the docker.
It shows how to deploy the `reputation.wasm` file, call `mint` and `whitelist` entrypoints,
and check the result. Full DAO installation scripts are still under development.
