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
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  console.log(`... Try install`);

  const installDeployHash = await daoReputation.install(
    KEYS,
    INSTALL_PAYMENT_AMOUNT!,
    WASM_PATH!
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  if (installDeployHash == null) {
    return;
  }

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));
};

test();
