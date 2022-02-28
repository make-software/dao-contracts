import { utils } from "casper-js-client-helper";
import { EventName, EventStream, Keys } from "casper-js-sdk";
import { config } from "dotenv";

import {
  installReputationContract,
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

  const installDeployHash = await installReputationContract(
    CHAIN_NAME,
    NODE_ADDRESS,
    ownerKeys,
    INSTALL_PAYMENT_AMOUNT,
    WASM_PATH
  );

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

  console.log(`\n`);
  console.log(" - mint deploy sent");

  const mintAmount = "200000000000";
  const deployHashMint = await reputationContract.mint(
    ownerKeys,
    ownerKeys.publicKey,
    mintAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, deployHashMint);
  const totalSupplyEqMint =
    mintAmount === (await reputationContract.getTotalSupply()).toString();

  console.log(
    ` - Total supply equals mint: ${totalSupplyEqMint ? "Success" : "Failed"}`
  );

  console.log(`\n`);
  console.log(" - burn deploy sent");

  const burnAmount = "100000000000";
  const burnDeployHash = await reputationContract.burn(
    ownerKeys,
    ownerKeys.publicKey,
    burnAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, burnDeployHash);
  const totalSupplyEqMintSubtractedByBurn =
    (Number(mintAmount) - Number(burnAmount)).toString() ===
    (await reputationContract.getTotalSupply()).toString();

  console.log(
    ` - Total supply equals mint subtracted by burn: ${
      totalSupplyEqMintSubtractedByBurn ? "Success" : "Failed"
    }`
  );

  console.log(`\n`);
  console.log(" - transfer_from deploy sent");

  const transferAmount = "10000000";
  const transferDeployHash = await reputationContract.transferFrom(
    ownerKeys,
    ownerKeys.publicKey,
    recipientKeys.publicKey,
    transferAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, transferDeployHash);
  // FIXME: @maciej balance named-key returns a simple string while total-supply returns a Big number representation, both of them can represent the same number. I would suggest to have balance also return a Big number representation so they are consistent.
  const recipientBalanceEqTransferAmount =
    transferAmount ===
    (await reputationContract.getBalanceOf(recipientKeys.publicKey));

  console.log(
    ` - Recipient balance received transfer: ${
      recipientBalanceEqTransferAmount ? "Success" : "Failed"
    }`
  );

  console.log(`\n`);
  console.log(" - add_to_whitelist deploy sent");

  const whitelistAddDeployHash = await reputationContract.addToWhitelist(
    ownerKeys,
    recipientKeys.publicKey,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, whitelistAddDeployHash);
  const recipientAddedToTheWhitelist =
    "true" ===
    (await reputationContract.getWhitelistOf(recipientKeys.publicKey));

  console.log(
    ` - Recipient is added to the whitelist: ${
      recipientAddedToTheWhitelist ? "Success" : "Failed"
    }`
  );

  console.log(`\n`);
  console.log(" - remove_from_whitelist deploy sent");

  const whitelistRemoveDeployHash =
    await reputationContract.removeFromWhitelist(
      ownerKeys,
      recipientKeys.publicKey,
      DEPLOY_PAYMENT_AMOUNT
    );
  await waitForDeploy(NODE_ADDRESS, whitelistRemoveDeployHash);
  const recipientRemovedFromTheWhitelist =
    "false" ===
    (await reputationContract.getWhitelistOf(recipientKeys.publicKey));

  console.log(
    ` - Recipient is removed from the whitelist: ${
      recipientRemovedFromTheWhitelist ? "Success" : "Failed"
    }`
  );

  console.log(`\n`);
  console.log(" - stake deploy sent");

  const stakeAmount = "10000000";
  const stakeDeployHash = await reputationContract.stake(
    ownerKeys,
    ownerKeys.publicKey,
    stakeAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, stakeDeployHash);
  const stakeAmountWasStaked =
    stakeAmount === (await reputationContract.getStakeOf(ownerKeys.publicKey));

  console.log(
    ` - Requested amount was staked: ${
      stakeAmountWasStaked ? "Success" : "Failed"
    }`
  );

  console.log(`\n`);
  console.log(" - unstake deploy sent");

  const unstakeDeployHash = await reputationContract.unstake(
    ownerKeys,
    ownerKeys.publicKey,
    stakeAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, unstakeDeployHash);
  const stakeAmountWasUnstaked =
    "0" === (await reputationContract.getStakeOf(ownerKeys.publicKey));

  console.log(
    ` - Requested amount was unstaked: ${
      stakeAmountWasUnstaked ? "Success" : "Failed"
    }`
  );

  console.log(`\n`);
  console.log(" - change_ownership deploy sent");

  const changeOwnerDeployHash = await reputationContract.changeOwnership(
    ownerKeys,
    recipientKeys.publicKey,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, changeOwnerDeployHash);
  // FIXME: @maciej @jan I couldn't found any better way to read the value from this CLType, it's not consumer friendly, I guess that is the result of parsing in casper-js-sdk? Isn't there any cleaner way to do it?
  const newOwner = (await reputationContract.getOwner())["val"].data.data;
  const ownerChangedToRecipient =
    Buffer.from(recipientKeys.publicKey.toAccountHash()).toString("hex") ===
    Buffer.from(newOwner).toString("hex");

  console.log(
    ` - Owner changed to recipient: ${
      ownerChangedToRecipient ? "Success" : "Failed"
    }`
  );
};

test();
