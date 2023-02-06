const fs = require('fs');
const {
  CasperServiceByJsonRPC,
  Keys,
  Contracts,
  CasperClient,
  CLValueBuilder,
  CLOption,
  CLU64,
  RuntimeArgs, decodeBase16, CLAccountHash, CLU8,
} = require('casper-js-sdk');
const path = require('path');
const {
  installContract,
  waitForDeploy,
  parseExecutionResult,
  parseWriteContract,
  stringToCLKey,
} = require('./utils');


const {  None } = require('ts-results');


const rawConfig = fs.readFileSync(path.resolve(__dirname, './config.json'), 'utf-8');

const config = JSON.parse(rawConfig);

const rpcAPI = new CasperServiceByJsonRPC(config.node_rpc_uri);
const contractClient = new Contracts.Contract(new CasperClient(config.node_rpc_uri));

const USER0_KEYS = Keys.Ed25519.parseKeyFiles(
    `/home/zhars/make/dao-contracts/scripts/install-contracts/keys/user-1/public_key.pem`,
    `/home/zhars/make/dao-contracts/scripts/install-contracts/keys/user-1/secret_key.pem`
);

const USER1_KEYS = Keys.Ed25519.parseKeyFiles(
    `/home/zhars/make/dao-contracts/scripts/install-contracts/keys/user-2/public_key.pem`,
    `/home/zhars/make/dao-contracts/scripts/install-contracts/keys/user-2/secret_key.pem`
);

const pk = Keys.Ed25519.loadKeyPairFromPrivateFile(process.env.PRIVATE_KEY_PATH || config.private_key_path);

