use odra::{
    contract_env::{caller, revert},
    types::{event::OdraEvent, Address, Balance, BlockTime},
    Composer, Event, Instance, Mapping, OdraType, UnwrapOrRevert, Variable,
};

use crate::rules::validation::IsVa;
use crate::rules::RulesBuilder;
use crate::{
    configuration::ConfigurationBuilder,
    modules::{refs::ContractRefs, AccessControl},
    utils::Error,
    voting::{
        ballot::{Ballot, Choice},
        types::VotingId,
        voting_engine::{
            events::VotingCreatedInfo,
            voting_state_machine::{VotingResult, VotingStateMachine, VotingSummary, VotingType},
            VotingEngine, VotingEngineComposer,
        },
    },
};

/// Slashing Voter contract uses [VotingEngine](VotingEngine) to vote on changes of ownership and managing whitelists of other contracts.
///
/// Slashing Voter contract needs to have permissions to perform those actions.
#[odra::module(skip_instance, events = [SlashingVotingCreated])]
pub struct SlashingVoterContract {
    refs: ContractRefs,
    voting_engine: VotingEngine,
    tasks: Mapping<VotingId, SlashTask>,
    slashable_contracts: Variable<Vec<Address>>,
    access_control: AccessControl,
}

impl Instance for SlashingVoterContract {
    fn instance(namespace: &str) -> Self {
        let refs = Composer::new(namespace, "refs").compose();
        let voting_engine = VotingEngineComposer::new(namespace, "voting_engine")
            .with_refs(&refs)
            .compose();

        Self {
            refs,
            voting_engine,
            tasks: Composer::new(namespace, "tasks").compose(),
            slashable_contracts: Composer::new(namespace, "slashable_contracts").compose(),
            access_control: Composer::new(namespace, "access_control").compose(),
        }
    }
}

#[odra::module]
impl SlashingVoterContract {
    delegate! {
        to self.voting_engine {
            pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            pub fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            pub fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            pub fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                voter: Address,
            ) -> Option<Ballot>;
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

    pub fn update_slashable_contracts(&mut self, slashable_contracts: Vec<Address>) {
        self.access_control.ensure_whitelisted();
        self.slashable_contracts.set(slashable_contracts);
    }

    pub fn create_voting(&mut self, address_to_slash: Address, slash_ratio: u32, stake: Balance) {
        let creator = caller();

        // Both creator and address_to_slash must be VA.
        RulesBuilder::new()
            .add_validation(IsVa::create(
                !self.refs.va_token().balance_of(&creator).is_zero(),
            ))
            .add_validation(IsVa::create(
                !self.refs.va_token().balance_of(&address_to_slash).is_zero(),
            ))
            .build()
            .validate_generic_validations();

        let current_reputation = self.refs.reputation_token().balance_of(address_to_slash);

        let voting_configuration = ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .build();

        let (info, _) = self
            .voting_engine
            .create_voting(creator, stake, voting_configuration);

        let task = SlashTask {
            subject: address_to_slash,
            ratio: slash_ratio,
            reputation_at_voting_creation: current_reputation,
        };
        self.tasks.set(&info.voting_id, task);

        SlashingVotingCreated::new(address_to_slash, slash_ratio, info).emit();
    }

    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
    ) {
        // Check if the caller is not a subject for the voting.
        let task = self.tasks.get(&voting_id).unwrap_or_revert();
        if caller() == task.subject {
            revert(Error::SubjectOfSlashing);
        }
        self.voting_engine
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary {
        let summary = self.voting_engine.finish_voting(voting_id, voting_type);
        if summary.is_formal() && summary.result() == VotingResult::InFavor {
            self.slash(voting_id);
        }
        summary
    }

    pub fn slash_voter(&mut self, voter: Address) {
        self.access_control.ensure_whitelisted();
        self.voting_engine.slash_voter(voter);
    }
}

impl SlashingVoterContract {
    fn slash(&mut self, voting_id: VotingId) {
        let slash_task = self.tasks.get(&voting_id).unwrap_or_revert();

        // Burn VA token.
        self.refs.va_token().burn(slash_task.subject);

        let mut reputation = self.refs.reputation_token();
        // If partial slash only burn reputation.
        if slash_task.ratio != 1000 {
            let slash_amount = (slash_task.reputation_at_voting_creation
                * Balance::from(slash_task.ratio))
                / Balance::from(1000);
            reputation.burn(slash_task.subject, slash_amount);
            return;
        }

        // Slash subject in all voter contracts.
        for address in self.slashable_contracts.get_or_default() {
            SlashableRef::at(&address).slash_voter(slash_task.subject);
        }

        // If full slash burn all reputation
        reputation.burn_all(slash_task.subject);
    }
}

#[odra::external_contract]
trait Slashable {
    fn slash_voter(&mut self, voter: Address);
}

#[derive(Debug, OdraType)]
pub struct SlashTask {
    pub subject: Address,
    pub ratio: u32,
    pub reputation_at_voting_creation: Balance,
}

/// Event emitted when slashing voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct SlashingVotingCreated {
    address_to_slash: Address,
    slash_ratio: u32,
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
