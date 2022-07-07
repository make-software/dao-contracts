import BigNumber from "bignumber.js";
import { utils } from "casper-js-client-helper";
import { createRecipientAddress } from "casper-js-client-helper/dist/helpers/lib";
import {
  CLKey,
  CLKeyParameters,
  CLPublicKey,
  CLU256,
  CLValue,
  CLValueBuilder,
  EventName,
  EventStream,
  Keys,
} from "casper-js-sdk";
import { Option, Result } from "ts-results";

if (process.env.NODE_ENV !== "ci") {
  require("dotenv").config({ path: "./.env", debug: true });
}

import {
  createInstallReputationContractDeploy,
  ReputationContractEventParser,
  ReputationContractEvents,
  GenericContractJSClient,
} from "../src";
import { createRpcClient } from "../src/common/rpc-client";
import {
  getAccountInfo,
  getAccountNamedKeyValue,
  waitForDeploy,
  assert,
} from "./utils";

const {
  NODE_ENV,
  CHAIN_NAME,
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  WASM_RELEASE_PATH,
  NCTL_USERS_FOLDER_PATH,
  INSTALL_PAYMENT_AMOUNT,
  DEPLOY_PAYMENT_AMOUNT,
} = process.env;

console.log("testing env variables", {
  NODE_ENV,
  CHAIN_NAME,
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  WASM_RELEASE_PATH,
  NCTL_USERS_FOLDER_PATH,
  INSTALL_PAYMENT_AMOUNT,
  DEPLOY_PAYMENT_AMOUNT,
});

const ownerKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-1/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-1/secret_key.pem`
);
const recipientKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-2/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-2/secret_key.pem`
);
const test = async () => {
  /** SCHEMA */

  const reputationContractSchema = {
    entry_points: {
      mint: [
        { name: "recipient", cl_type: "Address" },
        { name: "amount", cl_type: "U256" },
      ],
      add_to_whitelist: [{ name: "address", cl_type: "Address" }],
    },
    named_keys: {
      owner: {
        named_key: "owner_owner_access_control_contract",
        cl_type: [{ name: "Option", inner: "Address" }],
      },
      total_supply: {
        named_key: "total_supply_token_token_contract",
        cl_type: "U256",
      },
      balance: {
        named_key: "balances_token_token_contract",
        cl_type: [{ name: "Mapping", key: "Address", value: "U256" }],
      },
      whitelist: {
        named_key: "whitelist_whitelist_access_control_contract",
        cl_type: [{ name: "Mapping", key: "Address", value: "Bool" }],
      },
      stakes: {
        named_key: "token_stakes",
        cl_type: [{ name: "Mapping", key: "Address", value: "U256" }],
      },
    },
  };

  const contractHashWithHashPrefix =
    "hash-10539a97a58adf60498ecd0e7be1f7284c6dcc01a09154f45f97c8fc5d395a7e";
  const contractPackageHash =
    "hash-49f5fc30a7888b74508a1ee4365353e858a4fc26ff1cdd3d7a81a7aff36d5276";

  // Initialize contract client
  const reputationContract = new GenericContractJSClient(
    NODE_ADDRESS,
    CHAIN_NAME,
    EVENT_STREAM_ADDRESS,
    contractHashWithHashPrefix,
    contractPackageHash,
    reputationContractSchema
  );

  console.log(`\n`);
  console.log(`... Testing deploys ...`);

};

test().then(() => {
  process.exit(0);
});
