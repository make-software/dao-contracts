# MVPR DAO for Casper

Reusable smart contracts for building DAOs on top of Casper.

Repository contains following modules:
- `dao-contracts` and `dao-modules` provides smart contracts implementation,
- `dao-erc20` and `dao-erc721` allows to use those standards,
- `dao-utils` and `dao-macros` makes writing code easier,
- `client` implements a JavaScript client for smart contracts interactions.

## Build contracts
Build `WASM` files.

```bash
$ make build-contracts
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
