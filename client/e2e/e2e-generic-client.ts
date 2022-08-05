import BigNumber from "bignumber.js";
import { utils } from "casper-js-client-helper";
import { createRecipientAddress } from "casper-js-client-helper/dist/helpers/lib";
import {
  CLKey,
  CLU256,
  CLValue,
  CLValueBuilder,
  EventName,
  EventStream,
  Keys,
} from "casper-js-sdk";
import { Deploy } from "casper-js-sdk/dist/lib/DeployUtil";
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

const wasmContractPath = WASM_RELEASE_PATH + "/reputation_contract.wasm";
const wasmContractSchemaPath =
  WASM_RELEASE_PATH + "/reputation_contract_schema.yaml";

const test = async () => {
  console.log(`\n`);
  console.log(`... Testing install ...`);
  console.log(`\n`);

  const installDeploy = createInstallReputationContractDeploy(
    CHAIN_NAME,
    NODE_ADDRESS,
    INSTALL_PAYMENT_AMOUNT,
    wasmContractPath,
    ownerKeys
  );

  const installDeployHash = await installDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, installDeployHash);
  let accountInfo = await getAccountInfo(NODE_ADDRESS, ownerKeys.publicKey);
  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `reputation_contract_package_hash`
  );
  if (!contractPackageHash) {
    throw Error("Contract not installed correctly!");
  }

  console.log(`... Contract installed successfully.`);
  console.log(` - Contract Package Hash: ${contractPackageHash}`);

  const stateRootHash = await utils.getStateRootHash(NODE_ADDRESS);
  const rpcClient = createRpcClient(NODE_ADDRESS);
  const res = await rpcClient.fetchStateGetItem(
    stateRootHash,
    contractPackageHash,
    []
  );
  const contractHash = res.ContractPackage.versions[0].contract_hash;
  const contractHashWithHashPrefix = contractHash.replace("contract-", "hash-");
  console.log(` - Contract Hash: ${contractHashWithHashPrefix}`);

  // Initialize contract client
  const reputationContractClient = new GenericContractJSClient(
    NODE_ADDRESS,
    CHAIN_NAME,
    EVENT_STREAM_ADDRESS,
    contractHashWithHashPrefix,
    contractPackageHash,
    wasmContractSchemaPath
  );

  console.log(`\n`);
  console.log(`... Testing deploys ...`);

  /** MINT DEPLOY */

  console.log(`\n`);
  console.log(" - Trying to create and send a mint deploy");

  const mintAmount = "200000000000";

  const mintDeployResult = await reputationContractClient.createDeploy(
    "mint",
    ownerKeys.publicKey,
    DEPLOY_PAYMENT_AMOUNT,
    createRecipientAddress(ownerKeys.publicKey),
    CLValueBuilder.u256(mintAmount)
  );

  if (mintDeployResult.ok) {
    const deploy = mintDeployResult.val;

    // 1) Signing with NodeJS
    deploy.sign([ownerKeys]);

    // 2) Signing with Signer Extension
    // const deployJson = DeployUtil.deployToJson(deploy);
    // Signer.sign(deployJson, senderPublicKey).then(signedDeployJson => /* send deploy */);

    const deployHash = await deploy.send(NODE_ADDRESS);
    await waitForDeploy(NODE_ADDRESS, deployHash);

    const total_supply_after_mint = (await reputationContractClient.getNamedKey(
      "total_supply"
    )) as BigNumber; // should be CLU256 but sdk returns BigNumber, need to fix in casper-js-sdk
    console.log(` - Requested Mint Amount: `, mintAmount);
    console.log(
      ` - Total Supply After Mint: `,
      total_supply_after_mint.toString()
    );
    const totalSupplyEqMint = total_supply_after_mint.eq(mintAmount);

    console.log(
      ` - Total supply equals mint: ${
        totalSupplyEqMint ? "SUCCESS!" : "FAILED!"
      }`
    );
    assert(totalSupplyEqMint);
  } else {
    console.error(mintDeployResult.val);
    process.exit(1);
  }

  /** WHITELIST DEPLOY */

  console.log(`\n`);
  console.log(" - Trying to create and send a add to whitelist deploy");

  const isWhitelistedBefore = (await reputationContractClient.getNamedKey(
    "whitelist",
    recipientKeys.publicKey
  )) as Boolean;
  console.log(` - Whitelist Value Before Deploy: ${isWhitelistedBefore}`);

  const whitelistDeployResult = await reputationContractClient.createDeploy(
    "add_to_whitelist",
    ownerKeys.publicKey,
    DEPLOY_PAYMENT_AMOUNT,
    createRecipientAddress(recipientKeys.publicKey)
  );
  if (whitelistDeployResult.ok) {
    const deploy = whitelistDeployResult.val;
    deploy.sign([ownerKeys]);
    const deployHash = await deploy.send(NODE_ADDRESS);
    await waitForDeploy(NODE_ADDRESS, deployHash);

    const isWhitelisted = await reputationContractClient.getNamedKey(
      "whitelist",
      recipientKeys.publicKey
    );
    console.log(` - Whitelist Value After Deploy: ${isWhitelisted}`);

    const recipientAddedToTheWhitelist = "true" === isWhitelisted;

    console.log(
      ` - Recipient is added to the whitelist: ${
        recipientAddedToTheWhitelist ? "SUCCESS!" : "FAILED!"
      }`
    );
    assert(recipientAddedToTheWhitelist);
  } else {
    console.error(whitelistDeployResult.val);
    process.exit(1);
  }
};

test().then(() => {
  process.exit(0);
});
