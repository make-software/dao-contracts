use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::U512;

use crate::escrow::types::{BidId, JobOfferId};

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub enum BidAuctionTime {
    InternalAuction,
    PublicAuction,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, PartialEq, Clone)]
pub enum BidStatus {
    Created,
    Selected,
    Rejected,
    Reclaimed,
    Canceled,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
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

    pub fn reclaim(
        &mut self,
        new_bid_id: BidId,
        new_worker: Address,
        block_time: BlockTime,
        reputation_stake: U512,
        cspr_stake: Option<U512>,
        onboard: bool,
    ) -> Bid {
        let mut new_bid = self.clone();
        self.status = BidStatus::Reclaimed;

        new_bid.bid_id = new_bid_id;
        new_bid.status = BidStatus::Selected;
        new_bid.worker = new_worker;
        new_bid.timestamp = block_time;
        new_bid.reputation_stake = reputation_stake;
        new_bid.proposed_timeframe =
            self.timestamp + self.proposed_timeframe + self.proposed_timeframe;
        new_bid.cspr_stake = cspr_stake;
        new_bid.onboard = onboard;

        new_bid
    }

    pub fn pick(&mut self) {
        self.status = BidStatus::Selected;
    }

    pub fn reject(&mut self) {
        self.status = BidStatus::Rejected;
    }

    pub fn cancel(&mut self) {
        self.status = BidStatus::Canceled;
    }

    pub fn bid_id(&self) -> BidId {
        self.bid_id
    }
}
