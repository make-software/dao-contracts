use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, CLTyped, Event, FromBytes, Instance, ToBytes},
    casper_env::caller,
    Address,
    ContractCall,
    DocumentHash,
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
    DaoConfigurationBuilder,
};

/// Action to perform against reputation
#[derive(CLTyped, PartialEq, Eq, Debug, FromBytes, ToBytes, Clone)]
pub enum Action {
    Burn,
    Mint,
}

impl Action {
    pub fn entrypoint(&self) -> String {
        match self {
            Action::Burn => "burn".to_string(),
            Action::Mint => "mint".to_string(),
        }
    }

    pub fn runtime_args(&self, account: Address, amount: U512) -> RuntimeArgs {
        match self {
            Action::Burn => {
                runtime_args! {
                    "owner" => account,
                    "amount" => amount,
                }
            }
            Action::Mint => {
                runtime_args! {
                    "recipient" => account,
                    "amount" => amount,
                }
            }
        }
    }
}

/// Struct storing all information about reputation voting
#[derive(CLTyped, PartialEq, Eq, Debug, FromBytes, ToBytes, Clone)]
pub struct ReputationVoting {
    pub action: Action,
    pub account: Address,
    pub amount: U512,
    pub document_hash: DocumentHash,
}

/// An event thrown when new reputation voting starts
#[derive(Debug, PartialEq, Eq, Event)]
pub struct ReputationVotingCreated {
    pub reputation_voting: ReputationVoting,
    pub voting_id: VotingId,
}

#[casper_contract_interface]
pub trait ReputationVoterContractInterface {
    /// see [GovernanceVoting](GovernanceVoting::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
    /// Creates new ReputationVoter voting.
    ///
    /// `account` - subject of voting
    /// `action` - action to perform (burn/mint)
    /// `amount` - how many tokens to burn/mint
    /// `document_hash` - hash of the document explaining an action
    fn create_voting(
        &mut self,
        account: Address,
        action: Action,
        amount: U512,
        document_hash: DocumentHash,
        stake: U512,
    );
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

/// ReputationVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[derive(Instance)]
pub struct ReputationVoterContract {
    voting: GovernanceVoting,
    reputation_votings: Mapping<VotingId, ReputationVoting>,
}

impl ReputationVoterContractInterface for ReputationVoterContract {
    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
            fn get_dust_amount(&self) -> U512;
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn create_voting(
        &mut self,
        account: Address,
        action: Action,
        amount: U512,
        document_hash: DocumentHash,
        stake: U512,
    ) {
        let voting_configuration = DaoConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .contract_call(ContractCall {
            address: self.voting.reputation_token_address(),
            entry_point: action.entrypoint(),
            runtime_args: action.runtime_args(account, amount),
        })
        .build();

        let voting_id = self
            .voting
            .create_voting(caller(), stake, voting_configuration);

        let reputation_voting = ReputationVoting {
            action,
            account,
            amount,
            document_hash,
        };

        self.reputation_votings
            .set(&voting_id, reputation_voting.clone());

        ReputationVotingCreated {
            reputation_voting,
            voting_id,
        }
        .emit();
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
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
