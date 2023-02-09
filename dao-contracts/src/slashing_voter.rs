//! Contains Slashing Voter Contract definition and related abstractions.
//!
//! # Definitions
//! * Job Offer - A description of a Job posted by JobPoster
//! * Bid - on offer that can be accepted by the Job Poster
//! * JobPoster - user of the system that posts a Job Offer; it has to be KYC’d
//! * Worker - the user who does a job
//! * Voting Associate (or VA) - users of the system with Reputation and permissions to vote
//!
//! # Automated Reputation slashing
//! It is a process of automated burning certain amount of `Reputation` of the `VA`. The amount of `Reputation` to burn
//! is calculated using a formula:
//!
//! `reputation to burn = worker's total reputation * DefaultReputationSlash`
//!
//! If the `Worker` has `Reputation` staked in other parts of the system, we burn it as soon as it is released,
//! until the required amount is burned.
//!
//! If the range of [`DefaultReputationSlash`] is [0.01..0.99] tokens burnt is the only side effect but
//! the value of `1.0` brings additional consequences:
//!  
//! * all the `Reputation` is burnt,
//! * fail all the `Jobs` where this `VA` is either the `Worker` or the `Job Poster`,
//! * fail all the `Voting` started by this `VA`,
//! * remove all the votes from all active `Voting`,
//! * remove all the `Bids` from all auctions,
//! * remove [`VA Token`].
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`]. The subject of voting cannot participate in the process.
//!
//! [`DefaultReputationSlash`]: crate::variable_repository
//! [`VA Token`]: crate::va_nft
use casper_dao_modules::{access_control, AccessControl};
use casper_dao_utils::{
    casper_contract::contract_api::runtime::revert,
    casper_dao_macros::{casper_contract_interface, CLTyped, FromBytes, Instance, ToBytes},
    casper_env::{caller, emit},
    Address,
    BlockTime,
    ContractCall,
    Error,
    Mapping,
    Variable,
};
use casper_event_standard::{Event, Schemas};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    config::ConfigurationBuilder,
    reputation::ReputationContractInterface,
    va_nft::VaNftContractInterface,
    voting::{
        self,
        events::VotingCreatedInfo,
        refs::{ContractRefs, ContractRefsStorage},
        voting_state_machine::{VotingResult, VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
        VotingId,
    },
};

#[casper_contract_interface]
pub trait SlashingVoterContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsStorage`] by writing addresses of [`Variable Repository`],
    /// [`Reputation Token`], [`VA Token`].
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    ///
    /// [`Variable Repository`]: crate::variable_repository::VariableRepositoryContract
    /// [`Reputation Token`]: crate::reputation::ReputationContract
    /// [`VA Token`]: crate::va_nft::VaNftContract
    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address);
    /// Creates new Slashing voting.
    ///
    /// # Arguments
    /// * `address_to_slash` - the [Address] of an account the be slashed,
    /// * `slash_ratio` - the percentage of tokens to slash, if the ratio == 1000, full slashing
    /// is performed.
    ///
    /// # Events
    /// [`SlashingVotingCreated`]
    fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512);
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
        voter: Address,
    ) -> Option<Ballot>;
    /// Returns the address of nth voter who voted on Voting with `voting_id`.
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// Overrides the stored [`Bid Escrow Contract`](crate::bid_escrow) addresses.
    fn update_bid_escrow_list(&mut self, bid_escrows: Vec<Address>);
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
    /// Checks if voting of a given type and id exists.
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    /// Erases the voter from voting with the given id. [Read more](VotingEngine::slash_voter).
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
}

/// Slashing Voter contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Slashing Voter contract needs to have permissions to perform those actions.
///
/// For details see [SlashingVoterContractInterface](SlashingVoterContractInterface)
#[derive(Instance)]
pub struct SlashingVoterContract {
    refs: ContractRefsStorage,
    voting_engine: VotingEngine,
    tasks: Mapping<VotingId, SlashTask>,
    bid_escrows: Variable<Vec<Address>>,
    access_control: AccessControl,
}

