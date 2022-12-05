use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Event, Instance},
    casper_env::caller,
    Address,
    DocumentHash,
    Error,
    Mapping,
};
use casper_types::U256;
use delegate::delegate;

use crate::{
    voting::{
        types::VotingId,
        voting::{Voting, VotingType},
        Ballot,
        Choice,
        GovernanceVoting,
    },
    DaoConfigurationBuilder,
};

#[casper_contract_interface]
pub trait SimpleVoterContractInterface {
    /// see [GovernanceVoting](GovernanceVoting::init())
    fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
    /// Creates new SimpleVoter voting.
    ///
    /// `variable_repo_to_edit` takes an [Address](Address) of a [Variable Repo](crate::VariableRepositoryContract) instance that will be updated
    ///
    /// `key`, `value` and `activation_time` are parameters that will be passed to `update_at` method of a [Variable Repo](crate::VariableRepositoryContract)
    fn create_voting(&mut self, document_hash: DocumentHash, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [GovernanceVoting](GovernanceVoting::get_dust_amount())
    fn get_dust_amount(&self) -> U256;
    /// see [GovernanceVoting](GovernanceVoting::get_variable_repo_address())
    fn variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_voting())
    fn get_voting(&self, voting_id: VotingId, voting_type: VotingType) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// Returns document hash being voted on for given voting id.
    fn get_document_hash(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
    ) -> Option<DocumentHash>;
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
    voting: GovernanceVoting,
    simple_votings: Mapping<VotingId, DocumentHash>,
}

impl SimpleVoterContractInterface for SimpleVoterContract {
    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
            fn get_dust_amount(&self) -> U256;
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn create_voting(&mut self, document_hash: DocumentHash, stake: U256) {
        let voting_configuration = DaoConfigurationBuilder::new(
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

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U256) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_voting(&self, voting_id: VotingId, voting_type: VotingType) -> Option<Voting> {
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
}
