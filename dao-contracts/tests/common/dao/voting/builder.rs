use casper_dao_contracts::{
    action::Action as AdminAction,
    reputation_voter::Action as ReputationAction,
};
use casper_dao_utils::{Address, BlockTime, DocumentHash};
use casper_types::bytesrepr::{Bytes, ToBytes};

use crate::{
    common::{
        params::{
            voting::{Ballot, Voting, VotingType},
            Account,
            Balance,
            Contract,
        },
        DaoWorld,
    },
    on_voting_contract,
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
                "change_ownership" => AdminAction::ChangeOwner,
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
            let value = Bytes::from(0.to_bytes().unwrap_or_default());

            VotingSetup::Repository(variable_repository_address, key, value, None)
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

            let amount = voting.get_parsed_arg::<Balance>(2);

            VotingSetup::Reputation(recipient_address, action, amount, Default::default())
        }
        contract => panic!("{:?} is not a voting contract", contract),
    }
}

pub enum VotingSetup {
    Admin(Address, AdminAction, Address),
    Kyc(Address, DocumentHash),
    Slasher(Address, u32),
    Repository(Address, String, Bytes, Option<BlockTime>),
    Simple(DocumentHash),
    Reputation(Address, ReputationAction, Balance, DocumentHash),
}