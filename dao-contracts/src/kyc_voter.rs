use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Event, Instance},
    casper_env::{self, caller},
    consts,
    Address,
    BlockTime,
    ContractCall,
    DocumentHash,
    Error,
};
use casper_types::{runtime_args, RuntimeArgs, U512};
use delegate::delegate;

use crate::{
    voting::{
        kyc_info::KycInfo,
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
pub trait KycVoterContractInterface {
    /// Contract constructor
    ///
    /// Initializes modules.
    ///
    /// See [VotingEngine](VotingEngine::init()), [KycInfo](KycInfo::init())
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
    fn create_voting(&mut self, subject_address: Address, document_hash: DocumentHash, stake: U512);
    /// see [VotingEngine](VotingEngine::vote())
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// see [VotingEngine](VotingEngine::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// see [VotingEngine](VotingEngine::get_variable_repo_address())
    fn variable_repo_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_reputation_token_address())
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine::get_voting())
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    /// see [VotingEngine](VotingEngine::get_ballot())
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine::get_voter())
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    /// see [KycInfo](KycInfo::get_kyc_token_address())
    fn get_kyc_token_address(&self) -> Address;

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);

    // Whitelisting set.
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;

    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
}

/// KycVoterContract
///
/// It is responsible for managing kyc tokens (see [DaoOwnedNftContract](crate::DaoOwnedNftContract).
///
/// When the voting passes, a kyc token is minted.
#[derive(Instance)]
pub struct KycVoterContract {
    kyc: KycInfo,
    access_control: AccessControl,
    voting: VotingEngine,
}

impl KycVoterContractInterface for KycVoterContract {
    delegate! {
        to self.kyc {
            fn get_kyc_token_address(&self) -> Address;
        }

        to self.voting {
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
        }

        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
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
        self.access_control.init(caller());
    }

    fn create_voting(
        &mut self,
        subject_address: Address,
        document_hash: DocumentHash,
        stake: U512,
    ) {
        self.assert_no_ongoing_voting(&subject_address);
        self.assert_not_kyced(&subject_address);

        let creator = caller();

        let voting_configuration = ConfigurationBuilder::new(
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

        let info = self
            .voting
            .create_voting(creator, stake, voting_configuration);

        KycVotingCreated::new(subject_address, document_hash, info).emit();

        self.kyc.set_voting(&subject_address);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        self.voting
            .vote(caller(), voting_id, voting_type, choice, stake);
    }

    // TODO: Store action in Mapping instead of extracting it from args of the call.
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let summary = self.voting.finish_voting(voting_id, voting_type);
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

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }
}

// non-contract implementation
impl KycVoterContract {
    fn extract_address_from_args(&self, voting: &VotingStateMachine) -> Address {
        let runtime_args = voting
            .contract_calls()
            .first()
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

#[derive(Debug, PartialEq, Eq, Event)]
pub struct KycVotingCreated {
    subject_address: Address,
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

impl KycVotingCreated {
    pub fn new(
        subject_address: Address,
        document_hash: DocumentHash,
        info: VotingCreatedInfo,
    ) -> Self {
        Self {
            subject_address,
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
