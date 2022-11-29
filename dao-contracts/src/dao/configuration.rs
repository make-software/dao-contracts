use std::{collections::HashMap, io::Bytes};

use casper_dao_utils::{casper_dao_macros::{CLTyped, FromBytes, ToBytes}, BlockTime, ContractCall, Address};
use casper_types::{U256, U512};

pub trait DaoConfigurationTrait {
    fn reputation_conversion_rate(&self) -> u32;
    fn fiat_conversion_rate_address(&self) -> u32;
    fn forum_kyc_required(&self) -> u32;
    fn governance_informal_quorum_ratio(&self) -> u32;
    fn governance_formal_quorum_ratio(&self) -> u32;
    fn governance_informal_voting_time(&self) -> BlockTime;
    fn governance_formal_voting_time(&self) -> BlockTime;
    fn informal_quorum_ratio(&self) -> u32;
    fn formal_quorum_ratio(&self) -> u32;
    fn formal_voting_quorum(&self) -> U256;
    fn informal_voting_quorum(&self) -> U256;
    fn informal_voting_time(&self) -> BlockTime;
    fn formal_voting_time(&self) -> BlockTime;
    fn time_between_informal_and_formal_voting(&self) -> BlockTime;
    fn governance_wallet_address(&self) -> u32;
    fn default_reputation_slash(&self) -> u32;
    fn voting_clearness_delta(&self) -> u32;
    fn voting_start_after_job_submition(&self) -> u32;
    fn governance_payment_ratio(&self) -> u32;
    fn post_job_dosfee(&self) -> u32;
    fn internal_auction_time(&self) -> BlockTime;
    fn public_auction_time(&self) -> BlockTime;
    fn default_policing_rate(&self) -> u32;
    fn vabid_acceptance_timeout(&self) -> u32;
    fn vacan_bid_on_public_auction(&self) -> bool;
    fn distribute_payment_to_non_voters(&self) -> u32;
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
    fn reputation_conversion_rate(&self) -> u32 {
        todo!()
    }

    fn fiat_conversion_rate_address(&self) -> u32 {
        todo!()
    }

    fn forum_kyc_required(&self) -> u32 {
        todo!()
    }

    fn governance_informal_quorum_ratio(&self) -> u32 {
        todo!()
    }

    fn governance_formal_quorum_ratio(&self) -> u32 {
        todo!()
    }

    fn governance_informal_voting_time(&self) -> BlockTime {
        // TODO: implement
        432000
    }

    fn governance_formal_voting_time(&self) -> BlockTime {
        todo!()
    }

    fn informal_quorum_ratio(&self) -> u32 {
        500
    }

    fn formal_quorum_ratio(&self) -> u32 {
        500
    }

    fn formal_voting_quorum(&self) -> U256 {
        U256::from(3)
    }

    fn informal_voting_quorum(&self) -> U256 {
        U256::from(3)
    }

    fn informal_voting_time(&self) -> BlockTime {
        // TODO: implement
        432000
    }

    fn formal_voting_time(&self) -> BlockTime {
        432000
    }

    fn time_between_informal_and_formal_voting(&self) -> BlockTime {
        self.time_between_informal_and_formal_voting
    }

    fn governance_wallet_address(&self) -> u32 {
        todo!()
    }

    fn default_reputation_slash(&self) -> u32 {
        todo!()
    }

    fn voting_clearness_delta(&self) -> u32 {
        todo!()
    }

    fn voting_start_after_job_submition(&self) -> u32 {
        todo!()
    }

    fn governance_payment_ratio(&self) -> u32 {
        todo!()
    }

    fn post_job_dosfee(&self) -> u32 {
        todo!()
    }

    fn internal_auction_time(&self) -> BlockTime {
        // TODO: implement
        604800
    }

    fn public_auction_time(&self) -> BlockTime {
        // TODO: implement
        864000
    }

    fn default_policing_rate(&self) -> u32 {
        300
    }

    fn vabid_acceptance_timeout(&self) -> u32 {
        todo!()
    }

    fn vacan_bid_on_public_auction(&self) -> bool {
        true
    }

    fn distribute_payment_to_non_voters(&self) -> u32 {
        todo!()
    }
}
