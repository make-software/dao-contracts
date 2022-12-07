# `dao-contracts-js-client`

This JavaScript client gives you an easy way to install and interact with all the DAO contracts.

## Installation

Run this command to install the client:

```bash
npm i dao-contracts-js-client
```

## Usage

### Prepare env variables

```ts
// prepare chain name
const CHAIN_NAME = "casper-net-1";

// prepare node and event stream addresses
const NODE_ADDRESS = "http://localhost:11101";
const EVENT_STREAM_ADDRESS = "http://localhost:18101/events/main";

// Optional: prepare keys (only for NodeJS env)
const ownerKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-1/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-1/secret_key.pem`
);
```

### Install the contract on the network

```ts
// create install deploy
const installDeploy = createInstallReputationContractDeploy(
  CHAIN_NAME,
  NODE_ADDRESS,
  200000000000, // Payment amount
  "../target/wasm32-unknown-unknown/release/reputation_contract.wasm" // Path to WASM file
  ownerKeys, // Optional key pair (used for signing deploy in NodeJS env)
);

// Alternatively can also sign deploy with the Signer extension in the browser
const deployJson = DeployUtil.deployToJson(installDeploy);
Signer.sign(deployJson, senderPublicKey).then(signedDeployJson => {/* send deploy */});

// send deploy to network
const installDeployHash = await installDeploy.send(NODE_ADDRESS);

// wait for deploy
await waitForDeploy(NODE_ADDRESS, installDeployHash);
```

### Create a client instance to interact with the contract

```ts
// prepare the contractHash and contractPackageHash of your contract by reading from the network state
const contractHashWithHashPrefix =
  "hash-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
const contractPackageHash =
  "hash-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

// prepare path to the wasm contract schema file
const wasmContractSchemaPath =
  WASM_RELEASE_PATH + "/reputation_contract_schema.yaml";

// create a client instance
const reputationContractClient = new GenericContractJSClient(
  NODE_ADDRESS,
  CHAIN_NAME,
  EVENT_STREAM_ADDRESS, // Event stream address
  contractHashWithHashPrefix,
  contractPackageHash,
  wasmContractSchemaPath
);
```

## API

### Getters

Use getter methods to retrieve values:

```ts
const totalSupply = await reputationContract.getNamedKey("total_supply");

const isWhitelisted = await reputationContract.getNamedKey(
  "whitelist",
  publicKey
);
```

### Deploys

Use deploys to interact with contract:

```ts
const mintAmount = "200000000000";

// create a deploy using the client
const mintDeployResult = await reputationContract.createDeploy(
  "mint",
  ownerKeys.publicKey,
  DEPLOY_PAYMENT_AMOUNT,
  createRecipientAddress(ownerKeys.publicKey), // import { createRecipientAddress } from "casper-js-client-helper/dist/helpers/lib";
  CLValueBuilder.u512(mintAmount)
);

// check the result
if (mintDeployResult.ok) {
  // handle success
  const deploy = mintDeployResult.val;

  // Sign the deploy and send it to the network

  // - Using NodeJS
  deploy.sign([ownerKeys]);
  const deployHash = await deploy.send(NODE_ADDRESS);

  // - Using a Signer Extension in the browser
  const deployJson = DeployUtil.deployToJson(deploy);
  Signer.sign(deployJson, senderPublicKey).then(signedDeployJson => {/* send deploy */});

  // wait for deploy
  await waitForDeploy(NODE_ADDRESS, deployHash);  
} else {
  // handle error
  console.error(mintDeployResult.val);
}
```

## Development

- Set up a local Casper Network using the dockerized NCTL (use instructions in the README): <https://github.com/make-software/casper-nctl-docker>
- Start the local Casper Network by using a docker command: `docker run --rm -it --name mynctl -d -p 11101:11101 -p 14101:14101 -p 18101:18101 makesoftware/casper-nctl`
- Set the environment variables in the `.env.js-client` file (Use `.env.js-client.example` as a template)
- Install dependencies: `npm i`
- Run the contract with e2e test script: `npm run e2e:reputation`

## Related Projects

You can find all the available examples in the [central project repository](https://github.com/casper-network/casper-contracts-js-clients).
