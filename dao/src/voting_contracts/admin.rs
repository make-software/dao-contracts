use crate::configuration::ConfigurationBuilder;
use crate::modules::refs::ContractRefs;
use crate::modules::AccessControl;
use crate::utils::ContractCall;
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::events::VotingCreatedInfo;
use crate::voting::voting_engine::voting_state_machine::VotingType;
use crate::voting::voting_engine::voting_state_machine::{VotingStateMachine, VotingSummary};
use crate::voting::voting_engine::VotingEngine;
use crate::voting_contracts::SlashedVotings;
use odra::contract_env::{caller, emit_event};
use odra::prelude::string::{String, ToString};
use odra::types::{Address, Balance, BlockTime, CallArgs};
use odra::{Event, OdraType};

/// Admin contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Admin contract needs to have permissions to perform those actions.
#[odra::module(events = [AdminVotingCreated])]
pub struct AdminContract {
    refs: ContractRefs,
    #[odra(using = "refs")]
    voting_engine: VotingEngine,
    access_control: AccessControl,
}

#[odra::module]
impl AdminContract {
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
        contract_to_update: Address,
        action: Action,
        address: Address,
        stake: Balance,
    ) {
        let mut call_args = CallArgs::new();
        call_args.insert(action.get_arg(), address);

        let voting_configuration = ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .contract_call(ContractCall {
            address: contract_to_update,
            entry_point: action.get_entry_point(),
            call_args,
            amount: None,
        })
        .build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        emit_event(AdminVotingCreated::new(
            contract_to_update,
            action,
            address,
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

    pub fn slash_voter(&mut self, voter: Address) -> SlashedVotings {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter)
    }
}

/// Event emitted once voting is created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct AdminVotingCreated {
    contract_to_update: Address,
    action: Action,
    address: Address,
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

impl AdminVotingCreated {
    pub fn new(
        contract_to_update: Address,
        action: Action,
        address: Address,
        info: VotingCreatedInfo,
    ) -> Self {
        Self {
            contract_to_update,
            action,
            address,
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

/// Enum for actions that [AdminContract] can perform
///
/// - `AddToWhitelist` - calls `add_to_whitelist` method
/// - `RemoveFromWhitelist` - calls `remove_from_whitelist` method
/// - `ChangeOwner` - calls `change_ownership` method
#[derive(OdraType, Eq, PartialEq, Debug)]
pub enum Action {
    AddToWhitelist,
    RemoveFromWhitelist,
    ChangeOwner,
}

impl Action {
    pub(crate) fn get_entry_point(&self) -> String {
        match self {
            Action::AddToWhitelist => "add_to_whitelist",
            Action::RemoveFromWhitelist => "remove_from_whitelist",
            Action::ChangeOwner => "change_ownership",
        }
        .to_string()
    }

    pub(crate) fn get_arg(&self) -> &str {
        match self {
            Action::AddToWhitelist => "address",
            Action::RemoveFromWhitelist => "address",
            Action::ChangeOwner => "owner",
        }
    }
}
