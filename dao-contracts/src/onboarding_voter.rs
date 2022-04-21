use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    Address, Error, Sequence,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    proxy::reputation_proxy::ReputationContractProxy,
    voting::{
        kyc::KycInfo,
        onboarding::{self, OnboardingInfo},
        voting::Voting,
        Ballot, Choice, GovernanceVoting, VotingId,
    },
};
use delegate::delegate;

const ARG_TO: &str = "to";
const ARG_TOKEN_ID: &str = "token_id";
const ENTRY_POINT_MINT: &str = "mint";
const ENTRY_POINT_BURN: &str = "burn";

#[casper_contract_interface]
pub trait OnboardingVoterContractInterface {
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );

    // - Require no voting for a given `address` is in progress.
    // For Adding new VA:
    // - Check if VA is not onboarderd.
    // - Check if `address` is KYCed.
    // - Check if `address` has positive reputation amount.
    // For Removing existing VA:
    // - Check if VA is already onboarderd.
    // - Check if `address` has positive reputation amount.
    fn create_voting(&mut self, action: onboarding::Action, subject_address: Address, stake: U256);
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_kyc_token_address(&self) -> Address;
    fn get_va_token_address(&self) -> Address;

    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
}

#[derive(Instance)]
pub struct OnboardingVoterContract {
    onboarding: OnboardingInfo,
    kyc: KycInfo,
    voting: GovernanceVoting,
    sequence: Sequence,
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

    fn create_voting(&mut self, action: onboarding::Action, subject_address: Address, stake: U256) {
        self.assert_no_ongoing_voting(&subject_address);

        let (entry_point, runtime_args) = match action {
            onboarding::Action::Add => self.config_add_voting(subject_address),
            onboarding::Action::Remove => self.config_remove_voting(subject_address),
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
        self.voting.finish_voting(voting_id);
        self.onboarding.clear_voting(&address);
    }
}

// non-contract implementation
impl OnboardingVoterContract {
    fn config_remove_voting(&mut self, subject_address: Address) -> (String, RuntimeArgs) {
        self.assert_onboarded(&subject_address);
        self.assert_has_reputation(&subject_address);

        let token_id = self.onboarding.token_id_of(&subject_address).unwrap();

        let runtime_args = runtime_args! {
            ARG_TOKEN_ID => token_id,
        };
        let entry_point = ENTRY_POINT_BURN.to_string();

        (entry_point, runtime_args)
    }

    fn config_add_voting(&mut self, subject_address: Address) -> (String, RuntimeArgs) {
        self.assert_not_onboarded(&subject_address);
        self.assert_kyced(&subject_address);
        self.assert_has_reputation(&subject_address);

        let token_id = self.sequence.next_value();

        let runtime_args = runtime_args! {
            ARG_TO => subject_address,
            ARG_TOKEN_ID => token_id,
        };
        let entry_point = ENTRY_POINT_MINT.to_string();

        (entry_point, runtime_args)
    }

    fn extract_address_from_args(&self, voting_id: VotingId) -> Address {
        let voting = self
            .voting
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VoterDoesNotExist);

        let arg = voting
            .runtime_args()
            .named_args()
            .find(|arg| arg.name() == ARG_TO);

        if let Some(to_arg) = arg {
            return to_arg.cl_value().clone().into_t().unwrap();
        }

        let arg = voting
            .runtime_args()
            .named_args()
            .find(|arg| arg.name() == ARG_TOKEN_ID);

        if let Some(token_id_arg) = arg {
            let token_id = token_id_arg.cl_value().clone().into_t().unwrap();
            return self.onboarding.owner_of(token_id);
        }

        casper_env::revert(Error::UnexpectedOnboardingError)
    }

    fn assert_has_reputation(&self, address: &Address) {
        if !ReputationContractProxy::has_reputation(self.get_reputation_token_address(), address) {
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
