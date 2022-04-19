use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    Address, Error,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    proxy::reputation_proxy::ReputationContractProxy,
    voting::{
        kyc::KycInfo,
        onboarding::{self, OnboardingInfo},
        voting::Voting,
        GovernanceVoting, Vote, VotingId,
    },
};
use delegate::delegate;

#[casper_contract_interface]
pub trait OnboardingVoterContractInterface {
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );

    // - Require no voting for a given `address` is on.

    // For Adding new VA:
    // - Check if VA is not onboarderd.
    // - Check if `address` is KYCed.
    // - Check if `address` has positive reputation amount.

    // For Removing existing VA:
    // - Check if VA is already onboarderd.
    // - Check if `address` has positive reputation amount.
    fn create_voting(&mut self, action: onboarding::Action, subject_address: Address, stake: U256);
    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_kyc_token_address(&self) -> Address;
    fn get_va_token_address(&self) -> Address;

    fn get_voting(&self, voting_id: U256) -> Voting;
    fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
    fn get_voter(&self, voting_id: U256, at: u32) -> Address;
}

#[derive(Instance)]
pub struct OnboardingVoterContract {
    onboarding: OnboardingInfo,
    kyc: KycInfo,
    voting: GovernanceVoting,
}

impl OnboardingVoterContractInterface for OnboardingVoterContract {
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
            fn get_voting(&self, voting_id: U256) -> Voting;
            fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
            fn get_voter(&self, voting_id: U256, at: u32) -> Address;
        }
    }

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

    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256) {
        let voter = caller();
        self.voting.vote(voter, voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId) {
        self.voting.finish_voting(voting_id);
        let address = self.extract_address_from_args(voting_id);
        self.onboarding.clear_voting(&address);
    }
}

impl OnboardingVoterContract {
    fn config_remove_voting(&mut self, subject_address: Address) -> (String, RuntimeArgs) {
        self.assert_onboarded(&subject_address);

        let runtime_args = runtime_args! {};
        let entry_point = "burn".to_string();

        (entry_point, runtime_args)
    }

    fn config_add_voting(&mut self, subject_address: Address) -> (String, RuntimeArgs) {
        self.assert_not_onboarded(&subject_address);
        self.assert_kyced(&subject_address);
        self.assert_has_reputation(&subject_address);

        let runtime_args = runtime_args! {
            "to" => subject_address,
            "token_id" => U256::one(),
        };
        let entry_point = "mint".to_string();

        (entry_point, runtime_args)
    }

    fn extract_address_from_args(&self, voting_id: VotingId) -> Address {
        let voting = self.voting.get_voting(voting_id);
        let arg = voting
            .runtime_args()
            .named_args()
            .find(|arg| arg.name() == "to")
            .unwrap();

        arg.cl_value().clone().into_t().unwrap()
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
        if self.onboarding.exists_ongoing_voting(&address) {
            casper_env::revert(Error::OnboardingAlreadyInProgress);
        }
    }

    fn assert_onboarded(&self, address: &Address) {
        if !self.onboarding.is_onboarded(address) {
            casper_env::revert(Error::VaNotOnboarded);
        }
    }
}
