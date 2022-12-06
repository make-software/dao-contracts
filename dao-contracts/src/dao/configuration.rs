use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
    ContractCall,
};
use casper_types::{U256, U512};

pub trait DaoConfigurationTrait {
    fn reputation_conversion_rate(&self) -> U256;
    fn fiat_conversion_rate_address(&self) -> Address;
    fn forum_kyc_required(&self) -> bool;
    fn governance_informal_quorum_ratio(&self) -> U256;
    fn governance_formal_quorum_ratio(&self) -> U256;
    fn governance_informal_voting_time(&self) -> BlockTime;
    fn governance_formal_voting_time(&self) -> BlockTime;
    fn informal_quorum_ratio(&self) -> U256;
    fn formal_quorum_ratio(&self) -> U256;
    fn governance_formal_voting_quorum(&self) -> u32;
    fn governance_informal_voting_quorum(&self) -> u32;
    fn formal_voting_quorum(&self) -> u32;
    fn informal_voting_quorum(&self) -> u32;
    fn informal_voting_time(&self) -> BlockTime;
    fn formal_voting_time(&self) -> BlockTime;
    fn informal_stake_reputation(&self) -> bool;
    fn time_between_informal_and_formal_voting(&self) -> BlockTime;
    fn governance_wallet_address(&self) -> Address;
    fn default_reputation_slash(&self) -> U256;
    fn voting_clearness_delta(&self) -> U256;
    fn voting_start_after_job_submition(&self) -> BlockTime;
    fn governance_payment_ratio(&self) -> U512;
    fn post_job_dos_fee(&self) -> U512;
    fn internal_auction_time(&self) -> BlockTime;
    fn public_auction_time(&self) -> BlockTime;
    fn default_policing_rate(&self) -> U256;
    fn va_bid_acceptance_timeout(&self) -> BlockTime;
    fn va_can_bid_on_public_auction(&self) -> bool;
    fn distribute_payment_to_non_voters(&self) -> bool;
    fn total_onboarded(&self) -> U256;
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct DaoConfiguration {
    pub post_job_dos_fee: U512,
    pub internal_auction_time: BlockTime,
    pub public_auction_time: BlockTime,
    pub default_policing_rate: U256,
    pub reputation_conversion_rate: U256,
    pub fiat_conversion_rate_address: Address,
    pub forum_kyc_required: bool,
    pub governance_informal_quorum_ratio: U256,
    pub governance_formal_quorum_ratio: U256,
    pub informal_quorum_ratio: U256,
    pub formal_quorum_ratio: U256,
    pub governance_informal_voting_time: BlockTime,
    pub governance_formal_voting_time: BlockTime,
    pub informal_voting_time: BlockTime,
    pub formal_voting_time: BlockTime,
    pub informal_stake_reputation: bool,
    pub time_between_informal_and_formal_voting: BlockTime,
    pub va_bid_acceptance_timeout: BlockTime,
    pub va_can_bid_on_public_auction: bool,
    pub distribute_payment_to_non_voters: bool,
    pub governance_wallet_address: Address,
    pub default_reputation_slash: U256,
    pub voting_clearness_delta: U256,
    pub voting_start_after_job_worker_submission: BlockTime,
    pub governance_payment_ratio: U512,
    pub total_onboarded: U256,
    pub contract_call: Option<ContractCall>,
    pub only_va_can_create: bool,
    pub unbounded_tokens_for_creator: bool,
    pub onboard_creator: bool,
}

impl DaoConfigurationTrait for DaoConfiguration {
    fn reputation_conversion_rate(&self) -> U256 {
        self.reputation_conversion_rate
    }

    fn fiat_conversion_rate_address(&self) -> Address {
        self.fiat_conversion_rate_address
    }

    fn forum_kyc_required(&self) -> bool {
        self.forum_kyc_required
    }

    fn governance_informal_quorum_ratio(&self) -> U256 {
        self.governance_informal_quorum_ratio
    }

    fn governance_formal_quorum_ratio(&self) -> U256 {
        self.governance_formal_quorum_ratio
    }

    fn governance_informal_voting_time(&self) -> BlockTime {
        self.governance_informal_voting_time
    }

    fn governance_formal_voting_time(&self) -> BlockTime {
        self.governance_formal_voting_time
    }

    fn informal_quorum_ratio(&self) -> U256 {
        self.informal_quorum_ratio
    }

    fn formal_quorum_ratio(&self) -> U256 {
        self.formal_quorum_ratio
    }

    fn governance_formal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.governance_formal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U256::from(1000))
            .unwrap()
            .as_u32()
    }

    fn governance_informal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.governance_informal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U256::from(1000))
            .unwrap()
            .as_u32()
    }

    fn formal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.formal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U256::from(1000))
            .unwrap()
            .as_u32()
    }

    fn informal_voting_quorum(&self) -> u32 {
        // TODO: make the math not fail and reusable
        self.informal_quorum_ratio()
            .checked_mul(self.total_onboarded())
            .unwrap()
            .checked_div(U256::from(1000))
            .unwrap()
            .as_u32()
    }

    fn informal_voting_time(&self) -> BlockTime {
        self.informal_voting_time
    }

    fn formal_voting_time(&self) -> BlockTime {
        self.formal_voting_time
    }

    fn informal_stake_reputation(&self) -> bool {
        self.informal_stake_reputation
    }

    fn time_between_informal_and_formal_voting(&self) -> BlockTime {
        self.time_between_informal_and_formal_voting
    }

    fn governance_wallet_address(&self) -> Address {
        self.governance_wallet_address
    }

    fn default_reputation_slash(&self) -> U256 {
        self.default_reputation_slash
    }

    fn voting_clearness_delta(&self) -> U256 {
        self.voting_clearness_delta
    }

    fn voting_start_after_job_submition(&self) -> BlockTime {
        self.voting_start_after_job_worker_submission
    }

    fn governance_payment_ratio(&self) -> U512 {
        self.governance_payment_ratio
    }

    fn post_job_dos_fee(&self) -> U512 {
        self.post_job_dos_fee
    }

    fn internal_auction_time(&self) -> BlockTime {
        self.internal_auction_time
    }

    fn public_auction_time(&self) -> BlockTime {
        self.public_auction_time
    }

    fn default_policing_rate(&self) -> U256 {
        self.default_policing_rate
    }

    fn va_bid_acceptance_timeout(&self) -> BlockTime {
        self.va_bid_acceptance_timeout
    }

    fn va_can_bid_on_public_auction(&self) -> bool {
        self.va_can_bid_on_public_auction
    }

    fn distribute_payment_to_non_voters(&self) -> bool {
        self.distribute_payment_to_non_voters
    }

    fn total_onboarded(&self) -> U256 {
        self.total_onboarded
    }
}
