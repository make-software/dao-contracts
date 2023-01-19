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

  const pk = Keys.Ed25519.loadKeyPairFromPrivateFile(config.private_key_path);

  console.log(`
    Info: Installing contracts:
      1) VariableRepositoryContract
      2) ReputationContract
      3) DaoIdsContract
      4) VaNftContract
      5) KycNftContract
  `);

  let contractsConfig = [
    {
      name: 'VariableRepositoryContract',
      wasmPath: path.resolve(__dirname, config.contracts.VariableRepositoryContract.wasm_relative_path),
      args: [],
      paymentAmount: config.contracts.VariableRepositoryContract.payment_amount,
    },
    {
      name: 'ReputationContract',
      wasmPath: path.resolve(__dirname, config.contracts.ReputationContract.wasm_relative_path),
      args: [],
      paymentAmount: config.contracts.ReputationContract.payment_amount,
    },
    {
      name: 'VaNftContract',
      wasmPath: path.resolve(__dirname, config.contracts.VaNftContract.wasm_relative_path),
      args: {
        name: CLValueBuilder.string(config.contracts.VaNftContract.args.name),
        symbol: CLValueBuilder.string(config.contracts.VaNftContract.args.symbol),
        base_uri: CLValueBuilder.string(config.contracts.VaNftContract.args.base_uri),
      },
      paymentAmount: config.contracts.VaNftContract.payment_amount,
    },
    {
      name: 'KycNftContract',
      wasmPath: path.resolve(__dirname, config.contracts.KycNftContract.wasm_relative_path),
      args: {
        name: CLValueBuilder.string(config.contracts.KycNftContract.args.name),
        symbol: CLValueBuilder.string(config.contracts.KycNftContract.args.symbol),
        base_uri: CLValueBuilder.string(config.contracts.KycNftContract.args.base_uri),
      },
      paymentAmount: config.contracts.KycNftContract.payment_amount,
    },
    {
      name: 'DaoIdsContract',
      wasmPath: path.resolve(__dirname, config.contracts.DaoIdsContract.wasm_relative_path),
      args: [],
      paymentAmount: config.contracts.DaoIdsContract.payment_amount,
    },
  ];

  let contractDeploymentResults = await Promise.all(contractsConfig.map(async (cfg) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      cfg.wasmPath,
      cfg.args,
      cfg.paymentAmount,
      status.chainspec_name,
      pk,
    );

    const contract = parseWriteContract(deploy);

    return { name: cfg.name, ...contract, deployHash: deploy.deploy.hash };
  }));

  contractDeploymentResults.forEach(logContractOutput);

  let contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), {});

  console.log(`
    Info: Installing contracts:
      6) SlashingVoterContract
      7) SimpleVoterContract
      8) ReputationVoterContract
      9) RepoVoterContract
      10) AdminContract
      11) OnboardingRequestContract
      12) KycVoterContract
      13) BidEscrowContract
  `);

  contractsConfig = [
    {
      name: 'SlashingVoterContract',
      wasmPath: path.resolve(__dirname, config.contracts.SlashingVoterContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.SlashingVoterContract.payment_amount,
    },
    {
      name: 'SimpleVoterContract',
      wasmPath: path.resolve(__dirname, config.contracts.SimpleVoterContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.SimpleVoterContract.payment_amount,
    },
    {
      name: 'ReputationVoterContract',
      wasmPath: path.resolve(__dirname, config.contracts.ReputationVoterContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.ReputationVoterContract.payment_amount,
    },
    {
      name: 'RepoVoterContract',
      wasmPath: path.resolve(__dirname, config.contracts.RepoVoterContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.RepoVoterContract.payment_amount,
    },
    {
      name: 'AdminContract',
      wasmPath: path.resolve(__dirname, config.contracts.AdminContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.AdminContract.payment_amount,
    },
    {
      name: 'OnboardingRequestContract',
      wasmPath: path.resolve(__dirname, config.contracts.OnboardingRequestContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        kyc_token: stringToCLKey(contractsMap.KycNftContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.OnboardingRequestContract.payment_amount,
    },
    {
      name: 'KycVoterContract',
      wasmPath: path.resolve(__dirname, config.contracts.KycVoterContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        kyc_token: stringToCLKey(contractsMap.KycNftContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.KycVoterContract.payment_amount,
    },
    {
      name: 'BidEscrowContract',
      wasmPath: path.resolve(__dirname, config.contracts.BidEscrowContract.wasm_relative_path),
      args: {
        variable_repo: stringToCLKey(contractsMap.VariableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractHash),
        kyc_token: stringToCLKey(contractsMap.KycNftContract.contractHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractHash),
      },
      paymentAmount: config.contracts.BidEscrowContract.payment_amount,
    },
  ];

  contractDeploymentResults = await Promise.all(contractsConfig.map(async (cfg) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      cfg.wasmPath,
      cfg.args,
      cfg.paymentAmount,
      status.chainspec_name,
      pk,
    );

    const contract = parseWriteContract(deploy);

    return { name: cfg.name, ...contract, deployHash: deploy.deploy.hash };
  }));

  contractDeploymentResults.forEach(logContractOutput);

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

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
