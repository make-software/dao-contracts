use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Event, Instance},
    casper_env::caller,
    Address,
    DocumentHash,
    Error,
    Mapping,
};
use casper_types::U512;
use delegate::delegate;

use crate::{
    voting::{
        types::VotingId,
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
    },
    ConfigurationBuilder,
};

#[casper_contract_interface]
pub trait SimpleVoterContractInterface {
    /// see [VotingEngine](VotingEngine::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
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
    fn variable_repo_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn get_voting(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<VotingStateMachine>;
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
    fn get_document_hash(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<DocumentHash>;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
}

/// Event thrown after SimpleVoting is created
#[derive(Debug, PartialEq, Eq, Event)]
pub struct SimpleVotingCreated {
    pub document_hash: DocumentHash,
    pub voting_id: VotingId,
}

/// SimpleVoterContract
///
/// It is responsible for votings that do not perform any actions on the blockchain.
///
/// The topic of the voting is handled by `document_hash` which is a hash of a document being voted on.
#[derive(Instance)]
pub struct SimpleVoterContract {
    voting: VotingEngine,
    simple_votings: Mapping<VotingId, DocumentHash>,
    access_control: AccessControl,
}

impl SimpleVoterContractInterface for SimpleVoterContract {
    delegate! {
        to self.voting {
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
        }
    }

    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address) {
        self.voting.init(variable_repo, reputation_token, va_token);
        self.access_control.init(caller())
    }

    fn create_voting(&mut self, document_hash: DocumentHash, stake: U512) {
        let voting_configuration = ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .build();

        let voting_id = self
            .voting
            .create_voting(caller(), stake, voting_configuration);

        self.simple_votings.set(&voting_id, document_hash.clone());

        SimpleVotingCreated {
            document_hash,
            voting_id,
        }
        .emit();
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        let voting_summary = self.voting.finish_voting(voting_id);

        if let VotingType::Informal = voting_summary.voting_type() {
            match voting_summary.formal_voting_id() {
                None => {}
                // Informal voting ended in favor, creating a new formal voting
                Some(formal_voting_id) => {
                    self.simple_votings.set(
                        &formal_voting_id,
                        self.simple_votings
                            .get(&voting_id)
                            .unwrap_or_revert_with(Error::VariableValueNotSet),
                    );
                }
            }
        }
    }

    fn get_document_hash(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<DocumentHash> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.simple_votings.get(&voting_id)
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_voting(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<VotingStateMachine> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_voting(voting_id)
    }

    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_ballot(voting_id, address)
    }

    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address> {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.get_voter(voting_id, at)
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }
}
