use std::{cmp::Ordering, str::FromStr};

use dao::configuration::get_variable;
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
use odra::casper::casper_types::account::AccountHash;
use odra::types::Balance;
use odra::types::BlockTime;
use odra::types::OdraType;
use odra::{client_env, types::Address};

use crate::{
    cspr, error::Error, log, DaoSnapshot, DeployedContractsToml, DEFAULT_CSPR_USD_RATE,
    DEPLOYED_CONTRACTS_FILE,
};
use dao::utils::variable_type::VariableType;

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
    client_env::set_gas(cspr(110));
    let rate_provider = CSPRRateProviderContractDeployer::init(DEFAULT_CSPR_USD_RATE.into());
    contracts.add_contract("CSPRRateProviderContract", rate_provider.address());

    // Deploy Variable Repository.
    client_env::set_gas(cspr(200));
    let variable_repository = VariableRepositoryContractDeployer::init(
        *rate_provider.address(),
        multisig_wallet,
        *ids.address(),
    );
    contracts.add_contract("VariableRepositoryContract", variable_repository.address());

    // Deploy Reputation Token.
    client_env::set_gas(cspr(200));
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
    client_env::set_gas(cspr(500));
    let admin = AdminContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("AdminContract", admin.address());

    // Deploy Reputation Voter.
    client_env::set_gas(cspr(500));
    let reputation_voter = ReputationVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("ReputationVoterContract", reputation_voter.address());

    // Deploy KYC Voter.
    client_env::set_gas(cspr(600));
    let kyc_voter = KycVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
        *kyc_token.address(),
    );
    contracts.add_contract("KycVoterContract", kyc_voter.address());

    // Deploy Repo Voter.
    client_env::set_gas(cspr(600));
    let repo_voter = RepoVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("RepoVoterContract", repo_voter.address());

    // Deploy Simple Voter.
    client_env::set_gas(cspr(500));
    let simple_voter = SimpleVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("SimpleVoterContract", simple_voter.address());

    // Deploy Slashing Voter.
    client_env::set_gas(cspr(600));
    let slashing_voter = SlashingVoterContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("SlashingVoterContract", slashing_voter.address());

    // Deploy BidEscrow.
    client_env::set_gas(cspr(800));
    let bid_escrow = BidEscrowContractDeployer::init(
        *variable_repository.address(),
        *reputation_token.address(),
        *kyc_token.address(),
        *va_token.address(),
    );
    contracts.add_contract("BidEscrowContract", bid_escrow.address());

    // Deploy Onboarding.
    client_env::set_gas(cspr(600));
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
        match VariableType::from_key(name.as_str()) {
            VariableType::Balance => log::info(format!(
                "{}: {}",
                name,
                get_variable::<Balance>(name.as_str(), &variables)
            )),
            VariableType::BlockTime => log::info(format!(
                "{}: {}",
                name,
                get_variable::<BlockTime>(name.as_str(), &variables)
            )),
            VariableType::Address => log::info(format!(
                "{}: {:?}",
                name,
                get_variable::<Address>(name.as_str(), &variables)
            )),
            VariableType::Bool => log::info(format!(
                "{}: {:?}",
                name,
                get_variable::<bool>(name.as_str(), &variables)
            )),
            VariableType::Unknown => log::info(format!("Unknown variable type: {}", name)),
        }
    }
}

pub fn set_variable(name: &str, value: &str) {
    let mut dao = DaoSnapshot::load();
    client_env::set_gas(cspr(5));

    match VariableType::from_key(name) {
        VariableType::Balance => {
            dao.variable_repository.update_at(
                name.to_string(),
                Balance::from(value.parse::<u64>().unwrap())
                    .serialize()
                    .unwrap()
                    .into(),
                None,
            );
            log::info(format!("Updated {} to {}", name, value));
        }
        VariableType::BlockTime => {
            dao.variable_repository.update_at(
                name.to_string(),
                value
                    .parse::<BlockTime>()
                    .unwrap()
                    .serialize()
                    .unwrap()
                    .into(),
                None,
            );
            log::info(format!("Updated {} to {}", name, value));
        }
        VariableType::Address => {
            dao.variable_repository.update_at(
                name.to_string(),
                value
                    .parse::<Address>()
                    .unwrap()
                    .serialize()
                    .unwrap()
                    .into(),
                None,
            );
            log::info(format!("Updated {} to {}", name, value));
        }
        VariableType::Bool => {
            dao.variable_repository.update_at(
                name.to_string(),
                value.parse::<bool>().unwrap().serialize().unwrap().into(),
                None,
            );
            log::info(format!("Updated {} to {}", name, value));
        }
        VariableType::Unknown => {
            log::info(format!("Unknown variable: {}", name));
        }
    }
}

pub fn balance_of(account_hash: &str) {
    let account: Address = Address::Account(AccountHash::from_formatted_str(account_hash).unwrap());
    let dao = DaoSnapshot::load();
    let balance = dao.reputation_token.balance_of(account);
    log::info(format!("{} has {} reputation.", account_hash, balance));
}

pub fn stake_of(account_hash: &str) {
    let account: Address = Address::Account(AccountHash::from_formatted_str(account_hash).unwrap());
    let dao = DaoSnapshot::load();
    let stake = dao.reputation_token.get_stake(account);
    log::info(format!("{} has {} stake.", account_hash, stake));
}

pub fn get_voting(voting_id: &str, contract: &str) {
    let dao = DaoSnapshot::load();
    let voting_id = voting_id.parse::<u32>().unwrap();
    let voting = match contract {
        "bid_escrow" => dao.bid_escrow.get_voting(voting_id),
        "kyc_voter" => dao.kyc_voter.get_voting(voting_id),
        "slashing_voter" => dao.slashing_voter.get_voting(voting_id),
        "reputation_voter" => dao.reputation_voter.get_voting(voting_id),
        "repo_voter" => dao.repo_voter.get_voting(voting_id),
        "simple_voter" => dao.simple_voter.get_voting(voting_id),
        "onboarding" => dao.onboarding.get_voting(voting_id),
        "admin" => dao.admin.get_voting(voting_id),
        _ => panic!("Unknown contract: {}", contract),
    };
    log::info(format!("Voting: {:?}", voting));
}

pub fn get_account(account_hash: &str) {
    let account: Address = Address::Account(AccountHash::from_formatted_str(account_hash).unwrap());
    let dao = DaoSnapshot::load();
    let stake = dao.reputation_token.get_stake(account);
    let reputation = dao.reputation_token.balance_of(account);
    let kyc_token = dao.kyc_token.balance_of(&account);
    let va_token = dao.va_token.balance_of(&account);
    log::info(format!(
        "Account {}:\n\tstake: {}\n\treputation: {}\n\tkyc_token: {}\n\tva_token: {}",
        account_hash, stake, reputation, kyc_token, va_token
    ));
}
