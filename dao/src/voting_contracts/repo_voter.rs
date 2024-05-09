use odra::contract_env::revert;
use odra::types::OdraType;
use odra::{
    contract_env::caller,
    prelude::string::{String, ToString},
    types::{event::OdraEvent, Address, Balance, BlockTime, Bytes, CallArgs},
    Event,
};

use crate::utils::variable_type::VariableType;
use crate::utils::Error::CouldntDeserializeValueToCorrectType;
use crate::voting_contracts::SlashedVotings;
use crate::{
    configuration::ConfigurationBuilder,
    modules::{refs::ContractRefs, AccessControl},
    utils::{consts, ContractCall},
    voting::{
        ballot::{Ballot, Choice},
        types::VotingId,
        voting_engine::{
            events::VotingCreatedInfo,
            voting_state_machine::{VotingStateMachine, VotingSummary, VotingType},
            VotingEngine,
        },
    },
};

/// RepoVoterContract
///
/// It is responsible for managing variables held in [Variable Repo](crate::core_contracts::VariableRepositoryContract).
///
/// Each change to the variable is being voted on, and when the voting passes, a change is made at given time.
#[odra::module(events = [RepoVotingCreated])]
pub struct RepoVoterContract {
    refs: ContractRefs,
    #[odra(using = "refs")]
    voting_engine: VotingEngine,
    access_control: AccessControl,
}

#[odra::module]
impl RepoVoterContract {
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
            pub fn cancel_finished_voting(&mut self, voting_id: VotingId);
        }

        to self.access_control {
            pub fn propose_new_owner(&mut self, owner: Address);
            pub fn accept_new_owner(&mut self);
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
        variable_repo_to_edit: Address,
        key: String,
        value: Bytes,
        activation_time: Option<u64>,
        stake: Balance,
    ) {
        Self::assert_correct_value_type(&key, &value);

        let voting_configuration = ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .contract_call(ContractCall {
            address: variable_repo_to_edit,
            entry_point: consts::EP_UPDATE_AT.to_string(),
            call_args: {
                let mut args = CallArgs::new();
                args.insert(consts::ARG_KEY.to_string(), key.clone());
                args.insert(consts::ARG_VALUE.to_string(), value.clone());
                args.insert(consts::ARG_ACTIVATION_TIME.to_string(), activation_time);
                args
            },
            amount: None,
        })
        .build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        RepoVotingCreated::new(variable_repo_to_edit, key, value, activation_time, info).emit();
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

impl RepoVoterContract {
    fn assert_correct_value_type(key: &str, value: &Bytes) {
        let result = match VariableType::from_key(key) {
            VariableType::Balance => Balance::deserialize(value).is_some(),
            VariableType::BlockTime => BlockTime::deserialize(value).is_some(),
            VariableType::Address => Address::deserialize(value).is_some(),
            VariableType::Bool => bool::deserialize(value).is_some(),
            VariableType::Unknown => true,
        };

        if !result {
            revert(CouldntDeserializeValueToCorrectType)
        }
    }
}

/// Event emitted when repo voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct RepoVotingCreated {
    variable_repo_to_edit: Address,
    key: String,
    value: Bytes,
    activation_time: Option<u64>,
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

impl RepoVotingCreated {
    pub fn new(
        variable_repo_to_edit: Address,
        key: String,
        value: Bytes,
        activation_time: Option<u64>,
        info: VotingCreatedInfo,
    ) -> Self {
        Self {
            variable_repo_to_edit,
            key,
            value,
            activation_time,
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
