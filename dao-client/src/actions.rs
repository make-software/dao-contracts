use dao::{
    bid_escrow::contract::BidEscrowContractDeployer,
    core_contracts::{
        KycNftContractDeployer, ReputationContractDeployer, VaNftContractDeployer,
        VariableRepositoryContractDeployer,
    },
    utils_contracts::{CSPRRateProviderContractDeployer, DaoIdsContractDeployer},
    voting_contracts::{
        AdminContractDeployer, KycVoterContractDeployer, OnboardingRequestContractDeployer,
        RepoVoterContractDeployer, ReputationVoterContractDeployer, SimpleVoterContractDeployer,
        SlashingVoterContractDeployer,
    },
};
use odra::{client_env, types::Address};

use crate::{
    cspr, DaoSnapshot, DeployedContractsToml, DEFAULT_CSPR_USD_RATE, DEPLOYED_CONTRACTS_FILE,
};

/// Deploy all DAO contracts.
pub fn deploy_all() {
    DeployedContractsToml::handle_previous_version();
    let mut contracts = DeployedContractsToml::new();

    // Mutisig wallet
    let owner = client_env::caller();
    let multisig_wallet = owner;

    // Deploy Ids.
    client_env::set_gas(cspr(100));
    let ids = DaoIdsContractDeployer::init();
    contracts.add_contract("DaoIdsContract", ids.address());

    // Deploy CSPR Rate Provider.
    client_env::set_gas(cspr(90));
    let rate_provider = CSPRRateProviderContractDeployer::init(DEFAULT_CSPR_USD_RATE.into());
    contracts.add_contract("CSPRRateProviderContract", rate_provider.address());

    // Deploy Variable Repository.
    client_env::set_gas(cspr(230));
    let variable_repository = VariableRepositoryContractDeployer::init(
        *rate_provider.address(),
        multisig_wallet,
        *ids.address(),
    );
    contracts.add_contract("VariableRepositoryContract", variable_repository.address());

    // Deploy Reputation Token.
    client_env::set_gas(cspr(180));
    let reputation_token = ReputationContractDeployer::init();
    contracts.add_contract("ReputationContract", reputation_token.address());

    // Deploy KYC NFT.
    client_env::set_gas(cspr(200));
    let kyc_token =
        KycNftContractDeployer::init("kyc_token".to_string(), "KYC".to_string(), "".to_string());
    contracts.add_contract("KycNftContract", kyc_token.address());

    // Deploy VA NFT.
    client_env::set_gas(cspr(200));
    let va_token =
        VaNftContractDeployer::init("va_token".to_string(), "VAT".to_string(), "".to_string());
    contracts.add_contract("VaNftContract", va_token.address());

    // Deploy Admin.
    client_env::set_gas(cspr(350));
    let admin = AdminContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("AdminContract", admin.address());

    // Deploy Reputation Voter.
    client_env::set_gas(cspr(350));
    let reputation_voter = ReputationVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("ReputationVoterContract", reputation_voter.address());

    // Deploy KYC Voter.
    client_env::set_gas(cspr(350));
    let kyc_voter = KycVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
        *kyc_token.address(),
    );
    contracts.add_contract("KycVoterContract", kyc_voter.address());

    // Deploy Repo Voter.
    client_env::set_gas(cspr(350));
    let repo_voter = RepoVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("RepoVoterContract", repo_voter.address());

    // Deploy Simple Voter.
    client_env::set_gas(cspr(350));
    let simple_voter = SimpleVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("SimpleVoterContract", simple_voter.address());

    // Deploy Slashing Voter.
    client_env::set_gas(cspr(370));
    let slashing_voter = SlashingVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("SlashingVoterContract", slashing_voter.address());

    // Deploy BidEscrow.
    client_env::set_gas(cspr(550));
    let bid_escrow = BidEscrowContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *kyc_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("BidEscrowContract", bid_escrow.address());

    // Deploy Onboarding.
    client_env::set_gas(cspr(800));
    let onboarding = OnboardingRequestContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *kyc_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("OnboardingRequestContract", onboarding.address());
}

macro_rules! whitelist_all {
    ( $( $source:ident => [$( $target:ident ),+] ),+ ) => {
        $(
            $(
                client_env::set_gas(cspr(5));
                $source.add_to_whitelist(*$target.address());
            )+
        )+
    }
}

/// Whitelist neccessary contracts.
pub fn whitelist() {
    let DaoSnapshot {
        mut ids,
        mut admin,
        mut variable_repository,
        mut reputation_token,
        mut va_token,
        mut kyc_token,
        mut kyc_voter,
        mut slashing_voter,
        mut reputation_voter,
        mut repo_voter,
        mut simple_voter,
        mut bid_escrow,
        mut onboarding,
        ..
    } = DaoSnapshot::load();

    whitelist_all!(
        ids => [admin, kyc_voter, slashing_voter, repo_voter, reputation_voter, simple_voter, bid_escrow, onboarding],
        variable_repository => [repo_voter],
        reputation_token => [admin, repo_voter, reputation_voter, kyc_voter, slashing_voter, simple_voter, bid_escrow, onboarding],
        va_token => [slashing_voter, bid_escrow, onboarding],
        kyc_token => [kyc_voter],
        admin => [slashing_voter],
        kyc_voter => [slashing_voter],
        onboarding => [slashing_voter],
        repo_voter => [slashing_voter, simple_voter],
        reputation_voter => [slashing_voter],
        simple_voter => [slashing_voter],
        slashing_voter => [slashing_voter],
        bid_escrow => [slashing_voter]
    );
}

/// Add slashable contracts to slashing voter.
pub fn setup_slashing_voter() {
    let mut dao = DaoSnapshot::load();

    let slashable_contracts: Vec<Address> = vec![
        dao.admin.address(),
        dao.kyc_voter.address(),
        dao.onboarding.address(),
        dao.repo_voter.address(),
        dao.reputation_voter.address(),
        dao.simple_voter.address(),
        dao.slashing_voter.address(),
        dao.bid_escrow.address(),
    ]
    .into_iter()
    .cloned()
    .collect();

    client_env::set_gas(cspr(20));
    dao.slashing_voter
        .update_slashable_contracts(slashable_contracts);
}

/// Print addresses of deployed contracts.
pub fn print_addresses() {
    println!("Addresses from {} file:\n", DEPLOYED_CONTRACTS_FILE);
    for contract in DeployedContractsToml::load().unwrap().contracts {
        println!("{:30}: {}", contract.name, contract.package_hash);
    }
}
