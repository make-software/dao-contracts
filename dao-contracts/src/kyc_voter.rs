//! Contains KYC Voter Contract definition and related abstractions.
//!
//! # Definitions
//! KYC - Know Your Customer, is a process that validates that the user can be the user of the system.
//!
//! # KYC process
//! A type of Governance Voting which the VAs validate a user KYC submission.
//! If VAs vote in favor, the address is considered verified and the KYC Token contract
//! is called in order to mint a token which formally ends the KYC process.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`]. There can be only one active voting for a given subject address.
//! An address that is already KYC'd cannot be the subject of voting.
//!
//! [`VotingEngine`]: crate::voting::VotingEngine
use casper_dao_modules::access_control::{self, AccessControl};
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, emit},
    consts,
    Address,
    BlockTime,
    ContractCall,
    DocumentHash,
    Error,
};
use casper_event_standard::{Event, Schemas};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    config::ConfigurationBuilder,
    voting::{
        self,
        events::VotingCreatedInfo,
        refs::ContractRefsWithKycStorage,
        submodules::KycInfo,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
        VotingId,
    },
};

#[casper_contract_interface]
pub trait KycVoterContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsWithKycStorage`] by writing addresses of [`Variable Repository`](crate::variable_repository::VariableRepositoryContract),
    /// [`Reputation Token`](crate::reputation::ReputationContract), [`VA Token`](crate::va_nft::VaNftContract), [`KYC Token`](crate::kyc_nft::KycNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
        kyc_token: Address,
    );
    /// Creates a new KYC Voting. Once the voting passes a KYC Token is minted to the `subject_address`.
    ///
    /// # Prerequisites
    ///
    /// * no voting on the given `subject_address` is in progress,
    /// * `subject_address` does not own a kyc token.
    ///
    /// # Arguments
    /// * `subject_address` - [address](Address) of a user to be verified,
    /// * `document_hash` - a hash of a document that verifies the user. The hash is used as an id of a freshly minted kyc token,
    /// * `subject_address` - an [Address] to be on/offboarded.
    ///
    /// # Events
    /// * [`KycVotingCreated`]
    fn create_voting(&mut self, subject_address: Address, document_hash: DocumentHash, stake: U512);
    /// Casts a vote. [Read more](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// Finishes voting. Depending on type of voting, different actions are performed.
    /// [Read more](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// Returns the address of [Variable Repository](crate::variable_repository::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::reputation::ReputationContract) contract.
    fn reputation_token_address(&self) -> Address;
    /// Checks if voting of a given type and id exists.
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    /// Erases the voter from voting with the given id. [Read more](VotingEngine::slash_voter).
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
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
    /// Returns the address of [KYC Token](crate::kyc_nft::KycNftContract) contract.
    fn kyc_token_address(&self) -> Address;
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

/// KycVoterContract
///
/// It is responsible for managing kyc tokens (see [KYC Token Contract](crate::kyc_nft).
///
/// When the voting passes, a kyc token is minted.
#[derive(Instance)]
pub struct KycVoterContract {
    refs: ContractRefsWithKycStorage,
    kyc: KycInfo,
    access_control: AccessControl,
    voting_engine: VotingEngine,
}

impl KycVoterContractInterface for KycVoterContract {
    delegate! {
        to self.voting_engine {
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
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
            fn kyc_token_address(&self) -> Address;
        }
    }

    fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
        kyc_token: Address,
    ) {
        casper_event_standard::init(event_schemas());
        self.refs
            .init(variable_repository, reputation_token, va_token, kyc_token);
        self.access_control.init(caller());
    }

    fn create_voting(
        &mut self,
        subject_address: Address,
        document_hash: DocumentHash,
        stake: U512,
    ) {
        self.assert_no_ongoing_voting(&subject_address);
        self.assert_not_kyced(&subject_address);

        let creator = caller();

        let voting_configuration = ConfigurationBuilder::new(&self.refs)
            .contract_call(ContractCall {
                address: self.kyc_token_address(),
                entry_point: consts::EP_MINT.to_string(),
                runtime_args: runtime_args! {
                    consts::ARG_TO => subject_address,
                },
            })
            .build();

        let (info, _) = self
            .voting_engine
            .create_voting(creator, stake, voting_configuration);

        self.kyc.set_voting(subject_address, info.voting_id);

        emit(KycVotingCreated::new(subject_address, document_hash, info));
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let summary = self.voting_engine.finish_voting(voting_id, voting_type);
        // The voting is ended when:
        // 1. Informal voting has been rejected.
        // 2. Formal voting has been finish (regardless of the final result).
        if summary.is_voting_process_finished() {
            let voting = self
                .voting_engine
                .get_voting(voting_id)
                .unwrap_or_revert_with(Error::VotingDoesNotExist);
            let address = self.kyc.get_voting_subject(voting.voting_id());
            self.kyc.clear_voting(&address);
        }
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter, voting_id);
    }
}

// non-contract implementation
impl KycVoterContract {
    fn assert_not_kyced(&self, address: &Address) {
        if self.kyc.is_kycd(address) {
            casper_env::revert(Error::UserKycedAlready);
        }
    }

    fn assert_no_ongoing_voting(&self, address: &Address) {
        if self.kyc.exists_ongoing_voting(address) {
            casper_env::revert(Error::KycAlreadyInProgress);
        }
    }
}

/// Informs kyc voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct KycVotingCreated {
    subject_address: Address,
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

impl KycVotingCreated {
    pub fn new(
        subject_address: Address,
        document_hash: DocumentHash,
        info: VotingCreatedInfo,
    ) -> Self {
        Self {
            subject_address,
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
    schemas.add::<KycVotingCreated>();
    schemas
}
