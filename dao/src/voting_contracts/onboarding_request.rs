//! Contains Onboarding Request Contract definition and related abstractions.
//!
//! # Definitions
//! * Job Offer - A description of a Job posted by JobPoster
//! * Bid - on offer that can be accepted by the Job Poster
//! * JobPoster - user of the system that posts a Job Offer; it has to be KYC’d
//! * External Worker - a Worker who completed the KYC and is not a Voting Associate
//! * Voting Associate (or VA) - users of the system with Reputation and permissions to vote
//! * KYC - Know Your Customer, a process that validates that the user can be the user of the system
//! * Bid Escrow Voting - Mints reputation
//!
//! # Onboarding Request
//! One of the side effects of completing a `Job` by an `External Worker` is the possibility to become a `Voting Associate`.
//! It is also possible to become one without completing a `Job` using [`Bid Escrow Contract`].
//! To do this, an `External Worker` submits an `Onboarding Request` containing `Document Hash` of a document containing the reason
//! why the `Onboarding` should be done and a `CSPR` stake.
//!
//! The rest of the process is analogous to the regular `Job` [submission process] of an `External Worker`, except that instead of
//! redistribution of `Job Payment` between `VA`’s we redistribute the stake of the `External Worker`.
//! If the process fails, the `CSPR` stake of the `External Worker` is returned.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! [`Bid Escrow Contract`]: crate::bid_escrow::contract::BidEscrowContract
//! [`VotingEngine`]: VotingEngine
//! [submission process]: crate::bid_escrow#submitting-a-job-proof
use crate::modules::refs::ContractRefs;
use crate::modules::AccessControl;
use crate::onboarding::Onboarding;
use crate::utils::types::DocumentHash;
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::events::VotingCreatedInfo;
use crate::voting::voting_engine::voting_state_machine::{
    VotingStateMachine, VotingSummary, VotingType,
};
use crate::voting::voting_engine::VotingEngine;
use crate::voting_contracts::SlashedVotings;
use odra::contract_env::{attached_value, caller, self_balance};
use odra::types::event::OdraEvent;
use odra::types::{Address, Balance, BlockTime};
use odra::Event;

/// Onboarding Request Contract.
#[odra::module(events = [OnboardingVotingCreated])]
pub struct OnboardingRequestContract {
    refs: ContractRefs,
    #[odra(using = "refs")]
    voting: VotingEngine,
    access_control: AccessControl,
    #[odra(using = "refs, voting")]
    onboarding: Onboarding,
}

#[odra::module]
impl OnboardingRequestContract {
    delegate! {
        to self.voting {
            /// Checks if voting of a given type and id exists.
            pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            /// Returns the Voter's [`Ballot`].
            pub fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            /// Gets the address of nth voter who voted on Voting with `voting_id`.
            pub fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            /// Returns [Voting](VotingStateMachine) for given id.
            pub fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
        }

        to self.access_control {
            /// Changes the ownership of the contract. Transfers ownership to the `owner`.
            /// Only the current owner is permitted to call this method.
            /// [`Read more`](AccessControl::change_ownership())
            pub fn change_ownership(&mut self, owner: Address);
            /// Adds a new address to the whitelist.
            /// [`Read more`](AccessControl::add_to_whitelist())
            pub fn add_to_whitelist(&mut self, address: Address);
            /// Remove address from the whitelist.
            /// [`Read more`](AccessControl::remove_from_whitelist())
            pub fn remove_from_whitelist(&mut self, address: Address);
            /// Checks whether the given address is added to the whitelist.
            /// [`Read more`](AccessControl::is_whitelisted()).
            pub fn is_whitelisted(&self, address: Address) -> bool;
            /// Returns the address of the current owner.
            /// [`Read more`](AccessControl::get_owner()).
            pub fn get_owner(&self) -> Option<Address>;
        }

        to self.onboarding {
            /// Finishes voting stage. Depending on stage, the voting can be converted to a formal one, end
            /// with a refund or convert the requester to a VA.
            pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary;
            /// Casts a vote. [Read more](VotingEngine::vote())
            pub fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: Balance);
        }

        to self.refs {
            /// Returns the address of [Variable Repository](crate::core_contracts::VariableRepositoryContract) contract.
            pub fn variable_repository_address(&self) -> Address;
            /// Returns the address of [Reputation Token](crate::core_contracts::ReputationContract) contract.
            pub fn reputation_token_address(&self) -> Address;
        }
    }

    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up the contract by writing addresses of [`Variable Repository`](crate::core_contracts::VariableRepositoryContract),
    /// [`Reputation Token`](crate::core_contracts::ReputationContract), [`VA Token`](crate::core_contracts::VaNftContract), [`KYC Token`](crate::core_contracts::KycNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// * [`OwnerChanged`](crate::modules::owner::events::OwnerChanged),
    /// * [`AddedToWhitelist`](crate::modules::whitelist::events::AddedToWhitelist),
    #[odra(init)]
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.refs.set_variable_repository(variable_repository);
        self.refs.set_reputation_token(reputation_token);
        self.refs.set_va_token(va_token);
        self.refs.set_kyc_token(kyc_token);
        self.access_control.init(caller());
    }

    /// Submits an onboarding request. If the request is valid voting starts.
    ///
    /// # Events
    /// * [`OnboardingVotingCreated`]
    #[odra(payable)]
    pub fn create_voting(&mut self, reason: DocumentHash) {
        let cspr_deposit = attached_value();
        let voting_info = self.onboarding.submit_request(reason.clone(), cspr_deposit);
        OnboardingVotingCreated::new(reason, cspr_deposit, voting_info).emit();
    }

    /// Gets the CSPR balance of the contract.
    pub fn get_cspr_balance(&self) -> Balance {
        self_balance()
    }

    /// Erases the voter from voting with the given id. [Read more](VotingEngine::slash_voter).
    pub fn slash_voter(&mut self, voter: Address) -> SlashedVotings {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter)
    }
}

/// Event emitted when onboarding voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct OnboardingVotingCreated {
    reason: DocumentHash,
    cspr_deposit: Balance,
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

impl OnboardingVotingCreated {
    pub fn new(reason: DocumentHash, cspr_deposit: Balance, info: VotingCreatedInfo) -> Self {
        Self {
            reason,
            cspr_deposit,
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
