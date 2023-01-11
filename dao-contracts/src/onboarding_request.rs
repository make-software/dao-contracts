use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Event, Instance},
    casper_env::caller,
    cspr,
    Address,
    BlockTime,
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
        VotingCreatedInfo,
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
    fn create_voting(&mut self, reason: DocumentHash, purse: URef);
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
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
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
            fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
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

    fn create_voting(&mut self, reason: DocumentHash, purse: URef) {
        let cspr_deposit = cspr::deposit_cspr(purse);
        let voting_info = self.onboarding.submit_request(reason.clone(), cspr_deposit);
        OnboardingVotingCreated::new(reason, cspr_deposit, voting_info).emit();
    }

    fn get_cspr_balance(&self) -> U512 {
        cspr::get_cspr_balance()
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

#[derive(Debug, PartialEq, Eq, Event)]
pub struct OnboardingVotingCreated {
    reason: DocumentHash,
    cspr_deposit: U512,
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

impl OnboardingVotingCreated {
    pub fn new(reason: DocumentHash, cspr_deposit: U512, info: VotingCreatedInfo) -> Self {
        Self {
            reason,
            cspr_deposit,
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
