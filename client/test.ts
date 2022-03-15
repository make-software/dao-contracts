import { config } from "dotenv";
config({ path: "./.env.js-client" });
const {
  CHAIN_NAME,
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  NCTL_USERS_FOLDER_PATH,
  INSTALL_PAYMENT_AMOUNT,
  DEPLOY_PAYMENT_AMOUNT,
} = process.env;
import { Ok, Err, Option, Some, None } from 'ts-results';

import { utils } from "casper-js-client-helper";
import {
  encodeAccountHashStrAsKey,
  getAccountInfo,
  getAccountNamedKeyValue,
  getDeploy,
  waitForDeploy,
} from "./e2e/utils";
import { CasperClient, Keys } from "casper-js-sdk";
import { ReputationContractEvents, ReputationContractJSClient } from "./src";
import { createRpcClient } from "./src/common/rpc-client";
import { decodeBase64, encodeBase64 } from "tweetnacl-ts";

const ownerKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-1/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-1/secret_key.pem`
);
const recipientKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-2/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-2/secret_key.pem`
);

const casperClient = new CasperClient(NODE_ADDRESS).nodeClient;
const rpcClient = createRpcClient(NODE_ADDRESS);

const run = async () => {
  const stateRootHash = await utils.getStateRootHash(NODE_ADDRESS);
  let accountInfo = await getAccountInfo(NODE_ADDRESS!, ownerKeys.publicKey);
  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `reputation_contract_package_hash`
  );
  const res = await rpcClient.fetchStateGetItem(
    stateRootHash,
    contractPackageHash,
    []
  );

  const ch = res.ContractPackage.versions[0].contract_hash;
  const contractHash = ch.replace("contract-", "hash-");

  const reputationContract = new ReputationContractJSClient(
    NODE_ADDRESS,
    CHAIN_NAME,
    contractHash,
    contractPackageHash,
    EVENT_STREAM_ADDRESS
  );
  const mintAmount = "200000000000";

  const ownerChangedToRecipient =
  Buffer.from(recipientKeys.publicKey.toAccountHash()).toString('hex') === Buffer.from((await reputationContract.getOwner())['val'].data.data).toString('hex');

  console.log(
    ` - Owner changed to recipient: ${
      ownerChangedToRecipient ? "Success" : "Failed"
    }`,
    '\n'
  );

  return;
};

run();
