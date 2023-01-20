use casper_dao_contracts::{
    admin::{AdminContract, AdminVotingCreated},
    bid_escrow::BidEscrowContract,
    ids::DaoIdsContract,
    kyc_nft::KycNftContract,
    kyc_voter::{KycVoterContract, KycVotingCreated},
    onboarding_request::{OnboardingRequestContract, OnboardingVotingCreated},
    repo_voter::{RepoVoterContract, RepoVotingCreated},
    reputation::ReputationContract,
    reputation_voter::{ReputationVoterContract, ReputationVotingCreated},
    simple_voter::{SimpleVoterContract, SimpleVotingCreated},
    slashing_voter::{SlashingVoterContract, SlashingVotingCreated},
    va_nft::VaNftContract,
    variable_repository::VariableRepositoryContract,
    voting::{BallotCanceled, BallotCast, VotingCanceled, VotingEnded},
};
use casper_dao_modules::events::{
    AddedToWhitelist,
    OwnerChanged,
    RemovedFromWhitelist,
    ValueUpdated,
};
use casper_dao_utils::definitions::{ContractDef, ContractDefinition};

pub fn all_contracts() -> Vec<ContractDef> {
    let mut contracts = vec![
        // Core contracts.
        ReputationContract::contract_def(),
        variable_repository(),
        kyc_token(),
        va_token(),
        DaoIdsContract::contract_def(),
        // Voters.
        admin(),
        kyc_voter(),
        repo_voter(),
        reputation_voter(),
        slashing_voter(),
        simple_voter(),
        onboarding_request_voter(),
        // Bid Escrow.
        BidEscrowContract::contract_def(),
    ];
    with_access_control_events(&mut contracts);
    with_voting_events(&mut contracts);
    contracts
}

fn with_access_control_events(contracts: &mut [ContractDef]) {
    for contract in contracts.iter_mut() {
        contract.add_event::<AddedToWhitelist>("init");
        contract.add_event::<OwnerChanged>("init");
        contract.add_event::<AddedToWhitelist>("add_to_whitelist");
        contract.add_event::<RemovedFromWhitelist>("remove_from_whitelist");
        contract.add_event::<OwnerChanged>("change_ownership");
    }
}

fn with_voting_events(contracts: &mut [ContractDef]) {
    for contract in contracts.iter_mut() {
        contract.add_event::<BallotCast>("vote");
        contract.add_event::<BallotCanceled>("slash_voter");
        contract.add_event::<VotingCanceled>("slash_voter");
        contract.add_event::<VotingEnded>("finish_voting");
    }
}

// Core Contracts

fn variable_repository() -> ContractDef {
    VariableRepositoryContract::contract_def()
        .with_event::<ValueUpdated>("init")
        .with_event::<ValueUpdated>("update_at")
}

fn kyc_token() -> ContractDef {
    KycNftContract::contract_def()
        .with_event::<casper_dao_erc721::events::Transfer>("mint")
        .with_event::<casper_dao_erc721::events::Transfer>("burn")
}

fn va_token() -> ContractDef {
    VaNftContract::contract_def()
        .with_event::<casper_dao_erc721::events::Transfer>("mint")
        .with_event::<casper_dao_erc721::events::Transfer>("burn")
}

// Voters

fn admin() -> ContractDef {
    AdminContract::contract_def().with_event::<AdminVotingCreated>("create_voting")
}

fn kyc_voter() -> ContractDef {
    KycVoterContract::contract_def().with_event::<KycVotingCreated>("create_voting")
}

fn repo_voter() -> ContractDef {
    RepoVoterContract::contract_def().with_event::<RepoVotingCreated>("create_voting")
}

fn reputation_voter() -> ContractDef {
    ReputationVoterContract::contract_def().with_event::<ReputationVotingCreated>("create_voting")
}

fn slashing_voter() -> ContractDef {
    SlashingVoterContract::contract_def().with_event::<SlashingVotingCreated>("create_voting")
}

fn simple_voter() -> ContractDef {
    SimpleVoterContract::contract_def().with_event::<SimpleVotingCreated>("create_voting")
}

fn onboarding_request_voter() -> ContractDef {
    OnboardingRequestContract::contract_def().with_event::<OnboardingVotingCreated>("create_voting")
}

pub fn print_all_contracts() {
    let contracts = all_contracts();

    for contract in contracts {
        let methods = contract.mutable_methods();
        println!("\n{} ({})", contract.name, methods.len());
        for method in methods {
            println!("    - {} ({})", method.name, method.events.len());
            for event in method.events {
                println!("        - {}", event.name);
            }
        }
    }
}
