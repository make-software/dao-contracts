const fs = require('fs');
const {
  CasperServiceByJsonRPC,
  Keys,
  Contracts,
  CasperClient,
  CLValueBuilder,
  RuntimeArgs,
} = require('casper-js-sdk');
const path = require('path');
const {
  installContract,
  waitForDeploy,
  parseExecutionResult,
  parseWriteContract,
  stringToCLKey,
} = require('./utils');

const rawConfig = fs.readFileSync(path.resolve(__dirname, './config.json'), 'utf-8');

const config = JSON.parse(rawConfig);

const rpcAPI = new CasperServiceByJsonRPC(config.node_rpc_uri);
const contractClient = new Contracts.Contract(new CasperClient(config.node_rpc_uri));

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
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
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
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        kyc_token: stringToCLKey(contractsMap.KycNftContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
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
          address: stringToCLKey(contractToAdd.contractHash),
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

      return stringToCLKey(contract.contractHash);
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

main();
