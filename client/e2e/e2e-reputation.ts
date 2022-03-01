import { utils } from "casper-js-client-helper";
import { EventName, EventStream, Keys } from "casper-js-sdk";
import { config } from "dotenv";

import {
  createInstallReputationContractDeploy,
  ReputationContractEventParser,
  ReputationContractEvents,
  ReputationContractJSClient,
} from "../src";
import { createRpcClient } from "../src/common/rpc-client";
import {
  getAccountInfo,
  getAccountNamedKeyValue,
  waitForDeploy,
} from "./utils";

config({ path: "./.env.js-client" });

const {
  CHAIN_NAME,
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  WASM_PATH,
  NCTL_USERS_FOLDER_PATH,
  INSTALL_PAYMENT_AMOUNT,
  DEPLOY_PAYMENT_AMOUNT,
} = process.env;

const ownerKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-1/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-1/secret_key.pem`
);
const recipientKeys = Keys.Ed25519.parseKeyFiles(
  `${NCTL_USERS_FOLDER_PATH}/user-2/public_key.pem`,
  `${NCTL_USERS_FOLDER_PATH}/user-2/secret_key.pem`
);
const test = async () => {
  console.log(`... Try install ...`);

  const installDeploy = createInstallReputationContractDeploy(
    CHAIN_NAME,
    NODE_ADDRESS,
    INSTALL_PAYMENT_AMOUNT,
    WASM_PATH,
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

  /** Register to event stream */

  const es = new EventStream(EVENT_STREAM_ADDRESS!);
  es.subscribe(EventName.DeployProcessed, (event) => {
    const parsedEvents = ReputationContractEventParser(
      {
        contractPackageHash,
        eventNames: [
          ReputationContractEvents.AddedToWhitelist,
          ReputationContractEvents.Burn,
          ReputationContractEvents.Transfer,
          ReputationContractEvents.Mint,
          ReputationContractEvents.RemovedFromWhitelist,
          ReputationContractEvents.OwnerChanged,
        ],
      },
      event
    );

    if (parsedEvents && parsedEvents.success) {
      console.log("*** EVENT START ***");
      console.log(parsedEvents.data);
      console.log("*** EVENT END ***");
    }
  });
  es.start();
  console.log(
    ` - Registered JS event parser to the event stream: ${contractPackageHash}`
  );

  const stateRootHash = await utils.getStateRootHash(NODE_ADDRESS);
  const rpcClient = createRpcClient(NODE_ADDRESS);
  const res = await rpcClient.fetchStateGetItem(
    stateRootHash,
    contractPackageHash,
    []
  );

  const contractHash = res.ContractPackage.versions[0].contract_hash;
  const contractHashWithHashPrefix = contractHash.replace("contract-", "hash-");

  // Initialize contract client
  const reputationContract = new ReputationContractJSClient(
    NODE_ADDRESS,
    CHAIN_NAME,
    contractHashWithHashPrefix,
    contractPackageHash,
    EVENT_STREAM_ADDRESS
  );

  console.log(`\n`);
  console.log(`... Testing named keys getters ...`);

  const owner = await reputationContract.getOwner();
  console.log(` - Owner: ${owner}`);

  const total_supply = await reputationContract.getTotalSupply();
  console.log(` - Total Supply: ${total_supply}`);

  const balances = await reputationContract.getBalanceOf(ownerKeys.publicKey);
  console.log(` - Balances: ${balances}`);

  const stakes = await reputationContract.getStakeOf(ownerKeys.publicKey);
  console.log(` - Stakes: ${stakes}`);

  const whitelist = await reputationContract.getWhitelistOf(
    ownerKeys.publicKey
  );
  console.log(` - Whitelist: ${whitelist}`);

  console.log(`\n`);
  console.log(`... Testing deploys ...`);

  /** MINT DEPLOY */

  console.log(`\n`);
  console.log(" - mint deploy");

  const mintAmount = "200000000000";
  const mintDeploy = reputationContract.createDeployMint(
    ownerKeys.publicKey,
    mintAmount,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const mintDeployHash = await mintDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, mintDeployHash);
  const totalSupplyEqMint =
    mintAmount === (await reputationContract.getTotalSupply()).toString();

  console.log(
    ` - Total supply equals mint: ${totalSupplyEqMint ? "SUCCESS!" : "FAILED!"}`
  );

  /** BURN DEPLOY */

  console.log(`\n`);
  console.log(" - burn deploy");

  const burnAmount = "100000000000";
  const burnDeploy = reputationContract.createDeployBurn(
    ownerKeys.publicKey,
    burnAmount,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const burnDeployHash = await burnDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, burnDeployHash);
  const totalSupplyEqMintSubtractedByBurn =
    (Number(mintAmount) - Number(burnAmount)).toString() ===
    (await reputationContract.getTotalSupply()).toString();

  console.log(
    ` - Total supply equals mint subtracted by burn: ${
      totalSupplyEqMintSubtractedByBurn ? "SUCCESS!" : "FAILED!"
    }`
  );

  /** TRANSFER_FROM DEPLOY */

  console.log(`\n`);
  console.log(" - transfer_from deploy");

  const transferAmount = "10000000";
  const transferDeploy = reputationContract.createDeployTransferFrom(
    ownerKeys.publicKey,
    recipientKeys.publicKey,
    transferAmount,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const transferDeployHash = await transferDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, transferDeployHash);
  // `balance` named-key returns a simple string type while `total_supply` named-key returns a Big number representation,
  // both of them conceptually represent a token amount. I would like to suggest to have
  // balance return the same representation as total so they are consistent, would be easier to consume by users
  const recipientBalanceEqTransferAmount =
    transferAmount ===
    (await reputationContract.getBalanceOf(recipientKeys.publicKey));

  console.log(
    ` - Recipient balance received transfer: ${
      recipientBalanceEqTransferAmount ? "SUCCESS!" : "FAILED!"
    }`
  );

  /** ADD_TO_WHITELIST DEPLOY */

  console.log(`\n`);
  console.log(" - add_to_whitelist deploy");

  const whitelistAddDeploy = reputationContract.createDeployAddToWhitelist(
    recipientKeys.publicKey,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const whitelistAddDeployHash = await whitelistAddDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, whitelistAddDeployHash);
  const recipientAddedToTheWhitelist =
    "true" ===
    (await reputationContract.getWhitelistOf(recipientKeys.publicKey));

  console.log(
    ` - Recipient is added to the whitelist: ${
      recipientAddedToTheWhitelist ? "SUCCESS!" : "FAILED!"
    }`
  );

  /** REMOVE_FROM_WHITELIST DEPLOY */

  console.log(`\n`);
  console.log(" - remove_from_whitelist deploy");

  const whitelistRemoveDeploy =
    reputationContract.createDeployRemoveFromWhitelist(
      recipientKeys.publicKey,
      DEPLOY_PAYMENT_AMOUNT,
      ownerKeys
    );
  const whitelistRemoveDeployHash = await whitelistRemoveDeploy.send(
    NODE_ADDRESS
  );
  await waitForDeploy(NODE_ADDRESS, whitelistRemoveDeployHash);
  const recipientRemovedFromTheWhitelist =
    "false" ===
    (await reputationContract.getWhitelistOf(recipientKeys.publicKey));

  console.log(
    ` - Recipient is removed from the whitelist: ${
      recipientRemovedFromTheWhitelist ? "SUCCESS!" : "FAILED!"
    }`
  );

  /** STAKE DEPLOY */

  console.log(`\n`);
  console.log(" - stake deploy");

  const stakeAmount = "10000000";
  const stakeDeploy = reputationContract.createDeployStake(
    ownerKeys.publicKey,
    stakeAmount,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const stakeDeployHash = await stakeDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, stakeDeployHash);
  const stakeAmountWasStaked =
    stakeAmount === (await reputationContract.getStakeOf(ownerKeys.publicKey));

  console.log(
    ` - Requested amount was staked: ${
      stakeAmountWasStaked ? "SUCCESS!" : "FAILED!"
    }`
  );

  /** UNSTAKE DEPLOY */

  console.log(`\n`);
  console.log(" - unstake deploy");

  const unstakeDeploy = reputationContract.createDeployUnstake(
    ownerKeys.publicKey,
    stakeAmount,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const unstakeDeployHash = await unstakeDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, unstakeDeployHash);
  const stakeAmountWasUnstaked =
    "0" === (await reputationContract.getStakeOf(ownerKeys.publicKey));

  console.log(
    ` - Requested amount was unstaked: ${
      stakeAmountWasUnstaked ? "SUCCESS!" : "FAILED!"
    }`
  );

  /** CHANGE_OWNERSHIP DEPLOY */

  console.log(`\n`);
  console.log(" - change_ownership deploy");

  const changeOwnerDeploy = reputationContract.createDeployChangeOwnership(
    recipientKeys.publicKey,
    DEPLOY_PAYMENT_AMOUNT,
    ownerKeys
  );
  const changeOwnerDeployHash = await changeOwnerDeploy.send(NODE_ADDRESS);
  await waitForDeploy(NODE_ADDRESS, changeOwnerDeployHash);
  // It's not dev-friendly but I couldn't find any better way to read the value from this CLType,
  // that is a result of parsing `ts-results` inside the `casper-js-sdk`? Is there any cleaner way to fix it?
  const newOwner = (await reputationContract.getOwner())["val"].data.data;
  const ownerChangedToRecipient =
    Buffer.from(recipientKeys.publicKey.toAccountHash()).toString("hex") ===
    Buffer.from(newOwner).toString("hex");

  console.log(
    ` - Owner changed to recipient: ${
      ownerChangedToRecipient ? "SUCCESS!" : "FAILED!"
    }`
  );
};

test();
