use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::contract_api::system::get_purse_balance,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    Address,
    DocumentHash,
};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::{
    escrow::onboarding::Onboarding,
    voting::{
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
        VotingId,
    },
};

#[casper_contract_interface]
pub trait OnboardingRequestContractInterface {
    /// Initializes the module with [Addresses](Address) of [Reputation Token](crate::ReputationContract), [Variable Repo](crate::VariableRepositoryContract)
    /// KYC Token and VA Token
    ///
    /// # Events
    /// Emits [`VotingContractCreated`](crate::voting::voting_engine::events::VotingContractCreated)
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );

    /// Submits onboarding request. If the request is valid voting starts.
    fn submit_onboarding_request(&mut self, reason: DocumentHash, purse: URef);
    /// Casts a vote over a job
    /// # Events
    /// Emits [`BallotCast`](crate::voting::voting_engine::events::BallotCast)

    /// # Errors
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// Finishes voting stage. Depending on stage, the voting can be converted to a formal one, end
    /// with a refund or convert the requestor to a VA.
    /// # Events
    /// Emits [`VotingEnded`](crate::voting::voting_engine::events::VotingEnded), [`VotingCreated`](crate::voting::voting_engine::events::VotingCreated)
    /// # Errors
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn finish_voting(&mut self, voting_id: VotingId);
    /// see [VotingEngine](VotingEngine)
    fn variable_repo_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine)
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine)
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine)
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine)
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;

    /// Returns the CSPR balance of the contract
    fn get_cspr_balance(&self) -> U512;

    // Whitelisting set.
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;

    // Slashing
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
}

#[derive(Instance)]
pub struct OnboardingRequestContract {
    voting: VotingEngine,
    access_control: AccessControl,
    onboarding: Onboarding,
}

impl OnboardingRequestContractInterface for OnboardingRequestContract {
    delegate! {
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

        to self.onboarding {
            #[call(submit_request)]
            fn submit_onboarding_request(&mut self, reason: DocumentHash, purse: URef);
            fn finish_voting(&mut self, voting_id: VotingId);
            fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
        }
    }

    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.voting.init(variable_repo, reputation_token, va_token);
        self.onboarding.init(va_token, kyc_token);
        self.access_control.init(caller());
    }

    fn get_cspr_balance(&self) -> U512 {
        get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
    }

    fn slash_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }
}

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

#[cfg(feature = "test-support")]
impl OnboardingRequestContractTest {
    pub fn submit_onboarding_request_with_cspr_amount(
        &mut self,
        reason: DocumentHash,
        cspr_amount: U512,
    ) -> Result<(), casper_dao_utils::Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "submit_onboarding_request.wasm",
            runtime_args! {
                "onboarding_address" => self.address(),
                "reason" => reason,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }
}
