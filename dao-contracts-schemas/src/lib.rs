use casper_dao_contracts::{
    voting::BallotCast,
    AdminContract,
    BidEscrowContract,
    KycNftContract,
    KycVoterContract,
    RepoVoterContract,
    ReputationContract,
    SlashingVoterContract,
    VaNftContract,
    VariableRepositoryContract,
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
        ReputationContract::contract_def(),
        variable_repository(),
        KycNftContract::contract_def(),
        VaNftContract::contract_def(),
        AdminContract::contract_def(),
        KycVoterContract::contract_def(),
        RepoVoterContract::contract_def(),
        ReputationContract::contract_def(),
        SlashingVoterContract::contract_def(),
        BidEscrowContract::contract_def(),
    ];
    with_access_control_events(&mut contracts);
    with_voting_events(&mut contracts);
    contracts
}

fn with_access_control_events(contracts: &mut Vec<ContractDef>) {
    for contract in contracts.iter_mut() {
        contract.add_event::<AddedToWhitelist>("init");
        contract.add_event::<OwnerChanged>("init");
        contract.add_event::<AddedToWhitelist>("add_to_whitelist");
        contract.add_event::<RemovedFromWhitelist>("remove_from_whitelist");
        contract.add_event::<OwnerChanged>("change_ownership");
    }
}

fn with_voting_events(contracts: &mut Vec<ContractDef>) {
    for contract in contracts.iter_mut() {
        contract.add_event::<BallotCast>("vote");
    }
}

fn variable_repository() -> ContractDef {
    let mut contract = VariableRepositoryContract::contract_def();
    contract.add_event::<ValueUpdated>("init");
    contract.add_event::<ValueUpdated>("update_at");
    contract
}

pub fn print_all_contracts() {
    let contracts = all_contracts();

    for contract in contracts {
        let methods = contract.mutable_methods();
        println!("\n{} ({})", contract.ident, methods.len());
        for method in methods {
            println!("    - {} ({})", method.ident, method.events.len());
            for event in method.events {
                println!("        - {}", event.ident);
            }
        }
    }
}
