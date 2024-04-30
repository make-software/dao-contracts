//! Bid struct and structs that the Bid uses.
use crate::bid_escrow::job::{JobStatus, PickBidRequest};
use crate::bid_escrow::job_offer::{AuctionState, JobOfferStatus};
use crate::bid_escrow::types::{BidId, JobOfferId};
use crate::rules::validation::bid_escrow::{
    CanBeOnboarded, CanBidBeCancelled, CanBidOnAuctionState, CanBidOnOwnJob, CanPickBid,
    DoesProposedPaymentExceedBudget, HasPermissionsToCancelBid, IsBidStakeCorrect, IsGracePeriod,
    IsStakeNonZero,
};
use crate::rules::validation::IsUserKyced;
use crate::rules::RulesBuilder;
use odra::types::{Address, Balance, BlockTime};
use odra::OdraType;

/// Bid status representation
#[derive(OdraType, PartialEq, Debug)]
pub enum BidStatus {
    /// Placed, awaiting to be picked.
    Created,
    /// Chosen by the `Job Poster`.
    Picked,
    /// The Bid was not chosen by the `Job Poster`.
    Rejected,
    /// The bid was taken over during [Grace Period](crate::bid_escrow#grace-period).
    Reclaimed,
    /// Cancel due eg. the `Job Poster` was slashed.
    Canceled,
}

/// Data required to create a Bid.
pub struct SubmitBidRequest {
    /// New bid id.
    pub bid_id: BidId,
    /// Submission creation time.
    pub timestamp: BlockTime,
    /// Id of the offer the new bid refers to.
    pub job_offer_id: JobOfferId,
    /// Time to complete the Job.
    pub proposed_timeframe: BlockTime,
    /// Proposed payment for completing the Job.
    pub proposed_payment: Balance,
    /// Bid reputation stake.
    pub reputation_stake: Balance,
    /// Bid CSPR stake - for an [External Worker](crate::bid_escrow#definitions).
    pub cspr_stake: Option<Balance>,
    /// Should be onborded when the Job is done.
    pub onboard: bool,
    /// [Worker](crate::bid_escrow#definitions) address.
    pub worker: Address,
    /// If the `Worker` passed the KYC process.
    pub worker_kyced: bool,
    /// If the `Worker` is a [VA](crate::bid_escrow#definitions).
    pub worker_is_va: bool,
    /// [JobPoster](crate::bid_escrow#definitions) address.
    pub job_poster: Address,
    /// The Job max budget
    pub max_budget: Balance,
    /// Auction state
    pub auction_state: AuctionState,
    /// Is allowed a VA bid on public auction (not an External Worker only).
    pub va_can_bid_on_public_auction: bool,
}

/// Data required to cancel a bid.
pub struct CancelBidRequest {
    /// Address who tries to cancel the Bid.
    pub caller: Address,
    /// The current JobOffer status
    pub job_offer_status: JobOfferStatus,
    /// Block time
    pub block_time: BlockTime,
    /// Indicates if is too late to cancel the Bid.
    pub va_bid_acceptance_timeout: BlockTime,
}

/// Data required to reclaim a bid during [Grace Period](crate::bid_escrow::contract#grace-period).
pub struct ReclaimBidRequest {
    /// New bid id.
    pub new_bid_id: BidId,
    /// The [`Address`] that reclaims the Bid.
    pub caller: Address,
    /// Bid CSPR stake - for an [External Worker](crate::bid_escrow#definitions).
    pub cspr_stake: Option<Balance>,
    /// Bid reputation stake.
    pub reputation_stake: Balance,
    /// New [Worker](crate::bid_escrow#definitions) address.
    pub new_worker: Address,
    /// If the `Worker` is a [VA](crate::bid_escrow#definitions).
    pub new_worker_va: bool,
    /// If the `Worker` passed the KYC process.
    pub new_worker_kyced: bool,
    /// The related Job creator.
    pub job_poster: Address,
    /// Should be onborded when the Job is done.
    pub onboard: bool,
    /// Reclaim creation time.
    pub block_time: BlockTime,
    /// The current status of reclaimed Bid.
    pub job_status: JobStatus,
    /// The related `Job` finish time.
    pub job_finish_time: BlockTime,
}

