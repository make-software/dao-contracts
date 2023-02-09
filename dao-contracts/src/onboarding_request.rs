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
//! [`Bid Escrow Contract`]: crate::bid_escrow::BidEscrowContractInterface
//! [`VotingEngine`]: crate::voting::VotingEngine
//! [submission process]: crate::bid_escrow#submitting-a-job-proof
use casper_dao_modules::access_control::{self, AccessControl};
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, emit},
    cspr,
    Address,
    BlockTime,
    DocumentHash,
};
use casper_event_standard::{Event, Schemas};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::voting::{
    events::VotingCreatedInfo,
    refs::ContractRefsWithKycStorage,
    voting_state_machine::{VotingStateMachine, VotingType},
    Ballot,
    Choice,
    VotingEngine,
    VotingId,
};

pub mod request;
pub mod voting;

#[casper_contract_interface]
pub trait OnboardingRequestContractInterface {
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
        kyc_token: Address,
        va_token: Address,
    );
    /// Submits an onboarding request. If the request is valid voting starts.
    ///
    /// # Events
    /// * [`OnboardingVotingCreated`]
    fn create_voting(&mut self, reason: DocumentHash, purse: URef);
    /// Casts a vote. [Read more](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// Finishes voting stage. Depending on stage, the voting can be converted to a formal one, end
    /// with a refund or convert the requestor to a VA.
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
    /// Gets the address of nth voter who voted on Voting with `voting_id`.
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// Checks if voting of a given type and id exists.
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    /// Erases the voter from voting with the given id. [Read more](VotingEngine::slash_voter).
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
    /// Gets the CSPR balance of the contract.
    fn get_cspr_balance(&self) -> U512;
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

/// TODO: docs
#[derive(Instance)]
pub struct OnboardingRequestContract {
    refs: ContractRefsWithKycStorage,
    voting: VotingEngine,
    access_control: AccessControl,
    onboarding: Onboarding,
}

impl OnboardingRequestContractInterface for OnboardingRequestContract {
    delegate! {
        to self.voting {
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

        to self.onboarding {
            fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
            fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
        }

        to self.refs {
            fn variable_repository_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        casper_event_standard::init(event_schemas());
        self.refs
            .init(variable_repository, reputation_token, va_token, kyc_token);
        self.access_control.init(caller());
    }

    fn create_voting(&mut self, reason: DocumentHash, purse: URef) {
        let cspr_deposit = cspr::deposit(purse);
        let voting_info = self.onboarding.submit_request(reason.clone(), cspr_deposit);
        emit(OnboardingVotingCreated::new(
            reason,
            cspr_deposit,
            voting_info,
        ));
    }

    fn get_cspr_balance(&self) -> U512 {
        cspr::main_purse_balance()
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }
}

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

use self::voting::Onboarding;

#[cfg(feature = "test-support")]
impl OnboardingRequestContractTest {
    pub fn submit_onboarding_request_with_cspr_amount(
        &mut self,
        reason: DocumentHash,
        cspr_amount: U512,
    ) -> Result<(), casper_dao_utils::Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "submit_onboarding_request.wasm",
            runtime_args! {
                "onboarding_address" => self.address(),
                "reason" => reason,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }
}

/// Informs onboarding voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct OnboardingVotingCreated {
    reason: DocumentHash,
    cspr_deposit: U512,
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

impl OnboardingVotingCreated {
    pub fn new(reason: DocumentHash, cspr_deposit: U512, info: VotingCreatedInfo) -> Self {
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

pub fn event_schemas() -> Schemas {
    let mut schemas = Schemas::new();
    access_control::add_event_schemas(&mut schemas);
    crate::voting::events::add_event_schemas(&mut schemas);
    schemas.add::<OnboardingVotingCreated>();
    schemas
}
