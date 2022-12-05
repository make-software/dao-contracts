use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    consts,
    Address,
    ContractCall,
    DocumentHash,
    Error,
};
use casper_types::{runtime_args, RuntimeArgs, U256};
use delegate::delegate;

use crate::{
    voting::{
        kyc_info::KycInfo,
        types::VotingId,
        voting::{Voting, VotingType},
        Ballot,
        Choice,
        GovernanceVoting,
    },
    DaoConfigurationBuilder,
};

#[casper_contract_interface]
pub trait KycVoterContractInterface {
    /// Contract constructor
    ///
    /// Initializes modules.
    ///
    /// See [GovernanceVoting](GovernanceVoting::init()), [KycInfo](KycInfo::init())
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        va_token: Address,
        kyc_token: Address,
    );
    /// Creates new kyc voting. Once the voting passes a kyc token is minted to the `subject_address`.
    ///
    /// # Prerequisites
    ///
    /// * no voting on the given `subject_address` is in progress,
    /// * `subject_address` does not own a kyc token.
    ///
    /// # Note
    ///
    /// `subject_address` - [address](Address) of a user to be verified.
    /// `document_hash` - a hash of a document that verify the user. The hash is used as an id of a freshly minted  kyc token.
    /// `subject_address` - an [Address](Address) to be on/offboarded.
    fn create_voting(&mut self, subject_address: Address, document_hash: DocumentHash, stake: U256);
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
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    /// see [GovernanceVoting](GovernanceVoting::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// see [KycInfo](KycInfo::get_kyc_token_address())
    fn get_kyc_token_address(&self) -> Address;
}

/// KycVoterContract
///
/// It is responsible for managing kyc tokens (see [DaoOwnedNftContract](crate::DaoOwnedNftContract).
///
/// When the voting passes, a kyc token is minted.
#[derive(Instance)]
pub struct KycVoterContract {
    kyc: KycInfo,
    voting: GovernanceVoting,
}

impl KycVoterContractInterface for KycVoterContract {
    delegate! {
        to self.kyc {
            fn get_kyc_token_address(&self) -> Address;
        }

        to self.voting {
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
            fn get_dust_amount(&self) -> U256;
        }
    }

    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        va_token: Address,
        kyc_token: Address,
    ) {
        self.kyc.init(kyc_token);
        self.voting.init(variable_repo, reputation_token, va_token);
    }

    fn create_voting(
        &mut self,
        subject_address: Address,
        _document_hash: DocumentHash,
        stake: U256,
    ) {
        self.assert_no_ongoing_voting(&subject_address);
        self.assert_not_kyced(&subject_address);

        let creator = caller();

        let voting_configuration = DaoConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .contract_call(ContractCall {
            address: self.get_kyc_token_address(),
            entry_point: consts::EP_MINT.to_string(),
            runtime_args: runtime_args! {
                consts::ARG_TO => subject_address,
            },
        })
        .build();

        self.voting
            .create_voting(creator, stake, voting_configuration);

        self.kyc.set_voting(&subject_address);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U256) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        let voter = caller();
        self.voting.vote(voter, voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let voting_id = self.voting.to_real_voting_id(voting_id, voting_type);
        let summary = self.voting.finish_voting(voting_id);
        // The voting is ended when:
        // 1. Informal voting has been rejected.
        // 2. Formal voting has been finish (regardless of the final result).
        if summary.is_voting_process_finished() {
            let voting = self
                .voting
                .get_voting(voting_id)
                .unwrap_or_revert_with(Error::VotingDoesNotExist);
            let address = self.extract_address_from_args(&voting);
            self.kyc.clear_voting(&address);
        }
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

    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool {
        self.voting.voting_exists(voting_id, voting_type)
    }
}

// non-contract implementation
impl KycVoterContract {
    fn extract_address_from_args(&self, voting: &Voting) -> Address {
        let runtime_args = voting
            .contract_call()
            .clone()
            .unwrap_or_revert()
            .runtime_args();
        let arg = runtime_args
            .named_args()
            .find(|arg| arg.name() == consts::ARG_TO)
            .unwrap_or_revert_with(Error::UnexpectedOnboardingError);

        arg.cl_value()
            .clone()
            .into_t()
            .unwrap_or_revert_with(Error::UnexpectedOnboardingError)
    }

    fn assert_not_kyced(&self, address: &Address) {
        if self.kyc.is_kycd(address) {
            casper_env::revert(Error::UserKycedAlready);
        }
    }

    fn assert_no_ongoing_voting(&self, address: &Address) {
        if self.kyc.exists_ongoing_voting(address) {
            casper_env::revert(Error::KycAlreadyInProgress);
        }
    }
}
