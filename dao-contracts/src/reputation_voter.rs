//! Contains Reputation Voter Contract definition and related abstractions.
//!
//! # General
//! The contract is used to operate on the [Reputation Token contract].
//!
//! Two types of voting can be created:
//! * to `mint` tokens for a user,
//! * to `burn` users' tokens.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! [Reputation Token contract]: crate::variable_repository::VariableRepositoryContractInterface
//! [VotingEngine]: crate::voting::VotingEngine
use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, CLTyped, Event, FromBytes, Instance, ToBytes},
    casper_env::caller,
    Address,
    BlockTime,
    ContractCall,
    DocumentHash,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    config::ConfigurationBuilder,
    voting::{
        refs::ContractRefsStorage,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingCreatedInfo,
        VotingEngine,
        VotingId,
    },
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

#[casper_contract_interface]
pub trait ReputationVoterContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsStorage`] by writing addresses of [`Variable Repository`](crate::variable_repository::VariableRepositoryContract),
    /// [`Reputation Token`](crate::reputation::ReputationContract), [`VA Token`](crate::va_nft::VaNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address);
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
    /// see [VotingEngine](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// see [VotingEngine](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// Returns the address of [Variable Repository](crate::variable_repository::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::reputation::ReputationContract) contract.
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;
}

/// ReputationVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::variable_repository::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[derive(Instance)]
pub struct ReputationVoterContract {
    refs: ContractRefsStorage,
    voting_engine: VotingEngine,
    access_control: AccessControl,
}

impl ReputationVoterContractInterface for ReputationVoterContract {
    delegate! {
        to self.voting_engine {
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
        }

        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }

        to self.refs {
            fn variable_repository_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address) {
        self.refs
            .init(variable_repository, reputation_token, va_token);
        self.access_control.init(caller());
    }

    fn create_voting(
        &mut self,
        account: Address,
        action: Action,
        amount: U512,
        document_hash: DocumentHash,
        stake: U512,
    ) {
        let voting_configuration = ConfigurationBuilder::new(&self.refs)
            .contract_call(ContractCall {
                address: self.refs.reputation_token_address(),
                entry_point: action.entrypoint(),
                runtime_args: action.runtime_args(account, amount),
            })
            .build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        ReputationVotingCreated::new(account, action, amount, document_hash, info).emit();
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter, voting_id);
    }
}

/// Informs reputation voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct ReputationVotingCreated {
    account: Address,
    action: Action,
    amount: U512,
    document_hash: DocumentHash,
    creator: Address,
    stake: Option<U512>,
    voting_id: VotingId,
    config_informal_quorum: u32,
    config_informal_voting_time: u64,
    config_formal_quorum: u32,
    config_formal_voting_time: u64,
    config_total_onboarded: U512,
    config_double_time_between_votings: bool,
    config_voting_clearness_delta: U512,
    config_time_between_informal_and_formal_voting: BlockTime,
}

impl ReputationVotingCreated {
    pub fn new(
        account: Address,
        action: Action,
        amount: U512,
        document_hash: DocumentHash,
        info: VotingCreatedInfo,
    ) -> Self {
        Self {
            account,
            action,
            amount,
            document_hash,
            creator: info.creator,
            stake: info.stake,
            voting_id: info.voting_id,
            config_informal_quorum: info.config_informal_quorum,
            config_informal_voting_time: info.config_informal_voting_time,
            config_formal_quorum: info.config_formal_quorum,
            config_formal_voting_time: info.config_formal_voting_time,
            config_total_onboarded: info.config_total_onboarded,
            config_double_time_between_votings: info.config_double_time_between_votings,
            config_voting_clearness_delta: info.config_voting_clearness_delta,
            config_time_between_informal_and_formal_voting: info
                .config_time_between_informal_and_formal_voting,
        }
    }
}
