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
use crate::modules::refs::ContractRefs;
use crate::modules::AccessControl;
use crate::utils::types::DocumentHash;
use crate::utils::ContractCall;
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::events::VotingCreatedInfo;
use crate::voting::voting_engine::voting_state_machine::VotingType;
use crate::voting::voting_engine::voting_state_machine::{VotingStateMachine, VotingSummary};
use crate::voting::voting_engine::{VotingEngine, VotingEngineComposer};
use odra::contract_env::{caller, emit_event};
use odra::types::{Address, Balance, BlockTime, CallArgs};
use odra::{Composer, Event, Instance, OdraType};

/// ReputationVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::core_contracts::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[odra::module(skip_instance, events = [ReputationVotingCreated])]
pub struct ReputationVoterContract {
    refs: ContractRefs,
    voting_engine: VotingEngine,
    access_control: AccessControl,
}

impl Instance for ReputationVoterContract {
    fn instance(namespace: &str) -> Self {
        let refs = Composer::new(namespace, "refs").compose();
        let voting_engine = VotingEngineComposer::new(namespace, "voting_engine")
            .with_refs(&refs)
            .compose();

        Self {
            refs,
            voting_engine,
            access_control: Composer::new(namespace, "access_control").compose(),
        }
    }
}

#[odra::module]
impl ReputationVoterContract {
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
            pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary;
        }

        to self.access_control {
            pub fn change_ownership(&mut self, owner: Address);
            pub fn add_to_whitelist(&mut self, address: Address);
            pub fn remove_from_whitelist(&mut self, address: Address);
            pub fn is_whitelisted(&self, address: Address) -> bool;
            pub fn get_owner(&self) -> Option<Address>;
        }

        to self.refs {
            pub fn variable_repository_address(&self) -> Address;
            pub fn reputation_token_address(&self) -> Address;
        }
    }

    #[odra(init)]
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        va_token: Address,
    ) {
        self.refs.set_variable_repository(variable_repository);
        self.refs.set_reputation_token(reputation_token);
        self.refs.set_va_token(va_token);
        self.access_control.init(caller());
    }

    pub fn create_voting(
        &mut self,
        account: Address,
        action: Action,
        amount: Balance,
        document_hash: DocumentHash,
        stake: Balance,
    ) {
        let voting_configuration = ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .contract_call(ContractCall {
            address: self.refs.reputation_token_address(),
            entry_point: action.entrypoint(),
            call_args: action.call_args(account, amount),
            amount: None,
        })
        .build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        emit_event(ReputationVotingCreated::new(
            account,
            action,
            amount,
            document_hash,
            info,
        ));
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

    pub fn slash_voter(&mut self, voter: Address) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter);
    }
}

/// Event emitted once voting is created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct ReputationVotingCreated {
    account: Address,
    action: Action,
    amount: Balance,
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

impl ReputationVotingCreated {
    pub fn new(
        account: Address,
        action: Action,
        amount: Balance,
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

/// Action to perform against reputation
#[derive(OdraType, Debug, PartialEq, Eq, Copy)]
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

    pub fn call_args(&self, account: Address, amount: Balance) -> CallArgs {
        match self {
            Action::Burn => {
                let mut call_args = CallArgs::new();
                call_args.insert("owner", account);
                call_args.insert("amount", amount);
                call_args
            }
            Action::Mint => {
                let mut call_args = CallArgs::new();
                call_args.insert("recipient", account);
                call_args.insert("amount", amount);
                call_args
            }
        }
    }
}