async function main() {
  const status = await rpcAPI.getStatus();

  const pk = Keys.Ed25519.loadKeyPairFromPrivateFile(process.env.PRIVATE_KEY_PATH || config.private_key_path);

  console.log(`
    Info: Installing contracts:
      1) VariableRepositoryContract
      2) ReputationContract
      3) DaoIdsContract
  `);

  let contractDeploymentResults = await Promise.all(['VariableRepositoryContract', 'ReputationContract', 'DaoIdsContract'].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      [],
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    const contract = parseWriteContract(deploy);

    return { name: cn, ...contract, deployHash: deploy.deploy.hash };
  }));

  let contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), {});

  contractDeploymentResults.forEach(logContractOutput);

  console.log(`
    Info: Installing contracts:
      4) VaNftContract
      5) KycNftContract
  `);

  contractDeploymentResults = await Promise.all(['VaNftContract', 'KycNftContract'].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      {
        name: CLValueBuilder.string(config.contracts[cn].args.name),
        symbol: CLValueBuilder.string(config.contracts[cn].args.symbol),
        base_uri: CLValueBuilder.string(config.contracts[cn].args.base_uri),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    const contract = parseWriteContract(deploy);

    return { name: cn, ...contract, deployHash: deploy.deploy.hash };
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach(logContractOutput);

  console.log(`
    Info: Installing contracts:
      6) SlashingVoterContract
      7) SimpleVoterContract
      8) ReputationVoterContract
      9) RepoVoterContract
      10) AdminContract
  `);

  contractDeploymentResults = await Promise.all([
    'SlashingVoterContract',
    'SimpleVoterContract',
    'ReputationVoterContract',
    'RepoVoterContract',
    'AdminContract',
  ].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractPackageHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractPackageHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractPackageHash),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    const contract = parseWriteContract(deploy);

    return { name: cn, ...contract, deployHash: deploy.deploy.hash };
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach(logContractOutput);

  console.log(`
    Info: Installing contracts:
      11) OnboardingRequestContract
      12) KycVoterContract
      13) BidEscrowContract
  `);

  contractDeploymentResults = await Promise.all([
    'OnboardingRequestContract',
    'KycVoterContract',
    'BidEscrowContract',
  ].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractPackageHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractPackageHash),
        kyc_token: stringToCLKey(contractsMap.KycNftContract.contractPackageHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractPackageHash
        ),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    const contract = parseWriteContract(deploy);

    return { name: cn, ...contract, deployHash: deploy.deploy.hash };
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach(logContractOutput);

  console.log(`
    Info: Setupping contracts:
  `);

  const contractCalls = Object.entries(config.contracts).reduce((acc, [key,contract]) => {
    if (!contract.whitelisting) {
      return acc;
    }

    const installedContract = contractsMap[key];
    if (!installedContract) {
      throw Error('failed to find installed contract for whitelisting');
    }

    contractClient.setContractHash(
      `hash-${installedContract.contractHash}`,
      `hash-${installedContract.contractPackageHash}`,
    );

    contract.whitelisting.contracts.forEach(cn => {
      const contractToAdd = contractsMap[cn];
      if (!contractToAdd) {
        throw new Error('failed to find contract to whitelist')
      }

      const deploy = contractClient.callEntrypoint(
        'add_to_whitelist',
        RuntimeArgs.fromMap({
          address: stringToCLKey(contractToAdd.contractPackageHash),
        }),
        pk.publicKey,
        status.chainspec_name,
        contract.whitelisting.payment_amount,
        [pk]
      );

      acc.push({ name: `${installedContract.name}->add_to_whitelist->(${contractToAdd.name})`, deploy });
    });

    return acc;
  }, []);

  if (config.contracts.SlashingVoterContract.bid_escrow_list) {
    contractClient.setContractHash(
      `hash-${contractsMap.SlashingVoterContract.contractHash}`,
      `hash-${contractsMap.SlashingVoterContract.contractPackageHash}`,
    );


    const clKeys = config.contracts.SlashingVoterContract.bid_escrow_list.contracts.map(contractName => {
      const contract = contractsMap[contractName];
      if (!contract) {
        throw new Error('failed to find contract for bid escrow list');
      }

      return stringToCLKey(contract.contractPackageHash);
    })

    const deploy = contractClient.callEntrypoint(
      'update_bid_escrow_list',
      RuntimeArgs.fromMap({
        bid_escrows: CLValueBuilder.list(clKeys),
      }),
      pk.publicKey,
      status.chainspec_name,
      config.contracts.SlashingVoterContract.bid_escrow_list.payment_amount,
      [pk]
    );

    contractCalls.push({ name: `SlashingVoterContract->update_bid_escrow_list(${config.contracts.SlashingVoterContract.bid_escrow_list.contracts.join(',')})`, deploy });
  }

  const results = await Promise.all(contractCalls.map(async (c) => {
    await rpcAPI.deploy(c.deploy);

    const processedDeploy = await waitForDeploy(rpcAPI, c.deploy.hash);

    const executionResult = parseExecutionResult(processedDeploy);

    return { name: c.name, actualCost: executionResult.cost, deployHash: processedDeploy.deploy.hash };
  }));

  results.map(logContractCallOutput);
}

const mintVA = async (recipient_key, va_hash, va_contract_package, wait=false) => {
  const status = await rpcAPI.getStatus();

  contractClient.setContractHash(
      `hash-${va_hash}`,
      `hash-${va_contract_package}`,
  );


  const deploy = contractClient.callEntrypoint(
      'mint',
      RuntimeArgs.fromMap({
        to: CLValueBuilder.key(new CLAccountHash(recipient_key)),
      }),
      pk.publicKey,
      status.chainspec_name,
      319825028240,
      [pk]
  );

  await rpcAPI.deploy(deploy);

  const processedDeploy = await waitForDeploy(rpcAPI, deploy.hash);
  console.log(processedDeploy)

  const executionResult = parseExecutionResult(processedDeploy);
  console.log(executionResult)
};

const mintKYC = async (recipient_key, kyc_hash, kyc_contract_package, wait=false) => {
  const status = await rpcAPI.getStatus();

  contractClient.setContractHash(
      `hash-${kyc_hash}`,
      `hash-${kyc_contract_package}`,
  );


  const deploy = contractClient.callEntrypoint(
      'mint',
      RuntimeArgs.fromMap({
        to: CLValueBuilder.key(new CLAccountHash(recipient_key)),
      }),
      pk.publicKey,
      status.chainspec_name,
      319825028240,
      [pk]
  );
  await rpcAPI.deploy(deploy);

  const processedDeploy = await waitForDeploy(rpcAPI, deploy.hash);
  console.log(processedDeploy)

  const executionResult = parseExecutionResult(processedDeploy);
  console.log(executionResult)
};

const mintReputation = async (recipient_key, reputation_contract_hash, reputation_contract_contract_package, reputation_amount, wait=false) => {
  const status = await rpcAPI.getStatus();

  contractClient.setContractHash(
      `hash-${reputation_contract_hash}`,
      `hash-${reputation_contract_contract_package}`,
  );

  const deploy = contractClient.callEntrypoint(
      'mint',
      RuntimeArgs.fromMap({
        recipient: CLValueBuilder.key(new CLAccountHash(recipient_key)),
        amount: CLValueBuilder.u512(reputation_amount),
      }),
      pk.publicKey,
      status.chainspec_name,
      319825028240,
      [pk]
  );

  await rpcAPI.deploy(deploy);

  const processedDeploy = await waitForDeploy(rpcAPI, deploy.hash);

  console.log(processedDeploy)
  const executionResult = parseExecutionResult(processedDeploy);
  console.log(executionResult)
};


const startVoting = async (sender, simple_voter_contract_hash, simple_voter_contract_contract_package, stake, wait=false) => {
  console.log("VA Start Simple Voting ----->")

  const status = await rpcAPI.getStatus();

  contractClient.setContractHash(
      `hash-${simple_voter_contract_hash}`,
      `hash-${simple_voter_contract_contract_package}`,
  );

  const deploy = contractClient.callEntrypoint(
      'create_voting',
      RuntimeArgs.fromMap({
        document_hash: CLValueBuilder.list([CLValueBuilder.u8(1)]),
        stake: CLValueBuilder.u512(stake),
      }),
      sender.publicKey,
      status.chainspec_name,
      319825028240,
      [sender]
  );

  await rpcAPI.deploy(deploy);

  const processedDeploy = await waitForDeploy(rpcAPI, deploy.hash);

  console.log(processedDeploy)
  const executionResult = parseExecutionResult(processedDeploy);

  console.log(executionResult)
};


const updateAT = async (variable_repo_contract_hash, variable_repo_contract_contract_package, wait=false) => {
  console.log("Updating Variable Repository")

  const status = await rpcAPI.getStatus();

  contractClient.setContractHash(
      `hash-${variable_repo_contract_hash}`,
      `hash-${variable_repo_contract_contract_package}`,
  );


  const key = "VotingIdsAddress";
  let utf8Encode = new TextEncoder();

    let bytes = [];
    const clU8 = utf8Encode.encode(key).map(byte => {
        bytes.push(new CLU8(byte));
    })
  const deploy = contractClient.callEntrypoint(
      'update_at',
      RuntimeArgs.fromMap({
        key: CLValueBuilder.string(key),
        value: CLValueBuilder.list(bytes),
        activation_time: new CLOption(None, new CLU64()),
      }),
      pk.publicKey,
      status.chainspec_name,
      319825028240,
      [pk]
  );

  await rpcAPI.deploy(deploy);

  const processedDeploy = await waitForDeploy(rpcAPI, deploy.hash);

  console.log(processedDeploy)
  const executionResult = parseExecutionResult(processedDeploy);

  console.log(executionResult)
};

function logContractOutput(contract) {
  console.log(`
    Name: ${contract.name}
    Actual Cost: ${contract.actualCost}
    Contract Hash: ${contract.contractHash}
    Contract Package Hash: ${contract.contractPackageHash}
    Deploy Hash: ${contract.deployHash}
  `);
}

function logContractCallOutput(contractCall) {
  console.log(`
    Call: ${contractCall.name}
    Actual Cost: ${contractCall.actualCost}
    Deploy Hash: ${contractCall.deployHash}
  `);
}

// main();
//

// console.log("--------------------------------------------------------------------------")
// console.log("user1")
// console.log("--------------------------------------------------------------------------")
//
// console.log("Mint VA User0")
// mintVA(USER0_KEYS.publicKey.toAccountHash(), 'c786af623946362ef5dbb90f850290c949792722a251006720674c85532d3148', '886d7c0852c3f6b70dba0e3fd5e3ba8d49cd6273398e0c676efe72b39e0b5be8')
//
// console.log("Mint KYC User0")
// mintKYC(USER0_KEYS.publicKey.toAccountHash(), '3f3e831d6e4f5d498bc12ba6c7415d01bc454533cab78d2f5a6b024b5db67c84', '122d9a0e3a0e9b01374cf5f962fa08e6da9c4e3813fc4a426346a4d86573060e')
//
// console.log("Mint Reputation User0")
// mintReputation(USER0_KEYS.publicKey.toAccountHash(), 'f1b67e8ce17fcbbc36f591ebdecf802f3256d7299c6da371ed899924bd046731', 'aa9d85d36b3cfa2df485fbf113d3baaff811be92605c21c23c09e8b1174759c0', 5000)

//
//
//
// console.log("--------------------------------------------------------------------------")
// console.log("user2")
// console.log("--------------------------------------------------------------------------")
//
// console.log("Mint VA User2")
// mintVA(USER1_KEYS.publicKey.toAccountHash(), 'c786af623946362ef5dbb90f850290c949792722a251006720674c85532d3148', '886d7c0852c3f6b70dba0e3fd5e3ba8d49cd6273398e0c676efe72b39e0b5be8')
//
// console.log("Mint KYC User2")
// mintKYC(USER1_KEYS.publicKey.toAccountHash(), '3f3e831d6e4f5d498bc12ba6c7415d01bc454533cab78d2f5a6b024b5db67c84', '122d9a0e3a0e9b01374cf5f962fa08e6da9c4e3813fc4a426346a4d86573060e')
//
// console.log("Mint Reputation User2")
// mintReputation(USER1_KEYS.publicKey.toAccountHash(), 'f1b67e8ce17fcbbc36f591ebdecf802f3256d7299c6da371ed899924bd046731', 'aa9d85d36b3cfa2df485fbf113d3baaff811be92605c21c23c09e8b1174759c0', 5000)

updateAT("58a9a01a25f14fa21996e4cbfab6b4036076aa54f493ad93aef76979a3476fa7", "b9bd1316183c174a8d3ac89bc25548120edb196585e9a2ca8f51ea7ad4f9d9aa")
//startVoting(USER0_KEYS, '16a414712240a7690314cc22a189aa4c166aaf3c617215bd9be3a767be12eacd', '0cb00c38e5bc87e351f735ddddcc637ec9710b00ea9123e9b3627eb411adeb74', 1000)
