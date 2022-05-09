use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    consts, Address, Error, SequenceGenerator,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    voting::{
        kyc_info::KycInfo,
        onboarding_info::{OnboardingAction, OnboardingInfo},
        voting::Voting,
        Ballot, Choice, GovernanceVoting, VotingId,
    },
    ReputationContractCaller,
};
use delegate::delegate;

#[casper_contract_interface]
pub trait OnboardingVoterContractInterface {
    /// Contract constructor
    /// 
    /// Initializes modules. 
    /// 
    /// See [GovernanceVoting](GovernanceVoting::init()), [KycInfo](KycInfo::init()), [OnboardingInfo](OnboardingInfo::init())
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );
    /// Creates new Onboarding\Offboarding voting.
    /// 
    /// # Prerequisites
    /// 
    /// * no voting on the given `subject_address` is in progress.
    /// 
    /// To onboard a new VA:
    /// * `subject_address` must not be already onboarded,
    /// * `subject_address` must be KYCd,
    /// * `subject_address` must have a positive reputation token amount.
    /// 
    /// To offboard a VA:
    /// * `subject_address` must be already onboarded,
    /// * `subject_address` must have a positive reputation token amount.
    /// 
    /// # Note
    ///
    /// `action` - the mode of newly created voting can be either onboarding ([Add](OnboardingAction::Add)) or offboarding ([Remove](OnboardingAction::Remove)). 
    /// 
    /// `subject_address` - an [Address](Address) to be on/offboarded.
    fn create_voting(&mut self, action: OnboardingAction, subject_address: Address, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting::vote())
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    /// see [GovernanceVoting](GovernanceVoting::finish_voting())
    fn finish_voting(&mut self, voting_id: VotingId);
    /// see [GovernanceVoting](GovernanceVoting::get_dust_amount())
    fn get_dust_amount(&self) -> U256;
    /// see [GovernanceVoting](GovernanceVoting::get_variable_repo_address())
    fn get_variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_reputation_token_address())
    fn get_reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting::get_voting())
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting::get_ballot())
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting::get_voter())
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
    /// see [KycInfo](KycInfo::get_kyc_token_address())
    fn get_kyc_token_address(&self) -> Address;
    /// see [OnboardingInfo](OnboardingInfo::get_va_token_address())
    fn get_va_token_address(&self) -> Address;
}


/// OnboardingVoterContract
///
/// It is responsible for managing va tokens (see [DaoOwnedNftContract](crate::DaoOwnedNftContract).
///
/// When the voting passes, a va token is minted (onboarding) or burned (offboarding).
#[derive(Instance)]
pub struct OnboardingVoterContract {
    onboarding: OnboardingInfo,
    kyc: KycInfo,
    voting: GovernanceVoting,
    sequence: SequenceGenerator,
}

impl OnboardingVoterContractInterface for OnboardingVoterContract {
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.onboarding.init(va_token);
        self.kyc.init(kyc_token);
        self.voting.init(variable_repo, reputation_token);
    }

    delegate! {
        to self.onboarding {
            fn get_va_token_address(&self) -> Address;
        }

        to self.kyc {
            fn get_kyc_token_address(&self) -> Address;
        }

        to self.voting {
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_dust_amount(&self) -> U256;
            fn get_voting(&self, voting_id: U256) -> Option<Voting>;
            fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
            fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
        }
    }

    fn create_voting(&mut self, action: OnboardingAction, subject_address: Address, stake: U256) {
        self.assert_no_ongoing_voting(&subject_address);

        let (entry_point, runtime_args) = match action {
            OnboardingAction::Add => self.configure_add_voting(subject_address),
            OnboardingAction::Remove => self.configure_remove_voting(subject_address),
        };
        let creator = caller();
        let contract_to_call = self.get_va_token_address();

        self.voting
            .create_voting(creator, stake, contract_to_call, entry_point, runtime_args);
        self.onboarding.set_voting(&subject_address);
    }

    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256) {
        let voter = caller();
        self.voting.vote(voter, voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId) {
        let address = self.extract_address_from_args(voting_id);
        let summary = self.voting.finish_voting(voting_id);
        // The voting is ended when:
        // 1. Informal voting has been rejected.
        // 2. Formal voting has been finish (regardless of the final result).
        if summary.is_voting_process_finished() {
            self.onboarding.clear_voting(&address);
        }
    }
}

// non-public implementation
impl OnboardingVoterContract {
    fn configure_remove_voting(&mut self, subject_address: Address) -> (String, RuntimeArgs) {
        self.assert_onboarded(&subject_address);
        self.assert_has_reputation(&subject_address);

        let token_id = self.onboarding.token_id_of(&subject_address);

        let runtime_args = runtime_args! {
            consts::ARG_TOKEN_ID => token_id,
        };
        let entry_point = consts::EP_BURN.to_string();

        (entry_point, runtime_args)
    }

    fn configure_add_voting(&mut self, subject_address: Address) -> (String, RuntimeArgs) {
        self.assert_not_onboarded(&subject_address);
        self.assert_kyced(&subject_address);
        self.assert_has_reputation(&subject_address);

        let token_id = self.sequence.next_value();

        let runtime_args = runtime_args! {
            consts::ARG_TO => subject_address,
            consts::ARG_TOKEN_ID => token_id,
        };
        let entry_point = consts::EP_MINT.to_string();

        (entry_point, runtime_args)
    }

    fn extract_address_from_args(&self, voting_id: VotingId) -> Address {
        let voting = self
            .voting
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        // If the action is `Add` we pass an `Address` as the `to` parameter
        let arg = voting
            .runtime_args()
            .named_args()
            .find(|arg| arg.name() == consts::ARG_TO);

        if let Some(to_arg) = arg {
            return to_arg
                .cl_value()
                .clone()
                .into_t()
                .unwrap_or_revert_with(Error::UnexpectedOnboardingError);
        }

        // If the action is `Remove` do not pass any `Address` but `token_id`
        // Having that we can obtain the `Address` by getting the token owner
        let arg = voting
            .runtime_args()
            .named_args()
            .find(|arg| arg.name() == consts::ARG_TOKEN_ID);

        if let Some(token_id_arg) = arg {
            let token_id = token_id_arg
                .cl_value()
                .clone()
                .into_t()
                .unwrap_or_revert_with(Error::UnexpectedOnboardingError);
            return self
                .onboarding
                .owner_of(token_id)
                .unwrap_or_revert_with(Error::UnexpectedOnboardingError);
        }

        // If the voting was created with some unexpected args an error is thrown
        casper_env::revert(Error::UnexpectedOnboardingError)
    }

    fn assert_has_reputation(&self, address: &Address) {
        let caller = ReputationContractCaller::at(self.get_reputation_token_address());
        if !caller.has_reputation(address) {
            casper_env::revert(Error::InsufficientBalance)
        }
    }

    fn assert_kyced(&self, address: &Address) {
        if !self.kyc.is_kycd(address) {
            casper_env::revert(Error::VaNotKyced);
        }
    }

    fn assert_not_onboarded(&self, address: &Address) {
        if self.onboarding.is_onboarded(address) {
            casper_env::revert(Error::VaOnboardedAlready);
        }
    }

    fn assert_no_ongoing_voting(&self, address: &Address) {
        if self.onboarding.exists_ongoing_voting(address) {
            casper_env::revert(Error::OnboardingAlreadyInProgress);
        }
    }

    fn assert_onboarded(&self, address: &Address) {
        if !self.onboarding.is_onboarded(address) {
            casper_env::revert(Error::VaNotOnboarded);
        }
    }
}
