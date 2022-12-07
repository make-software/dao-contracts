use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::{U512};

use crate::bid::types::{BidId, JobOfferId};

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub enum BidAuctionTime {
    InternalAuction,
    PublicAuction,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, PartialEq)]
pub enum BidStatus {
    Created,
    Selected,
    Rejected,
    Cancelled,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct Bid {
    pub bid_id: BidId,
    pub status: BidStatus,
    pub timestamp: BlockTime,
    pub job_offer_id: JobOfferId,
    pub proposed_timeframe: BlockTime,
    pub proposed_payment: U512,
    pub reputation_stake: U512,
    pub cspr_stake: Option<U512>,
    pub onboard: bool,
    pub worker: Address,
}

impl Bid {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bid_id: BidId,
        timestamp: BlockTime,
        job_offer_id: JobOfferId,
        proposed_timeframe: BlockTime,
        proposed_payment: U512,
        reputation_stake: U512,
        cspr_stake: Option<U512>,
        onboard: bool,
        worker: Address,
    ) -> Self {
        Self {
            bid_id,
            status: BidStatus::Created,
            timestamp,
            job_offer_id,
            proposed_timeframe,
            proposed_payment,
            reputation_stake,
            cspr_stake,
            onboard,
            worker,
        }
    }

    pub fn cancel(&mut self) {
        self.status = BidStatus::Cancelled;
    }

    pub fn pick(&mut self) {
        self.status = BidStatus::Selected;
    }

    pub fn reject(&mut self) {
        self.status = BidStatus::Rejected;
    }
}
