use std::{collections::HashMap, io::Bytes};

use casper_dao_utils::{casper_dao_macros::{CLTyped, FromBytes, ToBytes}, BlockTime, ContractCall, Address};
use casper_types::{U256, U512};

pub trait DaoConfigurationTrait {
    fn reputation_conversion_rate(&self) -> u32;
    fn FiatConversionRateAddress(&self) -> u32;
    fn ForumKYCRequired(&self) -> u32;
    fn GovernanceInformalQuorumRatio(&self) -> u32;
    fn GovernanceFormalQuorumRatio(&self) -> u32;
    fn GovernanceInformalVotingTime(&self) -> BlockTime;
    fn GovernanceFormalVotingTime(&self) -> BlockTime;
    fn InformalQuorumRatio(&self) -> u32;
    fn FormalQuorumRatio(&self) -> u32;
    fn formalVotingQuorum(&self) -> U256;
    fn informalVotingQuorum(&self) -> U256;
    fn InformalVotingTime(&self) -> BlockTime;
    fn FormalVotingTime(&self) -> BlockTime;
    fn TimeBetweenInformalAndFormalVoting(&self) -> BlockTime;
    fn GovernanceWalletAddress(&self) -> u32;
    fn DefaultReputationSlash(&self) -> u32;
    fn VotingClearnessDelta(&self) -> u32;
    fn VotingStartAfterJobSubmition(&self) -> u32;
    fn GovernancePaymentRatio(&self) -> u32;
    fn PostJobDOSFee(&self) -> u32;
    fn InternalAuctionTime(&self) -> BlockTime;
    fn PublicAuctionTime(&self) -> BlockTime;
    fn DefaultPolicingRate(&self) -> u32;
    fn VABidAcceptanceTimeout(&self) -> u32;
    fn VACanBidOnPublicAuction(&self) -> bool;
    fn DistributePaymentToNonVoters(&self) -> u32;
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

    fn FiatConversionRateAddress(&self) -> u32 {
        todo!()
    }

    fn ForumKYCRequired(&self) -> u32 {
        todo!()
    }

    fn GovernanceInformalQuorumRatio(&self) -> u32 {
        todo!()
    }

    fn GovernanceFormalQuorumRatio(&self) -> u32 {
        todo!()
    }

    fn GovernanceInformalVotingTime(&self) -> BlockTime {
        // TODO: implement
        432000
    }

    fn GovernanceFormalVotingTime(&self) -> BlockTime {
        todo!()
    }

    fn InformalQuorumRatio(&self) -> u32 {
        500
    }

    fn FormalQuorumRatio(&self) -> u32 {
        500
    }

    fn formalVotingQuorum(&self) -> U256 {
        U256::from(3)
    }

    fn informalVotingQuorum(&self) -> U256 {
        U256::from(3)
    }

    fn InformalVotingTime(&self) -> BlockTime {
        // TODO: implement
        432000
    }

    fn FormalVotingTime(&self) -> BlockTime {
        432000
    }

    fn TimeBetweenInformalAndFormalVoting(&self) -> BlockTime {
        self.time_between_informal_and_formal_voting
    }

    fn GovernanceWalletAddress(&self) -> u32 {
        todo!()
    }

    fn DefaultReputationSlash(&self) -> u32 {
        todo!()
    }

    fn VotingClearnessDelta(&self) -> u32 {
        todo!()
    }

    fn VotingStartAfterJobSubmition(&self) -> u32 {
        todo!()
    }

    fn GovernancePaymentRatio(&self) -> u32 {
        todo!()
    }

    fn PostJobDOSFee(&self) -> u32 {
        todo!()
    }

    fn InternalAuctionTime(&self) -> BlockTime {
        // TODO: implement
        604800
    }

    fn PublicAuctionTime(&self) -> BlockTime {
        // TODO: implement
        864000
    }

    fn DefaultPolicingRate(&self) -> u32 {
        300
    }

    fn VABidAcceptanceTimeout(&self) -> u32 {
        todo!()
    }

    fn VACanBidOnPublicAuction(&self) -> bool {
        true
    }

    fn DistributePaymentToNonVoters(&self) -> u32 {
        todo!()
    }
}
