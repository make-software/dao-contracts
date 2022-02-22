# `dao-reputation-js-client`

This JavaScript client gives you an easy way to install and interact with the DAO Reputation contract.

## Installation

Run this command to install the client:

```bash
npm i dao-reputation-js-client
```

## Usage

### Create an instance of the dao reputation client

```ts
const daoReputation = new DaoReputationJSClient(
  http://localhost:11101, // RPC address
  "casper-net-1", // Network name
  "http://localhost:18101/events/main" // Event stream address
);
```

### Install the contract

```ts
const installDeployHash = await daoReputation.install(
  KEYS, // Key pair used for signing
  200000000000, // Payment amount
  "../target/wasm32-unknown-unknown/release/reputation_contract.wasm" // Path to WASM file
);
```

### Set the contract hash (a unique identifier for the contract)

```ts
await daoReputation.setContractHash(
  "hash-c2402c3d88b13f14390ff46fde9c06b8590c9e45a9802f7fb8a2674ff9c1e5b1"
);
```

## API

### Getters

Use getter methods to retrieve values:

```ts
const todo = await daoReputation.todo();
```

### Deploys

Use deploys to interact with contract:

```ts
const todo = await daoReputation.todo();
```

## Development

- Set the environment variables in the `.env.js-client` file. Use `.env.js-client.example` as a template
- Install dependencies: `npm i`
- Install the contract with e2e test script: `npm run e2e:install`

## Related Projects

You can find all the available examples in the [central project repository](https://github.com/casper-network/casper-contracts-js-clients).
