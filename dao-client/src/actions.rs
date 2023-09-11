use std::{cmp::Ordering, str::FromStr};

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
use odra::types::BlockTime;
use odra::types::Balance;
use odra::types::OdraType;
use dao::configuration::get_variable;
use dao::utils::consts::*;

use crate::{
    cspr, error::Error, log, DaoSnapshot, DeployedContractsToml, DEFAULT_CSPR_USD_RATE,
    DEPLOYED_CONTRACTS_FILE,
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
    log::info(format!(
        "Addresses from {} file:\n",
        DEPLOYED_CONTRACTS_FILE
    ));
    for contract in DeployedContractsToml::load().unwrap().contracts {
        log::info(format!("{:30}: {}", contract.name, contract.package_hash));
    }
}

/// Setup the VA with reputation, kyc and va tokens.
pub fn setup_va(account_hash: &str, reputation_amount: u64) {
    let va = Address::from_str(account_hash)
        .unwrap_or_else(|_| Error::InvalidAccount(account_hash.to_string()).print_and_die());

    let reputation_amount = odra::types::Balance::from(reputation_amount);
    let mut dao = DaoSnapshot::load();

    log::info(format!(
        "Setting up VA {} with {} reputation.",
        va.to_string(),
        reputation_amount
    ));

    // Check if the account has KYC token.
    if dao.kyc_token.balance_of(&va).is_zero() {
        log::info("VA doesn't have a KYC Token. Minting 1 KYC Token.");
        client_env::set_gas(cspr(10));
        dao.kyc_token.mint(va);
    } else {
        log::info("VA already has a KYC Token.");
    }

    // Check if the account has VA token.
    if dao.va_token.balance_of(&va).is_zero() {
        log::info("VA doesn't have a VA Token. Minting 1 VA Token.");
        client_env::set_gas(cspr(10));
        dao.va_token.mint(va);
    } else {
        log::info("VA already has a VA Token.");
    }

    // Set the reputation.
    let current_reputation = dao.reputation_token.balance_of(va);
    match current_reputation.cmp(&reputation_amount) {
        Ordering::Less => {
            let reputation_to_mint = reputation_amount - current_reputation;
            log::info(format!(
                "VA has less reputation than needed. Minting {} reputation.",
                reputation_to_mint
            ));
            client_env::set_gas(cspr(10));
            dao.reputation_token.mint(va, reputation_to_mint);
        }
        Ordering::Equal => {
            log::info("VA already has the correct amount of reputation.");
        }
        Ordering::Greater => {
            let reputation_to_burn = current_reputation - reputation_amount;
            log::info(format!(
                "VA has more reputation than needed. Burning {} reputation.",
                reputation_to_burn
            ));
            client_env::set_gas(cspr(10));
            dao.reputation_token
                .burn(va, current_reputation - reputation_amount);
        }
    }
}

pub fn print_variables() {
    let dao = DaoSnapshot::load();
    let variables = dao.variable_repository.all_variables();

    // print all variables
    for (name, _) in variables.clone() {
        match name.as_str() {
            POST_JOB_DOS_FEE | DEFAULT_POLICING_RATE | REPUTATION_CONVERSION_RATE |
            BID_ESCROW_INFORMAL_QUORUM_RATIO | BID_ESCROW_FORMAL_QUORUM_RATIO |
            INFORMAL_QUORUM_RATIO | FORMAL_QUORUM_RATIO |
            DEFAULT_REPUTATION_SLASH | VOTING_CLEARNESS_DELTA | BID_ESCROW_PAYMENT_RATIO => {
                log::info(format!("{}: {}", name, get_variable::<Balance>(name.as_str(), &variables)));
            }
            INTERNAL_AUCTION_TIME | PUBLIC_AUCTION_TIME | BID_ESCROW_INFORMAL_VOTING_TIME |
            BID_ESCROW_FORMAL_VOTING_TIME | INFORMAL_VOTING_TIME | FORMAL_VOTING_TIME |
            TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING | VA_BID_ACCEPTANCE_TIMEOUT |
            VOTING_START_AFTER_JOB_WORKER_SUBMISSION => {
                log::info(format!("{}: {}", name, get_variable::<BlockTime>(name.as_str(), &variables)));
            }
            FIAT_CONVERSION_RATE_ADDRESS | BID_ESCROW_WALLET_ADDRESS | VOTING_IDS_ADDRESS=> {
                log::info(format!("{}: {:?}", name, get_variable::<Address>(name.as_str(), &variables)));
            }
            FORUM_KYC_REQUIRED | INFORMAL_STAKE_REPUTATION | VA_CAN_BID_ON_PUBLIC_AUCTION |
            DISTRIBUTE_PAYMENT_TO_NON_VOTERS => {
                log::info(format!("{}: {:?}", name, get_variable::<bool>(name.as_str(), &variables)));
            }

            _ => {
                log::info(format!("Unknown variable type: {}", name));
            }
        }
    }
}

pub fn set_variable(name: &str, value: &str) {
    let mut dao = DaoSnapshot::load();
        client_env::set_gas(cspr(5));

    match name {
        POST_JOB_DOS_FEE | DEFAULT_POLICING_RATE | REPUTATION_CONVERSION_RATE |
        BID_ESCROW_INFORMAL_QUORUM_RATIO | BID_ESCROW_FORMAL_QUORUM_RATIO |
        INFORMAL_QUORUM_RATIO | FORMAL_QUORUM_RATIO |
        DEFAULT_REPUTATION_SLASH | VOTING_CLEARNESS_DELTA | BID_ESCROW_PAYMENT_RATIO => {
            dao.variable_repository.update_at(name.to_string(), Balance::from(value.parse::<u64>().unwrap()).serialize().unwrap().into(), None);
            log::info(format!("Updated {} to {}", name,  value));
        }
        INTERNAL_AUCTION_TIME | PUBLIC_AUCTION_TIME | BID_ESCROW_INFORMAL_VOTING_TIME |
        BID_ESCROW_FORMAL_VOTING_TIME | INFORMAL_VOTING_TIME | FORMAL_VOTING_TIME |
        TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING | VA_BID_ACCEPTANCE_TIMEOUT |
        VOTING_START_AFTER_JOB_WORKER_SUBMISSION => {
            dao.variable_repository.update_at(name.to_string(), value.parse::<BlockTime>().unwrap().serialize().unwrap().into(), None);
            log::info(format!("Updated {} to {}", name,  value));
        }
        FIAT_CONVERSION_RATE_ADDRESS | BID_ESCROW_WALLET_ADDRESS | VOTING_IDS_ADDRESS=> {
            dao.variable_repository.update_at(name.to_string(), value.parse::<Address>().unwrap().serialize().unwrap().into(), None);
            log::info(format!("Updated {} to {}", name,  value));
        }
        FORUM_KYC_REQUIRED | INFORMAL_STAKE_REPUTATION | VA_CAN_BID_ON_PUBLIC_AUCTION |
        DISTRIBUTE_PAYMENT_TO_NON_VOTERS => {
            dao.variable_repository.update_at(name.to_string(), value.parse::<bool>().unwrap().serialize().unwrap().into(), None);
            log::info(format!("Updated {} to {}", name,  value));
        }

        _ => {
            log::info(format!("Unknown variable: {}", name));
        }
    }
}
