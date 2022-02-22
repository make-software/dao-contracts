import { config } from "dotenv";
config({ path: "./.env.js-client" });

import { DaoReputationJSClient,  } from "../src";
import { utils } from "casper-js-client-helper";
import { getDeploy } from "./utils";

import { Keys } from "casper-js-sdk";

const {
  CHAIN_NAME,
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  WASM_PATH,
  USER_KEY_PAIR_PATH,
  INSTALL_PAYMENT_AMOUNT,
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${USER_KEY_PAIR_PATH}/public_key.pem`,
  `${USER_KEY_PAIR_PATH}/secret_key.pem`
);

const test = async () => {
  const daoReputation = new DaoReputationJSClient(
    NODE_ADDRESS,
    CHAIN_NAME,
    EVENT_STREAM_ADDRESS
  );

  console.log(`... Try install`);

  const installDeployHash = await daoReputation.install(
    KEYS,
    INSTALL_PAYMENT_AMOUNT,
    WASM_PATH
  );

  console.log(`... Contract deploy is pending, waiting for next block finalisation (deployHash: ${installDeployHash})`);

  const deploy = await getDeploy(NODE_ADDRESS, installDeployHash);

  console.log(`... Deploy is settled which means contract is installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS, KEYS.publicKey);

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `reputation_contract_package_hash`
  );

  console.log(`... Here is your Contract Hash: ${contractHash}`);

  daoReputation.setContractHash('contractHash');
};

test();
