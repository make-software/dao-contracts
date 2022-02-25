import { utils } from 'casper-js-client-helper';
import { Keys } from 'casper-js-sdk';
import { config } from 'dotenv';

import { installReputationContract, ReputationContractJSClient } from '../src';
import { createRpcClient } from '../src/common/rpc-client';
import { getAccountInfo, getAccountNamedKeyValue, waitForDeploy } from './utils';

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

  console.log(
    `... Deploy is settled which means contract is installed successfully.`
  );

  let accountInfo = await getAccountInfo(NODE_ADDRESS, ownerKeys.publicKey);

  const contractPackageHash = await getAccountNamedKeyValue(
    accountInfo,
    `reputation_contract_package_hash`
  );

  console.log(` - Contract Package Hash: ${contractPackageHash}`);

  const stateRootHash = await utils.getStateRootHash(NODE_ADDRESS);
  const rpcClient = createRpcClient(NODE_ADDRESS);
  const res = await rpcClient.fetchStateGetItem(
    stateRootHash,
    contractPackageHash,
    []
  );

  const contractHash = res.ContractPackage.versions[0].contract_hash;
  console.log(contractHash);
  const contractHashWithHashPrefix = contractHash.replace("contract-", "hash-");
  console.log(contractHashWithHashPrefix);

  // Initialize contract client
  const reputationContract = new ReputationContractJSClient(
    NODE_ADDRESS,
    CHAIN_NAME,
    EVENT_STREAM_ADDRESS,
    contractPackageHash,
    contractHashWithHashPrefix
  );

  console.log(`... Testing named keys getters ...`);

  const owner = await reputationContract.getOwner();
  console.log(` - Owner: ${owner}`);

  const total_supply = await reputationContract.getTotalSupply();
  console.log(` - Total Supply: ${total_supply}`);

  const balances = await reputationContract.getBalanceOf(
    recipientKeys.publicKey
  );
  console.log(` - Balances: ${balances}`);

  const stakes = await reputationContract.getStakeOf(recipientKeys.publicKey);
  console.log(` - Stakes: ${stakes}`);

  const whitelist = await reputationContract.getWhitelistOf(
    recipientKeys.publicKey
  );
  console.log(` - Whitelist: ${whitelist}`);

  console.log(`... Testing deploys ...`);

  console.log("... mint");

  const mintAmount = "200000000000";
  const deployHashMint = await reputationContract.mint(
    ownerKeys,
    ownerKeys.publicKey,
    mintAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, deployHashMint);
  const totalSupplyEqMint =
    mintAmount === (await reputationContract.getTotalSupply());

  console.log(
    ` - Total supply equals mint: ${totalSupplyEqMint ? "Success" : "Failed"}`
  );

  console.log("... burn");

  const burnAmount = "100000000000";
  const deployHashBurn = await reputationContract.burn(
    ownerKeys,
    ownerKeys.publicKey,
    burnAmount,
    DEPLOY_PAYMENT_AMOUNT
  );
  await waitForDeploy(NODE_ADDRESS, deployHashBurn);
  const totalSupplySubtractedByBurn =
    (Number(mintAmount) - Number(burnAmount)).toString() ===
    (await reputationContract.getTotalSupply());

  console.log(
    ` - Total supply equals zero: ${totalSupplyEqMint ? "Success" : "Failed"}`
  );

  console.log("... transfer_from");
  console.log("... change_ownership");
  console.log("... add_to_whitelist");
  console.log("... remove_from_whitelist");
  console.log("... stake");
  console.log("... unstake");
};

test();
