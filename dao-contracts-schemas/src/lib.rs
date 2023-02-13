use casper_dao_contracts::{
    admin::AdminContract,
    bid_escrow::BidEscrowContract,
    ids::DaoIdsContract,
    kyc_nft::KycNftContract,
    kyc_voter::KycVoterContract,
    onboarding_request::OnboardingRequestContract,
    rate_provider::CSPRRateProviderContract,
    repo_voter::RepoVoterContract,
    reputation::ReputationContract,
    reputation_voter::ReputationVoterContract,
    simple_voter::SimpleVoterContract,
    slashing_voter::SlashingVoterContract,
    va_nft::VaNftContract,
    variable_repository::VariableRepositoryContract,
};
use casper_dao_utils::definitions::{ContractDef, ContractDefinition};

pub fn all_contracts() -> Vec<ContractDef> {
    vec![
        // Core contracts.
        reputation(),
        variable_repository(),
        kyc_token(),
        va_token(),
        ids(),
        rate_provider(),
        // Voters.
        admin(),
        kyc_voter(),
        repo_voter(),
        reputation_voter(),
        slashing_voter(),
        simple_voter(),
        onboarding_request_voter(),
        // Bid Escrow.
        bid_escrow(),
    ]
}

// Core Contracts.

fn reputation() -> ContractDef {
    ReputationContract::contract_def()
        .with_events(casper_dao_contracts::reputation::event_schemas())
}

fn variable_repository() -> ContractDef {
    VariableRepositoryContract::contract_def()
        .with_events(casper_dao_contracts::variable_repository::event_schemas())
}

fn kyc_token() -> ContractDef {
    KycNftContract::contract_def().with_events(casper_dao_contracts::kyc_nft::event_schemas())
}

fn va_token() -> ContractDef {
    VaNftContract::contract_def().with_events(casper_dao_contracts::va_nft::event_schemas())
}

fn ids() -> ContractDef {
    DaoIdsContract::contract_def().with_events(casper_dao_contracts::ids::event_schemas())
}

fn rate_provider() -> ContractDef {
    CSPRRateProviderContract::contract_def()
        .with_events(casper_dao_contracts::rate_provider::event_schemas())
}

// Voters.

fn admin() -> ContractDef {
    AdminContract::contract_def().with_events(casper_dao_contracts::admin::event_schemas())
}

fn kyc_voter() -> ContractDef {
    KycVoterContract::contract_def().with_events(casper_dao_contracts::kyc_voter::event_schemas())
}

fn repo_voter() -> ContractDef {
    RepoVoterContract::contract_def().with_events(casper_dao_contracts::repo_voter::event_schemas())
}

fn reputation_voter() -> ContractDef {
    ReputationVoterContract::contract_def()
        .with_events(casper_dao_contracts::reputation_voter::event_schemas())
}

fn slashing_voter() -> ContractDef {
    SlashingVoterContract::contract_def()
        .with_events(casper_dao_contracts::slashing_voter::event_schemas())
}

fn simple_voter() -> ContractDef {
    SimpleVoterContract::contract_def()
        .with_events(casper_dao_contracts::simple_voter::event_schemas())
}

fn onboarding_request_voter() -> ContractDef {
    OnboardingRequestContract::contract_def()
        .with_events(casper_dao_contracts::onboarding_request::event_schemas())
}

// Bid Escrow.

fn bid_escrow() -> ContractDef {
    BidEscrowContract::contract_def().with_events(casper_dao_contracts::bid_escrow::event_schemas())
}
