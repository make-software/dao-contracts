use std::collections::HashMap;
use std::io::Bytes;
use casper_types::U256;
use casper_dao_utils::{casper_dao_macros::{CLTyped, FromBytes, ToBytes}, BlockTime, ContractCall};

pub trait DaoConfigurationTrait {
    fn ReputationConversionRate(&self) -> u32;
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
    fn VACanBidOnPublicAuction(&self) -> u32;
    fn DistributePaymentToNonVoters(&self) -> u32;
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct DaoConfiguration {
    pub contract_call: Option<ContractCall>,
    pub only_va_can_create: bool,
    pub unbounded_tokens_for_creator: bool,
    pub onboard_creator: bool,
}

impl DaoConfigurationTrait for DaoConfiguration {
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
        todo!()
    }

    fn VABidAcceptanceTimeout(&self) -> u32 {
        todo!()
    }

    fn VACanBidOnPublicAuction(&self) -> u32 {
        todo!()
    }

    fn DistributePaymentToNonVoters(&self) -> u32 {
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
        todo!()
    }

    fn FormalQuorumRatio(&self) -> u32 {
        todo!()
    }

    fn formalVotingQuorum(&self) -> U256 {
        todo!()
    }

    fn informalVotingQuorum(&self) -> U256 {
        todo!()
    }

    fn InformalVotingTime(&self) -> BlockTime {
        // TODO: implement
        432000
    }

    fn FormalVotingTime(&self) -> BlockTime {
        todo!()
    }

    fn TimeBetweenInformalAndFormalVoting(&self) -> BlockTime {
        // TODO: implement
        86400
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

    fn ReputationConversionRate(&self) -> u32 {
        todo!()
    }

    fn FiatConversionRateAddress(&self) -> u32 {
        todo!()
    }

    fn ForumKYCRequired(&self) -> u32 {
        todo!()
    }
}