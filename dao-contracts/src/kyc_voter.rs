use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller},
    Address, Error, SequenceGenerator,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::voting::{kyc::KycInfo, voting::Voting, Ballot, Choice, GovernanceVoting, VotingId};
use delegate::delegate;

const ARG_TO: &str = "to";
const ARG_TOKEN_ID: &str = "token_id";
const ENTRY_POINT_MINT: &str = "mint";

#[casper_contract_interface]
pub trait KycVoterContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, kyc_token: Address);
    // Require no voting for a given `address` is on.
    // Precondition: KycNft.balance_of(address_to_onboard) == 0;
    // Action: KycNft.mint(address_to_onboard, next_token_id)
    fn create_voting(&mut self, address_to_onboard: Address, document_hash: String, stake: U256);
    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_kyc_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
}

#[derive(Instance)]
pub struct KycVoterContract {
    kyc: KycInfo,
    voting: GovernanceVoting,
    sequence: SequenceGenerator,
}

impl KycVoterContractInterface for KycVoterContract {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, kyc_token: Address) {
        self.kyc.init(kyc_token);
        self.voting.init(variable_repo, reputation_token);
    }

    delegate! {
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

    fn create_voting(&mut self, address_to_onboard: Address, document_hash: String, stake: U256) {
        self.assert_no_ongoing_voting(&address_to_onboard);

        let creator = caller();
        let contract_to_call = self.get_kyc_token_address();
        self.assert_kyced(&address_to_onboard);

        let token_id = self.sequence.next_value();

        let runtime_args = runtime_args! {
            ARG_TO => address_to_onboard,
            ARG_TOKEN_ID => token_id,
        };
        let entry_point = ENTRY_POINT_MINT.to_string();
        self.voting
            .create_voting(creator, stake, contract_to_call, entry_point, runtime_args);
        self.kyc.set_voting(&address_to_onboard);
    }

    fn vote(&mut self, voting_id: VotingId, choice: Choice, stake: U256) {
        let voter = caller();
        self.voting.vote(voter, voting_id, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId) {
        let address = self.extract_address_from_args(voting_id);
        self.voting.finish_voting(voting_id);
        self.kyc.clear_voting(&address);
    }
}

// non-contract implementation
impl KycVoterContract {
    fn extract_address_from_args(&self, voting_id: VotingId) -> Address {
        let voting = self
            .voting
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        let arg = voting
            .runtime_args()
            .named_args()
            .find(|arg| arg.name() == ARG_TO)
            .unwrap_or_revert_with(Error::UnexpectedOnboardingError);

        arg.cl_value()
            .clone()
            .into_t()
            .unwrap_or_revert_with(Error::UnexpectedOnboardingError)
    }

    fn assert_kyced(&self, address: &Address) {
        if !self.kyc.is_kycd(address) {
            casper_env::revert(Error::VaNotKyced);
        }
    }

    fn assert_no_ongoing_voting(&self, address: &Address) {
        if self.kyc.exists_ongoing_voting(address) {
            casper_env::revert(Error::OnboardingAlreadyInProgress);
        }
    }
}
