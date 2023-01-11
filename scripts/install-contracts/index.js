const fs = require('fs');
const {
  CasperServiceByJsonRPC,
  Keys,
  Contracts,
  CasperClient,
  CLValueBuilder,
  RuntimeArgs,
  encodeBase16,
  decodeBase16,
} = require('casper-js-sdk');
const path = require('path');

const rawConfig = fs.readFileSync(path.resolve(__dirname, './config.json'), 'utf-8');

const config = JSON.parse(rawConfig);

const rpcAPI = new CasperServiceByJsonRPC(config.node_rpc_uri);
const contractClient = new Contracts.Contract(new CasperClient(config.node_rpc_uri));

async function main() {
  const baseWasmPath = path.resolve(__dirname, '../../target/wasm32-unknown-unknown/release');

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

  const independentContractsConfig = [
    {
      name: 'VariableRepositoryContract',
      wasmPath: `${baseWasmPath}/variable_repository_contract.wasm`,
      args: [],
      paymentAmount: '108880890230',
    },
    {
      name: 'ReputationContract',
      wasmPath: `${baseWasmPath}/reputation_contract.wasm`,
      args: [],
      paymentAmount: '183129693550',
    },
    {
      name: 'VaNftContract',
      wasmPath: `${baseWasmPath}/va_nft_contract.wasm`,
      args: {
        name: CLValueBuilder.string(config.contracts.VaNftContract.name),
        symbol: CLValueBuilder.string(config.contracts.VaNftContract.symbol),
        base_uri: CLValueBuilder.string(config.contracts.VaNftContract.base_uri),
      },
      paymentAmount: '119826860400',
    },
    {
      name: 'KycNftContract',
      wasmPath: `${baseWasmPath}/kyc_nft_contract.wasm`,
      args: {
        name: CLValueBuilder.string(config.contracts.KycNftContract.name),
        symbol: CLValueBuilder.string(config.contracts.KycNftContract.symbol),
        base_uri: CLValueBuilder.string(config.contracts.KycNftContract.base_uri),
      },
      paymentAmount: '119828799090',
    },
    {
      name: 'DaoIdsContract',
      wasmPath: `${baseWasmPath}/dao_ids_contract.wasm`,
      args: [],
      paymentAmount: '60562436540',
    },
  ];

  const indepContracts = await Promise.all(independentContractsConfig.map(async (cfg) => {
    let deploy = await deployContract(cfg.wasmPath, cfg.args, cfg.paymentAmount, status.chainspec_name, pk);

    const contract = parseWriteContract(deploy);

    return { name: cfg.name, ...contract };
  }));

  indepContracts.forEach(logContractOutput);

  const [variableRepositoryContract, reputationContract, vaNFTContract, kycNFTContract] = indepContracts;

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

  const daoContractsConfig = [
    {
      name: 'SlashingVoterContract',
      wasmPath: `${baseWasmPath}/slashing_voter_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '252363913860',
    },
    {
      name: 'SimpleVoterContract',
      wasmPath: `${baseWasmPath}/simple_voter_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '223485461700',
    },
    {
      name: 'ReputationVoterContract',
      wasmPath: `${baseWasmPath}/reputation_voter_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '226933514950',
    },
    {
      name: 'RepoVoterContract',
      wasmPath: `${baseWasmPath}/repo_voter_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '224252440240',
    },
    {
      name: 'AdminContract',
      wasmPath: `${baseWasmPath}/admin_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '221501219360',
    },
    {
      name: 'OnboardingRequestContract',
      wasmPath: `${baseWasmPath}/onboarding_request_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        kyc_token: stringToCLKey(kycNFTContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '257216800760',
    },
    {
      name: 'KycVoterContract',
      wasmPath: `${baseWasmPath}/kyc_voter_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        kyc_token: stringToCLKey(kycNFTContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '225739722330',
    },
    {
      name: 'BidEscrowContract',
      wasmPath: `${baseWasmPath}/bid_escrow_contract.wasm`,
      args: {
        variable_repo: stringToCLKey(variableRepositoryContract.contractHash),
        reputation_token: stringToCLKey(reputationContract.contractHash),
        kyc_token: stringToCLKey(kycNFTContract.contractHash),
        va_token: stringToCLKey(vaNFTContract.contractHash),
      },
      paymentAmount: '314222329770',
    },
  ];

  const contracts = await Promise.all(daoContractsConfig.map(async (cfg) => {
    let deploy = await deployContract(cfg.wasmPath, cfg.args, cfg.paymentAmount, status.chainspec_name, pk);

    const contract = parseWriteContract(deploy);

    return { name: cfg.name, ...contract };
  }));

  contracts.forEach(logContractOutput);
}

async function waitForDeploy(signedDeploy, timeout = 60000) {
  const sleep = (ms) => {
    return new Promise(resolve => setTimeout(resolve, ms));
  };

  const timer = setTimeout(() => {
    throw new Error('Timeout');
  }, timeout);
  while (true) {
    try {
      const deploy = await rpcAPI.getDeployInfo(encodeBase16(signedDeploy.hash));
      if (deploy.execution_results.length > 0) {
        clearTimeout(timer);
        return deploy;
      } else {
        await sleep(1000);
      }
    } catch(err) {
      console.log({err});
      await sleep(1000);
    }
  }
}

function logContractOutput(contract) {
  console.log(`
    Name: ${contract.name}
    Actual Cost: ${contract.actualCost}
    Contract Hash: ${contract.contractHash}
    Contract Package Hash: ${contract.contractPackageHash}
  `);
}

async function deployContract(wasmPath, rawArgs, paymentAmount, chainName, pk) {
  const wasm = new Uint8Array(fs.readFileSync(wasmPath, null).buffer);
  const args = RuntimeArgs.fromMap(rawArgs);

  const signedDeploy = await contractClient.install(wasm, args, paymentAmount, pk.publicKey, chainName, [pk]);

  await rpcAPI.deploy(signedDeploy);

  const result = await waitForDeploy(signedDeploy);

  return result;
};

function parseWriteContract(deploy) {
  if (deploy.execution_results.length < 0) {
    throw new Error('empty deploy execution result');
  }

  const executionResult = deploy.execution_results[0].result.Success;
  if (!executionResult) {
    throw new Error('failed deploy');
  }

  const writeContractTransform = executionResult.effect.transforms.find(t => t.transform === 'WriteContract');
  if (!writeContractTransform) {
    throw new Error('no write contract transform');
  }

  const writeContractPackageTransform = executionResult.effect.transforms.find(t => t.transform === 'WriteContractPackage');
  if (!writeContractPackageTransform) {
    throw new Error('no write contract package transform');
  }

  return {
    actualCost: executionResult.cost,
    contractHash: writeContractTransform.key.replace('hash-', ''),
    contractPackageHash: writeContractPackageTransform.key.replace('hash-', ''),
  };
}

function stringToCLKey(param) {
  return CLValueBuilder.key(CLValueBuilder.byteArray(decodeBase16(param)))
}

main();
