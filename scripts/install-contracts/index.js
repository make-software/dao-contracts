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

  let totalCost = 0;

  // ======================================================================================================

  console.log(`
    Info: Installing contracts:
      1) ReputationContract
      2) DaoIdsContract
  `);

  let contractDeploymentResults = await Promise.all(['ReputationContract', 'DaoIdsContract'].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      [],
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    try {
      const contract = parseWriteContract(deploy);

      return { name: cn, ...contract, deployHash: deploy.deploy.hash };
    } catch(err) {
      throw new Error(`failed to parse write contract for contract name: ${cn}, err: ${err.message}`);
    }
  }));

  let contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), {});

  contractDeploymentResults.forEach((c) => {
    logContractOutput(c);
    totalCost += parseInt(c.actualCost);
  });

  // ======================================================================================================

  console.log(`
    Info: Installing contracts:
     3) CSPRRateProviderContract
  `)

  contractDeploymentResults = await Promise.all([
    'CSPRRateProviderContract',
  ].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      {
        rate: CLValueBuilder.u512(config.contracts[cn].args.rate),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    try {
      const contract = parseWriteContract(deploy);

      return { name: cn, ...contract, deployHash: deploy.deploy.hash };
    } catch (err) {
      throw new Error(`failed to parse write contract for contract name: ${cn}, err: ${err.message}`);
    }
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach((c) => {
    logContractOutput(c);
    totalCost += parseInt(c.actualCost);
  });

  // ======================================================================================================

  console.log(`
    Info: Installing contracts:
      4) VariableRepositoryContract
  `);

  contractDeploymentResults = await Promise.all(['VariableRepositoryContract'].map(async (cn) => {
    let deploy = await installContract(
      contractClient,
      rpcAPI,
      path.resolve(__dirname, config.contracts[cn].wasm_relative_path),
      {
        fiat_conversion: stringToCLKey(contractsMap.CSPRRateProviderContract.contractPackageHash),
        bid_escrow_wallet: CLValueBuilder.key(CLValueBuilder.byteArray(pk.accountHash())),
        voting_ids: stringToCLKey(contractsMap.DaoIdsContract.contractPackageHash),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    try {
      const contract = parseWriteContract(deploy);

      return { name: cn, ...contract, deployHash: deploy.deploy.hash };
    } catch (err) {
      throw new Error(`failed to parse write contract for contract name: ${cn}, err: ${err.message}`);
    }
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach((c) => {
    logContractOutput(c);
    totalCost += parseInt(c.actualCost);
  });

  // ======================================================================================================

  console.log(`
    Info: Installing contracts:
      5) VaNftContract
      6) KycNftContract
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

    try {
      const contract = parseWriteContract(deploy);

      return { name: cn, ...contract, deployHash: deploy.deploy.hash };
    } catch (err) {
      throw new Error(`failed to parse write contract for contract name: ${cn}, err: ${err.message}`);
    }
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach((c) => {
    logContractOutput(c);
    totalCost += parseInt(c.actualCost);
  });

  console.log(`
    Info: Installing contracts:
      7) SlashingVoterContract
      8) SimpleVoterContract
      9) ReputationVoterContract
      10) RepoVoterContract
      11) AdminContract
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
        variable_repository: stringToCLKey(contractsMap.VariableRepositoryContract.contractPackageHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractPackageHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractPackageHash),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    try {
      const contract = parseWriteContract(deploy);

      return { name: cn, ...contract, deployHash: deploy.deploy.hash };
    } catch (err) {
      throw new Error(`failed to parse write contract for contract name: ${cn}, err: ${err.message}`);
    }
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach((c) => {
    logContractOutput(c);
    totalCost += parseInt(c.actualCost);
  });

  console.log(`
    Info: Installing contracts:
      12) OnboardingRequestContract
      13) KycVoterContract
      14) BidEscrowContract
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
        variable_repository: stringToCLKey(contractsMap.VariableRepositoryContract.contractPackageHash),
        reputation_token: stringToCLKey(contractsMap.ReputationContract.contractPackageHash),
        kyc_token: stringToCLKey(contractsMap.KycNftContract.contractPackageHash),
        va_token: stringToCLKey(contractsMap.VaNftContract.contractPackageHash),
      },
      config.contracts[cn].payment_amount,
      status.chainspec_name,
      pk,
    );

    try {
      const contract = parseWriteContract(deploy);

      return { name: cn, ...contract, deployHash: deploy.deploy.hash };
    } catch (err) {
      throw new Error(`failed to parse write contract for contract name: ${cn}, err: ${err.message}`);
    }
  }));

  contractsMap = contractDeploymentResults.reduce((acc, el) => ({ ...acc, [el.name]: el }), contractsMap);

  contractDeploymentResults.forEach((c) => {
    logContractOutput(c);
    totalCost += parseInt(c.actualCost);
  });

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

  // Post install setup

  // contractClient.setContractHash(
  //   `hash-${contractsMap.VariableRepositoryContract.contractHash}`,
  //   `hash-${contractsMap.VariableRepositoryContract.contractPackageHash}`,
  // );

  // const conversionRateAddressBytes = decodeBase16(contractsMap.CSPRRateProviderContract.contractPackageHash).reduce((acc, el) => {
  //   acc.push(CLValueBuilder.u8(el));

  //   return acc;
  // }, []);

  // const conversionRateUpdateDeploy = contractClient.callEntrypoint(
  //   'update_at',
  //   RuntimeArgs.fromMap({
  //     key: CLValueBuilder.string('FiatConversionRateAddress'),
  //     value: CLValueBuilder.list(conversionRateAddressBytes),
  //     activation_time: CLValueBuilder.option(None, new CLU64Type()),
  //   }),
  //   pk.publicKey,
  //   status.chainspec_name,
  //   '2703423570',
  //   [pk]
  // );

  // contractCalls.push({ name: 'VariableRepositoryContract->update_at(CSPRRateProviderContract)', deploy: conversionRateUpdateDeploy });

  // const daoIdsAddressBytes = decodeBase16(contractsMap.DaoIdsContract.contractPackageHash).reduce((acc, el) => {
  //   acc.push(CLValueBuilder.u8(el));

  //   return acc;
  // }, []);

  // const daoIdsUpdateDeploy = contractClient.callEntrypoint(
  //   'update_at',
  //   RuntimeArgs.fromMap({
  //     key: CLValueBuilder.string('VotingIdsAddress'),
  //     value: CLValueBuilder.list(daoIdsAddressBytes),
  //     activation_time: CLValueBuilder.option(None, new CLU64Type()),
  //   }),
  //   pk.publicKey,
  //   status.chainspec_name,
  //   '2675884340',
  //   [pk]
  // );

  // contractCalls.push({ name: 'VariableRepositoryContract->update_at(DaoIdsContract)', deploy: daoIdsUpdateDeploy });

  const results = await Promise.all(contractCalls.map(async (c) => {
    await rpcAPI.deploy(c.deploy);

    const processedDeploy = await waitForDeploy(rpcAPI, c.deploy.hash);

    try {
      const executionResult = parseExecutionResult(processedDeploy);

      return { name: c.name, actualCost: executionResult.cost, deployHash: processedDeploy.deploy.hash };
    } catch (err) {
      throw new Error(`failed to parse contract call for: ${c.name}, err: ${err.message}`);
    }
  }));

  results.map((c) => {
    logContractCallOutput(c);
    totalCost += parseInt(c.actualCost);
  });

  console.log(`Total installation cost in motes: ${totalCost}`);
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
