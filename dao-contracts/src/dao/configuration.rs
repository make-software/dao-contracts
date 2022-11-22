use casper_dao_utils::BlockTime;
use casper_dao_utils::casper_dao_macros::{CLTyped, FromBytes, ToBytes};

pub trait DaoConfigurationTrait {
    fn ReputationConversionRate(&self) -> u32;
    fn FiatConversionRateAddress(&self) -> u32;
    fn ForumKYCRequired(&self) -> u32;
}

pub trait VotingConfigurationTrait {
    fn GovernanceQuorumRatio(&self) -> u32;
    fn GovernanceInformalQuorumRatio(&self) -> u32;
    fn InformalQuorumRatio(&self) -> u32;
    fn InformalStakeReputation(&self) -> u32;
    fn QuorumRatio(&self) -> u32;
    fn GovernanceInformalVotingTime(&self) -> u32;
    fn GovernanceVotingTime(&self) -> u32;
    fn InformalVotingTime(&self) -> u32;
    fn VotingTime(&self) -> u32;
    fn TimeBetweenInformalAndFormalVoting(&self) -> u32;
    fn GovernanceWalletAddress(&self) -> u32;
    fn DefaultReputationSlash(&self) -> u32;
    fn TimeDifferenceBetweenInformalAndFormal(&self) -> u32;
    fn VotingClearnessDelta(&self) -> u32;
    fn VotingStartAfterJobSubmition(&self) -> u32;
    fn GovernancePaymentRatio(&self) -> u32;
}

pub trait BidEscrowConfigurationTrait {
    fn PostJobDOSFee(&self) -> u32;
    fn InternalAuctionTime(&self) -> BlockTime;
    fn PublicAuctionTime(&self) -> BlockTime;
    fn DefaultPolicingRate(&self) -> u32;
    fn VABidAcceptanceTimeout(&self) -> u32;
    fn VACanBidOnPublicAuction(&self) -> u32;
    fn DistributePaymentToNonVoters(&self) -> u32;
}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct DaoConfiguration {
    
}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct BidEscrowConfiguration {

}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct VotingConfiguration {

}

impl BidEscrowConfigurationTrait for BidEscrowConfiguration {
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
}

impl VotingConfigurationTrait for VotingConfiguration {
    fn GovernanceQuorumRatio(&self) -> u32 {
        todo!()
    }

    fn GovernanceInformalQuorumRatio(&self) -> u32 {
        todo!()
    }

    fn InformalQuorumRatio(&self) -> u32 {
        todo!()
    }

    fn InformalStakeReputation(&self) -> u32 {
        todo!()
    }

    fn QuorumRatio(&self) -> u32 {
        todo!()
    }

    fn GovernanceInformalVotingTime(&self) -> u32 {
        todo!()
    }

    fn GovernanceVotingTime(&self) -> u32 {
        todo!()
    }

    fn InformalVotingTime(&self) -> u32 {
        todo!()
    }

    fn VotingTime(&self) -> u32 {
        todo!()
    }

    fn TimeBetweenInformalAndFormalVoting(&self) -> u32 {
        todo!()
    }

    fn GovernanceWalletAddress(&self) -> u32 {
        todo!()
    }

    fn DefaultReputationSlash(&self) -> u32 {
        todo!()
    }

    fn TimeDifferenceBetweenInformalAndFormal(&self) -> u32 {
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
}

impl DaoConfigurationTrait for DaoConfiguration {
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