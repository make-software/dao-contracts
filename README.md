# MVPR DAO for Casper

Reusable smart contracts for building DAOs on top of Casper.

Repository contains following modules:
- `contract` provides smart contracts implementation,
- `utils` and `macros` makes writing code easier,
- `tests` contain integration tests,
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
