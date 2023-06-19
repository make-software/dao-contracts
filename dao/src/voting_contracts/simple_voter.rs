use odra::{
    contract_env::caller,
    types::{event::OdraEvent, Address, Balance, BlockTime},
    Composer, Event, Instance, Mapping, UnwrapOrRevert,
};

use crate::{
    configuration::ConfigurationBuilder,
    modules::{refs::ContractRefs, AccessControl},
    utils::{types::DocumentHash, Error},
    voting::{
        ballot::{Ballot, Choice},
        types::VotingId,
        voting_engine::{
            events::VotingCreatedInfo,
            voting_state_machine::{VotingStateMachine, VotingSummary, VotingType},
            VotingEngine, VotingEngineComposer,
        },
    },
};

/// SimpleVoterContract
///
/// It is responsible for votings that do not perform any actions on the blockchain.
///
/// The topic of the voting is handled by `document_hash` which is a hash of a document being voted on.
#[odra::module(skip_instance, events = [SimpleVotingCreated])]
pub struct SimpleVoterContract {
    refs: ContractRefs,
    voting_engine: VotingEngine,
    simple_votings: Mapping<VotingId, DocumentHash>,
    access_control: AccessControl,
}

impl Instance for SimpleVoterContract {
    fn instance(namespace: &str) -> Self {
        let refs = Composer::new(namespace, "refs").compose();
        let voting_engine = VotingEngineComposer::new(namespace, "voting_engine")
            .with_refs(&refs)
            .compose();

        Self {
            refs,
            voting_engine,
            access_control: Composer::new(namespace, "access_control").compose(),
            simple_votings: Composer::new(namespace, "simple_votings").compose(),
        }
    }
}

#[odra::module]
impl SimpleVoterContract {
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
        self.access_control.init(caller())
    }

    pub fn create_voting(&mut self, document_hash: DocumentHash, stake: Balance) {
        let voting_configuration = ConfigurationBuilder::new(
            self.refs.va_token().total_supply(),
            &self.refs.variable_repository().all_variables(),
        )
        .build();

        let (info, _) = self
            .voting_engine
            .create_voting(caller(), stake, voting_configuration);

        self.simple_votings
            .set(&info.voting_id, document_hash.clone());

        SimpleVotingCreated::new(document_hash, info).emit();
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary {
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
        voting_summary
    }

    pub fn get_document_hash(&self, voting_id: VotingId) -> Option<DocumentHash> {
        self.simple_votings.get(&voting_id)
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

/// Informs simple voting has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct SimpleVotingCreated {
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
