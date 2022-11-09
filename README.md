# MVPR DAO for Casper

Reusable smart contracts for building DAOs on top of Casper.

Repository contains following modules:
- `dao-contracts` and `dao-modules` provides smart contracts implementation,
- `dao-erc20` and `dao-erc721` allows to use those standards,
- `dao-utils` and `dao-macros` makes writing code easier,
- `client` implements a JavaScript client for smart contracts interactions.

## Prerequisites

- `Rust` toolchain installed (see [rustup.rs](https://rustup.rs/)),
- `wasmstrip` tool installed (see [wabt](https://github.com/WebAssembly/wabt)).

Finally, prepare your environment:

```bash
$ make prepare
```

## Build contracts
Build `WASM` files.

```bash
$ make build-all
```

## Test
Run integration tests.

```bash
$ make test
```

## Docs
Generate `rustdoc`. Opens a new browser window.
```bash
$ make docs
```

## E2E Tests Prerequisites

- `docker` installed (see [Get Docker](https://docs.docker.com/get-docker/)),
- `docker-compose` installed (see [Docker Compose](https://docs.docker.com/compose/install/)),
- `nodejs` installed (see [nodejs.org](https://nodejs.org/en/download/)).

## E2E Tests
To run e2e tests execute:

```bash
$ make run-e2e-tests
```

The above command starts Docker container with running Casper network in it
(see [nctl](https://docs.casperlabs.io/dapp-dev-guide/building-dapps/setup-nctl/) for more info).
Then it executes [e2e-reputation.ts](client/e2e/e2e-reputation.ts) script agains the docker.
It shows how to deploy the `reputation.wasm` file, call `mint` and `whitelist` entrypoints,
and check the result. Full DAO installation scripts are still under development.
