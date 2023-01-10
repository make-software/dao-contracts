use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::U512;

use crate::{
    escrow::{
        job_offer::AuctionState,
        types::{BidId, JobOfferId},
        validation::rules::{
            can_be_onboarded::CanBeOnboarded,
            can_bid_on_auction_state::CanBidOnAuctionState,
            can_bid_on_own_job::CanBidOnOwnJob,
            does_proposed_payment_exceed_budget::DoesProposedPaymentExceedBudget,
        },
    },
    rules::{builder::RulesBuilder, validation::is_user_kyced::IsUserKyced},
};

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

pub struct SubmitBidRequest {
    pub bid_id: BidId,
    pub timestamp: BlockTime,
    pub job_offer_id: JobOfferId,
    pub proposed_timeframe: BlockTime,
    pub proposed_payment: U512,
    pub reputation_stake: U512,
    pub cspr_stake: Option<U512>,
    pub onboard: bool,
    pub worker: Address,
    pub worker_kyced: bool,
    pub worker_is_va: bool,
    pub job_poster: Address,
    pub max_budget: U512,
    pub auction_state: AuctionState,
    pub va_can_bid_on_public_auction: bool,
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
    pub fn new(request: SubmitBidRequest) -> Bid {
        RulesBuilder::new()
            .add_validation(IsUserKyced::create(request.worker_kyced))
            .add_validation(CanBidOnOwnJob::create(request.worker, request.job_poster))
            .add_validation(CanBeOnboarded::create(
                request.worker_is_va,
                request.onboard,
            ))
            .add_validation(DoesProposedPaymentExceedBudget::create(
                request.proposed_payment,
                request.max_budget,
            ))
            .add_validation(CanBidOnAuctionState::create(
                request.auction_state,
                request.worker_is_va,
                request.va_can_bid_on_public_auction,
            ))
            .validate();

        Bid {
            bid_id: request.bid_id,
            status: BidStatus::Created,
            timestamp: request.timestamp,
            job_offer_id: request.job_offer_id,
            proposed_timeframe: request.proposed_timeframe,
            proposed_payment: request.proposed_payment,
            reputation_stake: request.reputation_stake,
            cspr_stake: request.cspr_stake,
            onboard: request.onboard,
            worker: request.worker,
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

/// ShortenedBid struct
///
/// Derives from the [`Bid`] struct. 
/// Contains only the essential fields from the original [`Bid`] required in cross-contract communication.
#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct ShortenedBid {
    pub bid_id: BidId, 
    pub reputation_stake: U512,
    pub worker: Address, 
}

impl ShortenedBid {
    pub fn new(bid_id: BidId, reputation_stake: U512, worker: Address) -> Self {
        Self { bid_id, reputation_stake, worker }
    }
}

impl From<&Bid> for ShortenedBid {
    fn from(value: &Bid) -> Self {
        Self { bid_id: value.bid_id, reputation_stake: value.reputation_stake, worker: value.worker }
    }
} 
