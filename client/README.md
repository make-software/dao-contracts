# `dao-contracts-js-client`

This JavaScript client gives you an easy way to install and interact with all the DAO contracts.

## Installation

Run this command to install the client:

```bash
npm i dao-contracts-js-client
```

## Usage

### Install the contract on the network

```ts
// create deploy
const deploy = createInstallReputationContractDeploy(
  CHAIN_NAME,
  NODE_ADDRESS,
  200000000000, // Payment amount
  "../target/wasm32-unknown-unknown/release/reputation_contract.wasm" // Path to WASM file
  OWNER_KEYS, // Key pair used for signing deploy
);

// send deploy to network
const installDeployHash = await installDeploy.send(NODE_ADDRESS); 
```

### Create a client instance to interact with the contract

```ts
const reputationContract = new GenericContractJSClient(
  http://localhost:11101, // RPC address
  "casper-net-1", // Network name
  "http://localhost:18101/events/main", // Event stream address
  "hash-XXXXXXXXXXXXXXXXXXXXx", // contractPackageHash
  "hash-XXXXXXXXXXXXXXXXXXXXx", // contractHash
  'path-to-contract-yaml-schema-file'
);
```

## API

### Getters

Use getter methods to retrieve values:

```ts
const total_supply =
  await reputationContract.getNamedKey("total_supply");

const isWhitelisted = 
  await reputationContract.getNamedKey("whitelist", publicKey);
```

### Deploys

Use deploys to interact with contract:

```ts
const mintAmount = "200000000000";

const mintResult: Result<string, string> = await reputationContract.callEntryPoint(
  "mint",
  ownerKeys,
  DEPLOY_PAYMENT_AMOUNT,
  createRecipientAddress(ownerKeys.publicKey), // import { createRecipientAddress } from "casper-js-client-helper/dist/helpers/lib"; 
  CLValueBuilder.u256(mintAmount)
);

if (mintResult.ok) {
  // handle success
  mintResult.val
} else {
  // handle error
  mintResult.val
}
```

## Development

- Set the environment variables in the `.env.js-client` file. Use `.env.js-client.example` as a template
- Install dependencies: `npm i`
- Run the contract with e2e test script: `npm run e2e:reputation`

## Related Projects

You can find all the available examples in the [central project repository](https://github.com/casper-network/casper-contracts-js-clients).
