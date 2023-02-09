//! Contains Simple Voter Contract definition and related abstractions.
//!
//! # General
//! Simple voting is a formal, on-chain confirmation of a resolution that has
//! been made off-chain. The off-chain agreement has a form of a [`Document Hash`]
//! that is the voting subject.
//!
//! None action is performed after the voting process is completed.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! [`Repository contract`]: crate::variable_repository::VariableRepositoryContractInterface
//! [`VotingEngine`]: crate::voting::VotingEngine
//! [`Document Hash`]: casper_dao_utils::DocumentHash
use casper_dao_modules::{access_control, AccessControl};
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, emit},
    Address,
    BlockTime,
    DocumentHash,
    Error,
    Mapping,
};
use casper_event_standard::{Event, Schemas};
use casper_types::U512;
use delegate::delegate;

use crate::{
    config::ConfigurationBuilder,
    voting::{
        self,
        events::VotingCreatedInfo,
        refs::ContractRefsStorage,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
        VotingId,
    },
};

#[casper_contract_interface]
pub trait SimpleVoterContractInterface {
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
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged)
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist)
    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address);
    /// Creates new SimpleVoter voting.
    ///
    /// # Arguments
    /// * `variable_repo_to_edit` takes an [Address] of a [Variable Repo](crate::variable_repository::VariableRepositoryContract)
    /// instance that will be updated
    /// * `key`, `value` and `activation_time` are parameters that will be passed to `update_at`
    /// method of a [Variable Repo](crate::variable_repository::VariableRepositoryContract)
    ///
    /// # Events
    /// [`SimpleVotingCreated`]
    fn create_voting(&mut self, document_hash: DocumentHash, stake: U512);
    /// Casts a vote. [Read more](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// Finishes voting. Depending on type of voting, different actions are performed.
    /// [Read more](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// Returns the address of [Variable Repository](crate::variable_repository::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::reputation::ReputationContract) contract.
    fn reputation_token_address(&self) -> Address;
    /// Returns [Voting](VotingStateMachine) for given id.
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// Returns the Voter's [`Ballot`].
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// Returns the address of nth voter who voted on Voting with `voting_id`.
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// Returns document hash being voted on for a given voting id.
    fn get_document_hash(&self, voting_id: VotingId) -> Option<DocumentHash>;
    /// Checks if voting of a given type and id exists.
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    /// Erases the voter from Voting with a given id. [Read more](VotingEngine::slash_voter).
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
    /// Changes the ownership of the contract. Transfers ownership to the `owner`.
    /// Only the current owner is permitted to call this method.
    /// [`Read more`](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);
    /// Adds a new address to the whitelist.
    /// [`Read more`](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);
    /// Remove address from the whitelist.
    /// [`Read more`](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);
    /// Checks whether the given address is added to the whitelist.
    /// [`Read more`](AccessControl::is_whitelisted()).
    fn is_whitelisted(&self, address: Address) -> bool;
    /// Returns the address of the current owner.
    /// [`Read more`](AccessControl::get_owner()).
    fn get_owner(&self) -> Option<Address>;
}

/// SimpleVoterContract
///
/// It is responsible for votings that do not perform any actions on the blockchain.
///
/// The topic of the voting is handled by `document_hash` which is a hash of a document being voted on.
#[derive(Instance)]
pub struct SimpleVoterContract {
    refs: ContractRefsStorage,
    voting_engine: VotingEngine,
    simple_votings: Mapping<VotingId, DocumentHash>,
    access_control: AccessControl,
}

impl SimpleVoterContractInterface for SimpleVoterContract {
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
        casper_event_standard::init(event_schemas());
        self.refs
            .init(variable_repository, reputation_token, va_token);
        self.access_control.init(caller())
    }

    fn create_voting(&mut self, document_hash: DocumentHash, stake: U512) {
        let voting_configuration = ConfigurationBuilder::new(&self.refs).build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        self.simple_votings
            .set(&info.voting_id, document_hash.clone());

        emit(SimpleVotingCreated::new(document_hash, info));
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_summary = self.voting_engine.finish_voting(voting_id, voting_type);

        if let VotingType::Informal = voting_summary.voting_type() {
            match voting_summary.voting_type() {
                VotingType::Informal => {}
                // Informal voting ended in favor, creating a new formal voting
                VotingType::Formal => {
                    self.simple_votings.set(
                        &voting_id,
                        self.simple_votings
                            .get(&voting_id)
                            .unwrap_or_revert_with(Error::VariableValueNotSet),
                    );
                }
            }
        }
    }

    fn get_document_hash(&self, voting_id: VotingId) -> Option<DocumentHash> {
        self.simple_votings.get(&voting_id)
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

/// Informs simple voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct SimpleVotingCreated {
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

impl SimpleVotingCreated {
    pub fn new(document_hash: DocumentHash, info: VotingCreatedInfo) -> Self {
        Self {
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

pub fn event_schemas() -> Schemas {
    let mut schemas = Schemas::new();
    access_control::add_event_schemas(&mut schemas);
    voting::events::add_event_schemas(&mut schemas);
    schemas.add::<SimpleVotingCreated>();
    schemas
}
