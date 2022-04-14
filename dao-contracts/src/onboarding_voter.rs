use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::voting::{
    onboarding::{self, OnboardingContractStorage},
    voting::Voting,
    GovernanceVoting, Vote, VotingId,
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
    storage: OnboardingContractStorage,
    voting: GovernanceVoting,
}

impl OnboardingVoterContractInterface for OnboardingVoterContract {
    delegate! {
        to self.storage {
            fn get_kyc_token_address(&self) -> Address;
            fn get_va_token_address(&self) -> Address;
        }

        to self.voting {
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_dust_amount(&self) -> U256;
            fn finish_voting(&mut self, voting_id: VotingId);
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
        self.storage.init(kyc_token, va_token);
        self.voting.init(variable_repo, reputation_token);
    }

    fn create_voting(&mut self, action: onboarding::Action, subject_address: Address, stake: U256) {
        let creator = caller();
        let va_token_address = self.get_va_token_address();

        let entry_point = match action {
            onboarding::Action::Add => "mint",
            onboarding::Action::Remove => "burn",
        }
        .to_string();

        let runtime_args = match action {
            onboarding::Action::Add => runtime_args! {
                "to" => subject_address,
                "token_id" => U256::one(),
            },
            onboarding::Action::Remove => runtime_args! {},
        };

        self.voting
            .create_voting(creator, stake, va_token_address, entry_point, runtime_args);
    }

    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256) {
        let voter = caller();
        self.voting.vote(voter, voting_id, choice, stake);
    }
}
