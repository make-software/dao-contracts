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
  prepareContractCallDeploy,
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

  let contractsConfig = [
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

  const [
    variableRepositoryContract,
    reputationContract,
    vaNFTContract,
    kycNFTContract,
    daoIdsContract,
  ] = contractDeploymentResults;

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

  const [
    slashingVoterContract,
    simpleVoterContract,
    reputationVoterContract,
    repoVoterContract,
    adminContract,
    onboardingRequestContract,
    kycVoterContract,
    bidEscrowContract,
  ] = contractDeploymentResults;

  console.log(`
    Info: Whitelisting contracts:
  `);

  const contractCallsConfig = [
    // 1) OnboardingRequestContract <- SlashingVoterContract
    {
      name: "OnboardingRequestContract->add_to_whitelist(SlashingVoterContract)",
      contract: onboardingRequestContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "361497030",
    },
    // 2) DaoIdsContract <- KycVoterContract
    //    DaoIdsContract <- BidEscrowContract
    //    DaoIdsContract <- OnboardingRequestContract
    //    DaoIdsContract <- SlashingVoterContract
    //    DaoIdsContract <- RepoVoterContract
    //    DaoIdsContract <- ReputationVoterContract
    //    DaoIdsContract <- SimpleVoterContract
    //    DaoIdsContract <- AdminContract
    {
      name: "DaoIdsContract->add_to_whitelist(KycVoterContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(kycVoterContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(BidEscrowContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(bidEscrowContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(OnboardingRequestContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(onboardingRequestContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(SlashingVoterContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(RepoVoterContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(repoVoterContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(ReputationVoterContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(reputationVoterContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(SimpleVoterContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(simpleVoterContract.contractHash),
      }),
      payment: "319808600",
    },
    {
      name: "DaoIdsContract->add_to_whitelist(AdminContract)",
      contract: daoIdsContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(adminContract.contractHash),
      }),
      payment: "319808600",
    },
    // 3) ReputationContract <- AdminContract
    {
      name: "ReputationContract->add_to_whitelist(AdminContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(adminContract.contractHash),
      }),
      payment: "336074520",
    },
    // 4) ReputationContract <- KycVoterContract
    //    KycNftContract <- KycVoterContract
    {
      name: "ReputationContract->add_to_whitelist(KycVoterContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(kycVoterContract.contractHash),
      }),
      payment: "336074520",
    },
    {
      name: "KycNftContract->add_to_whitelist(KycVoterContract)",
      contract: kycNFTContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(kycVoterContract.contractHash),
      }),
      payment: "337962950",
    },
    // 5) ReputationContract <- RepoVoterContract
    //    RepoVoterContract <- RepoVoterContract
    {
      name: "ReputationContract->add_to_whitelist(RepoVoterContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(repoVoterContract.contractHash),
      }),
      payment: "336074520",
    },
    {
      name: "RepoVoterContract->add_to_whitelist(RepoVoterContract)",
      contract: repoVoterContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(repoVoterContract.contractHash),
      }),
      payment: "332353080",
    },
    // 6) ReputationContract <- ReputationVoterContract
    {
      name: "ReputationContract->add_to_whitelist(ReputationVoterContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(reputationVoterContract.contractHash),
      }),
      payment: "336074520",
    },
    // 7) ReputationContract <- SlashingVoterContract
    //    VaNftContract <- SlashingVoterContract
    //    RepoVoterContract <- SlashingVoterContract
    //    KycVoterContract <- SlashingVoterContract
    //    BidEscrowContract <- SlashingVoterContract
    //    AdminContract <- SlashingVoterContract
    {
      name: "ReputationContract->add_to_whitelist(SlashingVoterContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "336074520",
    },
    {
      name: "VaNftContract->add_to_whitelist(SlashingVoterContract)",
      contract: vaNFTContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "337962950",
    },
    {
      name: "RepoVoterContract->add_to_whitelist(SlashingVoterContract)",
      contract: repoVoterContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "332353080",
    },
    {
      name: "KycVoterContract->add_to_whitelist(SlashingVoterContract)",
      contract: kycVoterContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "337162130",
    },
    {
      name: "BidEscrowContract->add_to_whitelist(SlashingVoterContract)",
      contract: bidEscrowContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "365125380",
    },
    {
      name: "AdminContract->add_to_whitelist(SlashingVoterContract)",
      contract: adminContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(slashingVoterContract.contractHash),
      }),
      payment: "332336090",
    },
    // 8) RepoVoterContract <- SimpleVoterContract
    //    ReputationContract <- SimpleVoterContract
    {
      name: "RepoVoterContract->add_to_whitelist(SimpleVoterContract)",
      contract: repoVoterContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(simpleVoterContract.contractHash),
      }),
      payment: "332353080",
    },
    {
      name: "ReputationContract->add_to_whitelist(SimpleVoterContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(simpleVoterContract.contractHash),
      }),
      payment: "336074520",
    },
    // 9) ReputationContract <- BidEscrowContract
    //    VaNftCo?tract <- BidEscrowContract
    //    SlashingVoterContract <- (update_bid_escrow_list) <- BidEscrowContract
    {
      name: "ReputationContract->add_to_whitelist(BidEscrowContract)",
      contract: repoVoterContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(bidEscrowContract.contractHash),
      }),
      payment: "332353080",
    },
    {
      name: "VaNftContract->add_to_whitelist(BidEscrowContract)",
      contract: vaNFTContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(bidEscrowContract.contractHash),
      }),
      payment: "337962950",
    },
    {
      name: "SlashingVoterContract->update_bid_escrow_list([BidEscrowContract])",
      contract: slashingVoterContract,
      entrypoint: "update_bid_escrow_list",
      args: RuntimeArgs.fromMap({
        bid_escrows: CLValueBuilder.list([stringToCLKey(bidEscrowContract.contractHash)]),
      }),
      payment: "114789450",
    },
    // 10) ReputationContract <- OnboardingRequestContract
    //     VaNftContract <- OnboardingRequestContract
    {
      name: "ReputationContract->add_to_whitelist(OnboardingRequestContract)",
      contract: reputationContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(onboardingRequestContract.contractHash),
      }),
      payment: "336074520",
    },
    {
      name: "VaNftContract->add_to_whitelist(OnboardingRequestContract)",
      contract: vaNFTContract,
      entrypoint: "add_to_whitelist",
      args: RuntimeArgs.fromMap({
        address: stringToCLKey(onboardingRequestContract.contractHash),
      }),
      payment: "337962950",
    },
  ];

  const preparedContractCalls = contractCallsConfig.map((c) => prepareContractCallDeploy(contractClient, c, status.chainspec_name, pk));

  const results = await Promise.all(preparedContractCalls.map(async (c) => {
    await rpcAPI.deploy(c.deploy);

    const processedDeploy = await waitForDeploy(rpcAPI, c.deploy.hash);

    const executionResult = parseExecutionResult(processedDeploy);

    return { ...c, actualCost: executionResult.cost, deployHash: processedDeploy.deploy.hash };
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
