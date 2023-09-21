// impl KycVoterContractInterface for KycVoterContract {

// }

//! Contains Reputation Voter Contract definition and related abstractions.
//!
//! # General
//! A type of Governance Voting used to operate on the [`Reputation Token Contract`].
//!
//! Two types of voting can be created:
//! * to `mint` tokens for a user,
//! * to `burn` users' tokens.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! [`Reputation Token Contract`]: crate::core_contracts::ReputationContract
//! [`VotingEngine`]: VotingEngine
use crate::configuration::ConfigurationBuilder;
use crate::modules::kyc_info::KycInfo;
use crate::modules::refs::ContractRefs;
use crate::modules::AccessControl;
use crate::utils::types::DocumentHash;
use crate::utils::{consts, ContractCall, Error};
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::events::VotingCreatedInfo;
use crate::voting::voting_engine::voting_state_machine::VotingType;
use crate::voting::voting_engine::voting_state_machine::{VotingStateMachine, VotingSummary};
use crate::voting::voting_engine::VotingEngine;
use odra::contract_env::{self, caller};
use odra::prelude::string::ToString;
use odra::types::event::OdraEvent;
use odra::types::{Address, Balance, BlockTime, CallArgs};
use odra::{Event, UnwrapOrRevert};

/// KycVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::core_contracts::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[odra::module(events = [KycVotingCreated])]
pub struct KycVoterContract {
    refs: ContractRefs,
    #[odra(using = "refs")]
    voting_engine: VotingEngine,
    access_control: AccessControl,
    #[odra(using = "refs")]
    kyc: KycInfo,
}

#[odra::module]
impl KycVoterContract {
    delegate! {
        to self.voting_engine {
            pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            pub fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            pub fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            pub fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
        }

        to self.access_control {
            pub fn change_ownership(&mut self, owner: Address);
            pub fn add_to_whitelist(&mut self, address: Address);
            pub fn remove_from_whitelist(&mut self, address: Address);
            pub fn is_whitelisted(&self, address: Address) -> bool;
            pub fn get_owner(&self) -> Option<Address>;
        }
    }

    #[odra(init)]
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
        kyc_token: Address,
    ) {
        self.refs.set_variable_repository(variable_repository);
        self.refs.set_reputation_token(reputation_token);
        self.refs.set_va_token(va_token);
        self.refs.set_kyc_token(kyc_token);
        self.access_control.init(caller());
    }

    pub fn create_voting(
        &mut self,
        subject_address: Address,
        document_hash: DocumentHash,
        stake: Balance,
    ) {
        self.assert_no_ongoing_voting(&subject_address);
        self.assert_not_kyced(&subject_address);

        let creator = caller();

        let voting_configuration = ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .contract_call(ContractCall {
            address: self.refs.kyc_token_address(),
            entry_point: consts::EP_MINT.to_string(),
            call_args: {
                let mut args = CallArgs::new();
                args.insert(consts::ARG_TO.to_string(), subject_address);
                args
            },
            amount: None,
        })
        .build();

        let (info, _) = self
            .voting_engine
            .create_voting(creator, stake, voting_configuration);

        self.kyc.set_voting(subject_address, info.voting_id);

        KycVotingCreated::new(subject_address, document_hash, info).emit();
    }

    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
    ) {
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary {
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
        summary
    }

    pub fn slash_voter(&mut self, voter: Address) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter);
    }

    fn assert_not_kyced(&self, address: &Address) {
        if self.kyc.is_kycd(address) {
            contract_env::revert(Error::UserKycedAlready);
        }
    }

    fn assert_no_ongoing_voting(&self, address: &Address) {
        if self.kyc.exists_ongoing_voting(address) {
            contract_env::revert(Error::KycAlreadyInProgress);
        }
    }
}

/// Event emitted when kyc voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct KycVotingCreated {
    subject_address: Address,
    document_hash: DocumentHash,
    creator: Address,
    stake: Option<Balance>,
    voting_id: VotingId,
    config_informal_quorum: u32,
    config_informal_voting_time: u64,
    config_formal_quorum: u32,
    config_formal_voting_time: u64,
    config_total_onboarded: Balance,
    config_double_time_between_votings: bool,
    config_voting_clearness_delta: Balance,
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
