use dao::voting_contracts::ReputationAction;
use dao::{utils::types::DocumentHash, voting_contracts::AdminAction};
use odra::types::{Address, BlockTime, Bytes};

use crate::common::params::ReputationBalance;
use crate::common::{
    helpers::{to_milliseconds, value_to_bytes},
    params::{voting::Voting, Account, Contract},
    DaoWorld,
};

pub fn build(world: &DaoWorld, voting: Voting) -> VotingSetup {
    match voting.contract {
        Contract::Admin => {
            let contract_to_update = voting.get_parsed_arg::<Contract>(0);
            let contract_to_update = world.get_contract_address(&contract_to_update);

            let action = voting.get_parsed_arg::<String>(1);
            let action = match action.as_str() {
                "add_to_whitelist" => AdminAction::AddToWhitelist,
                "remove_from_whitelist" => AdminAction::RemoveFromWhitelist,
                "propose_new_owner" => AdminAction::ProposeNewOwner,
                unknown => panic!("{:?} is not a valid action", unknown),
            };

            let address = voting.get_parsed_arg::<Account>(2);
            let address = world.get_address(&address);

            VotingSetup::Admin(contract_to_update, action, address)
        }
        Contract::KycVoter => {
            let subject_address = voting.get_parsed_arg::<Account>(0);
            let subject_address = world.get_address(&subject_address);

            VotingSetup::Kyc(subject_address, DocumentHash::default())
        }
        Contract::SlashingVoter => {
            let address_to_slash = voting.get_parsed_arg::<Account>(0);
            let address_to_slash = world.get_address(&address_to_slash);

            let slash_ratio = voting.get_parsed_arg::<f32>(1);

            VotingSetup::Slasher(address_to_slash, (slash_ratio * 1000.0) as u32)
        }
        Contract::RepoVoter => {
            let variable_repository_address = voting.get_parsed_arg::<Account>(0);
            let variable_repository_address = world.get_address(&variable_repository_address);

            let key = voting.get_parsed_arg::<String>(1);

            let value = voting.get_parsed_arg::<String>(2);
            let value = value_to_bytes(&value, &key);

            let activation_time = voting.get_parsed_arg_or_none::<String>(3).map(|s| {
                let values = s.split(' ').collect::<Vec<_>>();
                let value = values.first().and_then(|s| s.parse().ok()).unwrap();
                let unit = values.get(1).and_then(|s| s.parse().ok()).unwrap();
                to_milliseconds(value, unit)
            });

            VotingSetup::Repository(variable_repository_address, key, value, activation_time)
        }
        Contract::SimpleVoter => VotingSetup::Simple(Default::default()),
        Contract::ReputationVoter => {
            let recipient_address = voting.get_parsed_arg::<Account>(0);
            let recipient_address = world.get_address(&recipient_address);

            let action = voting.get_parsed_arg::<String>(1);
            let action = match action.as_str() {
                "mint" => ReputationAction::Mint,
                "burn" => ReputationAction::Burn,
                unknown => panic!("{:?} is not a valid action", unknown),
            };

            let amount = voting.get_parsed_arg::<ReputationBalance>(2);

            VotingSetup::Reputation(recipient_address, action, amount, Default::default())
        }
        contract => panic!("{:?} is not a voting contract", contract),
    }
}

#[derive(Debug)]
pub enum VotingSetup {
    Admin(Address, AdminAction, Address),
    Kyc(Address, DocumentHash),
    Slasher(Address, u32),
    Repository(Address, String, Bytes, Option<BlockTime>),
    Simple(DocumentHash),
    Reputation(Address, ReputationAction, ReputationBalance, DocumentHash),
}
