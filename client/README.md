# `dao-contracts-js-client`

This JavaScript client gives you an easy way to install and interact with the DAO Reputation contract.

## Installation

Run this command to install the client:

```bash
npm i dao-contracts-js-client
```

## Usage

### Install the contract on the network

```ts
const installDeployHash = await installReputationContract(
  CHAIN_NAME,
  NODE_ADDRESS,
  KEYS, // Key pair used for signing
  200000000000, // Payment amount
  "../target/wasm32-unknown-unknown/release/reputation_contract.wasm" // Path to WASM file
);
```

### Create an instance to interact with the contract

```ts
const reputationContract = new ReputationContractJSClient(
  http://localhost:11101, // RPC address
  "casper-net-1", // Network name
  "http://localhost:18101/events/main", // Event stream address
  "hash-XXXXXXXXXXXXXXXXXXXXx", // contractPackageHash
  "hash-XXXXXXXXXXXXXXXXXXXXx", // contractHash
);
```

## API

### Getters

Use getter methods to retrieve values:

```ts
const owner = await reputationContract.getOwner();
const total_supply = await reputationContract.getTotalSupply();
```

### Deploys

Use deploys to interact with contract:

```ts
const mintAmount = "200000000000";

const deployHashMint = await reputationContract.mint(
  ownerKeys,
  ownerKeys.publicKey,
  mintAmount,
  DEPLOY_PAYMENT_AMOUNT
);
```

## Development

- Set the environment variables in the `.env.js-client` file. Use `.env.js-client.example` as a template
- Install dependencies: `npm i`
- Run the contract with e2e test script: `npm run e2e:reputation`

## Related Projects

You can find all the available examples in the [central project repository](https://github.com/casper-network/casper-contracts-js-clients).
