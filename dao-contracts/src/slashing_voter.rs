use casper_dao_utils::{
    casper_contract::contract_api::runtime::revert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
    ContractCall,
    Error,
    Mapping,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    voting::{
        types::VotingId,
        voting::{Voting, VotingType},
        Ballot,
        Choice,
        GovernanceVoting,
    },
    ConfigurationBuilder,
    ReputationContractInterface,
};

#[casper_contract_interface]
pub trait SlashingVoterContractInterface {
    /// see [GovernanceVoting](GovernanceVoting::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);

    fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512);
    /// see [GovernanceVoting](GovernanceVoting::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// see [GovernanceVoting](GovernanceVoting::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [GovernanceVoting](GovernanceVoting::get_dust_amount())
    fn get_dust_amount(&self) -> U512;
    /// see [GovernanceVoting](GovernanceVoting::get_variable_repo_address())
    fn variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_voting())
    fn get_voting(&self, voting_id: VotingId, voting_type: VotingType) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId);
}

/// Slashing Voter contract uses [GovernanceVoting](GovernanceVoting) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Slashing Voter contract needs to have permissions to perform those actions.
///
/// For details see [SlashingVoterContractInterface](SlashingVoterContractInterface)
#[derive(Instance)]
pub struct SlashingVoterContract {
    voting: GovernanceVoting,
    subjects: Mapping<VotingId, Address>,
}

impl SlashingVoterContractInterface for SlashingVoterContract {
    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
            fn get_dust_amount(&self) -> U512;
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512) {
        // TODO: contraints
        let current_reputation = self.voting.reputation_token().balance_of(address_to_slash);

        let contract_calls = match slash_ratio {
            1000 => {
                vec![
                    ContractCall {
                        address: self.voting.va_token_address(),
                        entry_point: "burn".to_string(),
                        runtime_args: runtime_args! {
                            "owner" => address_to_slash,
                        },
                    },
                    ContractCall {
                        address: self.voting.reputation_token_address(),
                        entry_point: "burn_all".to_string(),
                        runtime_args: runtime_args! {
                            "owner" => address_to_slash,
                        },
                    },
                ]
            }
            slash_ratio if slash_ratio < 1000 => vec![ContractCall {
                address: self.voting.reputation_token_address(),
                entry_point: "burn".to_string(),
                runtime_args: runtime_args! {
                    "owner" => address_to_slash,
                    "amount" => current_reputation * slash_ratio / 1000,
                },
            }],
            _ => {
                // TODO: come up with clever error
                revert(666);
            }
        };

        let voting_configuration = ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .contract_calls(contract_calls)
        .build();

        let creator = caller();
        let voting_id = self
            .voting
            .create_voting(creator, stake, voting_configuration);

        self.subjects.set(&voting_id, address_to_slash);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        // Check if the caller is not a subject for the voting.
        let address_to_slash = self.subjects.get_or_revert(&voting_id);
        if caller() == address_to_slash {
            revert(Error::SubjectOfSlashing);
        }
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_voting(&self, voting_id: VotingId, voting_type: VotingType) -> Option<Voting> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_voting(voting_id)
    }

    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_ballot(voting_id, address)
    }

    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_voter(voting_id, at)
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.finish_voting(voting_id);
    }

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.voting.cancel_voter(voter, voting_id);
    }
}
