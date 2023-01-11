use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Event, Instance},
    casper_env::caller,
    Address,
    BlockTime,
    DocumentHash,
    Error,
    Mapping,
};
use casper_types::U512;
use delegate::delegate;

use crate::{
    refs::ContractRefsStorage,
    voting::{
        types::VotingId,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingCreatedInfo,
        VotingEngine,
    },
    ConfigurationBuilder,
};

#[casper_contract_interface]
pub trait SimpleVoterContractInterface {
    /// see [VotingEngine](VotingEngine::init())
    fn init(&mut self, variable_repository: Address, reputation_token: Address, va_token: Address);
    /// Creates new SimpleVoter voting.
    ///
    /// `variable_repo_to_edit` takes an [Address](Address) of a [Variable Repo](crate::VariableRepositoryContract) instance that will be updated
    ///
    /// `key`, `value` and `activation_time` are parameters that will be passed to `update_at` method of a [Variable Repo](crate::VariableRepositoryContract)
    fn create_voting(&mut self, document_hash: DocumentHash, stake: U512);
    /// see [VotingEngine](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// see [VotingEngine](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [VotingEngine](VotingEngine::get_variable_repo_address())
    fn variable_repository_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// Returns document hash being voted on for given voting id.
    fn get_document_hash(&self, voting_id: VotingId) -> Option<DocumentHash>;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;

    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;
}

/// SimpleVoterContract
///
/// It is responsible for votings that do not perform any actions on the blockchain.
///
/// The topic of the voting is handled by `document_hash` which is a hash of a document being voted on.
#[derive(Instance)]
pub struct SimpleVoterContract {
    refs: ContractRefsStorage,
    voting: VotingEngine,
    simple_votings: Mapping<VotingId, DocumentHash>,
    access_control: AccessControl,
}

impl SimpleVoterContractInterface for SimpleVoterContract {
    delegate! {
        to self.voting {
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_voting(
                &self,
                voting_id: VotingId,
            ) -> Option<VotingStateMachine>;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
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
        self.refs
            .init(variable_repository, reputation_token, va_token);
        self.access_control.init(caller())
    }

    fn create_voting(&mut self, document_hash: DocumentHash, stake: U512) {
        let voting_configuration = ConfigurationBuilder::new(&self.refs).build();

        let info = self
            .voting
            .create_voting(caller(), stake, voting_configuration);

        self.simple_votings
            .set(&info.voting_id, document_hash.clone());

        SimpleVotingCreated::new(document_hash, info).emit();
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_summary = self.voting.finish_voting(voting_id, voting_type);

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
    }

    fn get_document_hash(&self, voting_id: VotingId) -> Option<DocumentHash> {
        self.simple_votings.get(&voting_id)
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct SimpleVotingCreated {
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