impl SlashingVoterContractInterface for SlashingVoterContract {
    delegate! {
        to self.voting_engine {
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                voter: Address,
            ) -> Option<Ballot>;
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
        self.access_control.init(caller());
    }

    fn update_bid_escrow_list(&mut self, bid_escrows: Vec<Address>) {
        self.access_control.ensure_whitelisted();
        self.bid_escrows.set(bid_escrows);
    }

    fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: U512) {
        // TODO: constraints
        let current_reputation = self.refs.reputation_token().balance_of(address_to_slash);

        let voting_configuration = ConfigurationBuilder::new(&self.refs).build();

        let creator = caller();
        let (info, _) = self
            .voting_engine
            .create_voting(creator, stake, voting_configuration);

        let task = SlashTask {
            subject: address_to_slash,
            ratio: slash_ratio,
            reputation_at_voting_creation: current_reputation,
        };
        self.tasks.set(&info.voting_id, task);

        emit(SlashingVotingCreated::new(
            address_to_slash,
            slash_ratio,
            info,
        ));
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        // Check if the caller is not a subject for the voting.
        let task = self.tasks.get_or_revert(&voting_id);
        if caller() == task.subject {
            revert(Error::SubjectOfSlashing);
        }
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let summary = self.voting_engine.finish_voting(voting_id, voting_type);
        if summary.is_formal() && summary.result() == VotingResult::InFavor {
            self.slash(voting_id);
        }
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter, voting_id);
    }
}

impl SlashingVoterContract {
    fn slash(&mut self, voting_id: VotingId) {
        let slash_task = self.tasks.get_or_revert(&voting_id);

        // Burn VA token.
        self.refs.va_token().burn(slash_task.subject);

        let mut reputation = self.refs.reputation_token();
        // If partial slash only burn reputation.
        if slash_task.ratio != 1000 {
            let slash_amount = slash_task.reputation_at_voting_creation * slash_task.ratio / 1000;
            reputation.burn(slash_task.subject, slash_amount);
            return;
        }

        // If full slash burn all reputation
        reputation.burn_all(slash_task.subject);

        // Load account stakes.
        let stakes = reputation.stakes_info(slash_task.subject);

        // Slash all open offers in bid escrows.
        let bid_escrows = self.bid_escrows.get().unwrap_or_default();
        for bid_escrow_address in bid_escrows {
            ContractCall {
                address: bid_escrow_address,
                entry_point: String::from("slash_all_active_job_offers"),
                runtime_args: runtime_args! {
                    "bidder" => slash_task.subject,
                },
            }
            .call();
        }

        // Slash all bids.
        for (bid_escrow_address, bid_id) in stakes.get_bids_stakes_origins() {
            ContractCall {
                address: *bid_escrow_address,
                entry_point: String::from("slash_bid"),
                runtime_args: runtime_args! {
                    "bid_id" => *bid_id,
                },
            }
            .call();
        }

        // Slash subject in all voter contracts.
        for (contract_address, voting_id) in stakes.get_voting_stakes_origins() {
            ContractCall {
                address: *contract_address,
                entry_point: String::from("slash_voter"),
                runtime_args: runtime_args! {
                    "voter" => slash_task.subject,
                    "voting_id" => *voting_id
                },
            }
            .call();
        }
    }
}

#[derive(Debug, Clone, CLTyped, ToBytes, FromBytes)]
struct SlashTask {
    pub subject: Address,
    pub ratio: u32,
    pub reputation_at_voting_creation: U512,
}

/// Informs slashing voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct SlashingVotingCreated {
    address_to_slash: Address,
    slash_ratio: u32,
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

impl SlashingVotingCreated {
    pub fn new(address_to_slash: Address, slash_ratio: u32, info: VotingCreatedInfo) -> Self {
        Self {
            address_to_slash,
            slash_ratio,
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
    schemas.add::<SlashingVotingCreated>();
    schemas
}
