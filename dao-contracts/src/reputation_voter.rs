use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address, ContractCall, DocumentHash, Mapping,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    voting::{voting::Voting, Ballot, Choice, GovernanceVoting},
    VotingConfigurationBuilder,
};

use crate::voting::types::VotingId;
use casper_dao_utils::casper_dao_macros::{CLTyped, Event, FromBytes, ToBytes};
use delegate::delegate;

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

    pub fn runtime_args(&self, account: Address, amount: U256) -> RuntimeArgs {
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
    pub amount: U256,
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
        amount: U256,
        document_hash: DocumentHash,
        stake: U256,
    );
    /// see [GovernanceVoting](GovernanceVoting::vote())
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId);
    /// see [GovernanceVoting](GovernanceVoting::get_dust_amount())
    fn get_dust_amount(&self) -> U256;
    /// see [GovernanceVoting](GovernanceVoting::get_variable_repo_address())
    fn get_variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_reputation_token_address())
    fn get_reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_voting())
    fn get_voting(&self, voting_id: VotingId) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting::get_ballot())
    fn get_ballot(&self, voting_id: VotingId, address: Address) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting::get_voter())
    fn get_voter(&self, voting_id: VotingId, at: u32) -> Option<Address>;
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
    fn create_voting(
        &mut self,
        account: Address,
        action: Action,
        amount: U256,
        document_hash: DocumentHash,
        stake: U256,
    ) {
        let voting_configuration = VotingConfigurationBuilder::defaults(&self.voting)
            .contract_call(ContractCall {
                address: self.voting.get_reputation_token_address(),
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

    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
            fn finish_voting(&mut self, voting_id: VotingId);
            fn get_dust_amount(&self) -> U256;
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_voting(&self, voting_id: VotingId) -> Option<Voting>;
            fn get_ballot(&self, voting_id: VotingId, address: Address) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, at: u32) -> Option<Address>;
        }
    }

    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256) {
        self.voting.vote(caller(), voting_id, choice, stake);
    }
}