/// Serializable representation of a `Bid`.
#[derive(OdraType)]
pub struct Bid {
    /// Bid id.
    pub bid_id: BidId,
    /// Bid Status
    pub status: BidStatus,
    /// Creation time.
    pub timestamp: BlockTime,
    /// Related JobOffer id
    pub job_offer_id: JobOfferId,
    /// Proposed Job completion time.
    pub proposed_timeframe: BlockTime,
    /// Proposed payment for the Job.
    pub proposed_payment: Balance,
    /// Bid reputation stake.
    pub reputation_stake: Balance,
    /// Bid CSPR stake - for an [External Worker](crate::bid_escrow::contract#definitions).
    pub cspr_stake: Option<Balance>,
    /// Should be onborded when the Job is done.
    pub onboard: bool,
    /// [Worker](crate::bid_escrow#definitions) address.
    pub worker: Address,
}

impl Bid {
    /// Conditionally creates a new instance of Bid.
    ///
    /// Runs validation:
    /// * [`IsUserKyced`]
    /// * [`CanBidOnOwnJob`]
    /// * [`CanBeOnboarded`]
    /// * [`DoesProposedPaymentExceedBudget`]
    /// * [`CanBidOnAuctionState`]
    ///
    /// Stops contract execution if any validation fails.
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
            .add_validation(IsBidStakeCorrect::create(
                request.worker_is_va,
                request.cspr_stake,
                request.reputation_stake,
            ))
            .build()
            .validate_generic_validations();

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

    /// Conditionally changes the status to [Reclaimed](BidStatus::Reclaimed), creates a new bid
    /// with a new proposed timeframe.
    ///
    /// Runs validation:
    /// * [`IsUserKyced`]
    /// * [`CanBidOnOwnJob`]
    /// * [`CanBeOnboarded`]
    /// * [`IsStakeNonZero`]
    /// * [`IsGracePeriod`]
    ///
    /// Stops contract execution if any validation fails.
    pub fn reclaim(&mut self, request: &ReclaimBidRequest) -> Bid {
        RulesBuilder::new()
            .add_validation(IsUserKyced::create(request.new_worker_kyced))
            .add_validation(CanBidOnOwnJob::create(
                request.new_worker,
                request.job_poster,
            ))
            .add_validation(CanBeOnboarded::create(
                request.new_worker_va,
                request.onboard,
            ))
            .add_validation(IsStakeNonZero::create(
                request.reputation_stake,
                request.cspr_stake,
            ))
            .add_validation(IsGracePeriod::create(
                request.job_status,
                request.job_finish_time,
                request.block_time,
            ))
            .build()
            .validate_generic_validations();

        let mut new_bid = self.clone();
        self.status = BidStatus::Reclaimed;

        new_bid.bid_id = request.new_bid_id;
        new_bid.status = BidStatus::Picked;
        new_bid.worker = request.new_worker;
        new_bid.timestamp = request.block_time;
        new_bid.reputation_stake = request.reputation_stake;
        new_bid.proposed_timeframe =
            self.timestamp + self.proposed_timeframe + self.proposed_timeframe;
        new_bid.cspr_stake = request.cspr_stake;
        new_bid.onboard = request.onboard;

        new_bid
    }

    /// Conditionally changes the status to [Picked](BidStatus::Picked).
    ///
    /// Runs validation:
    /// * [`CanPickBid`]
    ///
    /// Stops contract execution if the validation fails.
    pub fn picked(&mut self, request: &PickBidRequest) {
        RulesBuilder::new()
            .add_validation(CanPickBid::create(request.caller, request.poster))
            .build()
            .validate_generic_validations();

        self.status = BidStatus::Picked;
    }

    /// Unconditionally changes the status to [Rejected](BidStatus::Rejected).
    pub fn reject_without_validation(&mut self) {
        self.status = BidStatus::Rejected;
    }

    /// Conditionally changes the status to [Canceled](BidStatus::Canceled).
    ///
    /// Runs validation:
    /// * [`HasPermissionsToCancelBid`]
    /// * [`CanBidBeCancelled`]
    ///
    /// Stops contract execution if any validation fails.
    pub fn cancel(&mut self, request: CancelBidRequest) {
        RulesBuilder::new()
            .add_validation(HasPermissionsToCancelBid::create(
                request.caller,
                self.worker,
            ))
            .add_validation(CanBidBeCancelled::create(
                request.job_offer_status,
                request.block_time,
                self.timestamp,
                request.va_bid_acceptance_timeout,
            ))
            .build()
            .validate_generic_validations();
        self.status = BidStatus::Canceled;
    }

    /// Unconditionally changes the status to [Canceled](BidStatus::Canceled).
    pub fn cancel_without_validation(&mut self) {
        self.status = BidStatus::Canceled;
    }

    /// Gets the bid id.
    pub fn bid_id(&self) -> BidId {
        self.bid_id
    }
}
