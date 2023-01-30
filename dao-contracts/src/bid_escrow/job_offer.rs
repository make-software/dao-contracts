//! JobOffer-related structs.
use std::rc::Rc;

use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::U512;

use crate::{
    bid_escrow::{job::PickBidRequest, types::JobOfferId},
    config::Configuration,
    rules::{
        validation::{
            bid_escrow::{
                CanJobOfferBeCancelled,
                CanProgressJobOffer,
                HasPermissionsToCancelJobOffer,
                IsDosFeeEnough,
            },
            IsUserKyced,
        },
        RulesBuilder,
    },
};

/// Serializable JobOffer status representation.
#[derive(CLTyped, ToBytes, FromBytes, Debug, PartialEq)]
pub enum JobOfferStatus {
    /// Created, Bidders can place bids.
    Created,
    /// Bid selected, a Worker works on it.
    InProgress,
    /// Offer canceled, is no longer valid.
    Cancelled,
}

/// Auction state representation.
#[derive(PartialEq)]
pub enum AuctionState {
    /// Unknown state.
    None,
    /// Internal Auction - only VAs' can bid.
    Internal,
    /// Public Auction - nonVAs' can bid.
    Public,
}

/// Data required to post a job offer.
pub struct PostJobOfferRequest {
    /// New offer id.
    pub job_offer_id: JobOfferId,
    /// The offer creator.
    pub job_poster: Address,
    /// Is the creator passed the KYC process.
    pub job_poster_kyced: bool,
    /// Max amount the Job Poster can pay for the Job.
    pub max_budget: U512,
    /// The time the Job should be completed.
    pub expected_timeframe: BlockTime,
    /// CSPR amount attached to Post Job query.
    pub dos_fee: U512,
    /// The time since the offer is available for Bidders.
    pub start_time: BlockTime,
    /// Job configuration.
    pub configuration: Rc<Configuration>,
}

/// Data required to cancel a job offer.
pub struct CancelJobOfferRequest {
    /// The request caller.
    pub caller: Address,
    /// The request creation time.
    pub block_time: BlockTime,
}

/// Writeable/readable representation of a `Job Offer`.
#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct JobOffer {
    /// Offer id.
    pub job_offer_id: JobOfferId,
    /// The offer creator.
    pub job_poster: Address,
    /// Max amount the Job Poster can pay for the Job.
    pub max_budget: U512,
    /// The time the Job should be completed.
    pub expected_timeframe: BlockTime,
    /// CSPR amount attached to the offer.
    pub dos_fee: U512,
    /// The current job offer status.
    pub status: JobOfferStatus,
    /// The time since the offer is available for Bidders.
    pub start_time: BlockTime,
    /// Job configuration.
    pub configuration: Configuration,
}

impl JobOffer {
    /// Conditionally creates a new instance of JobOffer.
    ///
    /// Runs validation:
    /// * [`IsUserKyced`]
    /// * [`IsDosFeeEnough`]
    /// Stops contract execution if any validation fails.
    pub fn new(request: PostJobOfferRequest) -> JobOffer {
        RulesBuilder::new()
            .add_validation(IsUserKyced::create(request.job_poster_kyced))
            .add_validation(IsDosFeeEnough::create(
                request.configuration.clone(),
                request.dos_fee,
            ))
            .build()
            .validate_generic_validations();

        JobOffer {
            job_offer_id: request.job_offer_id,
            job_poster: request.job_poster,
            max_budget: request.max_budget,
            expected_timeframe: request.expected_timeframe,
            dos_fee: request.dos_fee,
            status: JobOfferStatus::Created,
            start_time: request.start_time,
            configuration: (*request.configuration).clone(),
        }
    }

    /// Conditionally changes the status to [InProgress](JobOfferStatus::InProgress).
    ///
    /// Runs validation:
    /// * [`CanProgressJobOffer`]
    ///
    /// Stops contract execution if the validation fails.
    pub fn in_progress(&mut self, request: &PickBidRequest) {
        RulesBuilder::new()
            .add_validation(CanProgressJobOffer::create(request.caller, self.job_poster))
            .build()
            .validate_generic_validations();

        self.status = JobOfferStatus::InProgress;
    }

    /// Conditionally changes the status to [Cancelled](JobOfferStatus::Cancelled).
    ///
    /// Runs validation:
    /// * [`HasPermissionsToCancelJobOffer`]
    /// * [`CanJobOfferBeCancelled`]
    ///
    /// Stops contract execution if any validation fails.
    pub fn cancel(&mut self, request: &CancelJobOfferRequest) {
        RulesBuilder::new()
            .add_validation(HasPermissionsToCancelJobOffer::create(
                request.caller,
                self.job_poster,
            ))
            .add_validation(CanJobOfferBeCancelled::create(
                self.auction_state(request.block_time),
            ))
            .build()
            .validate_generic_validations();

        self.status = JobOfferStatus::Cancelled;
    }

    /// Gets the auction state in a given time.
    pub fn auction_state(&self, block_time: BlockTime) -> AuctionState {
        let public_auction_start_time =
            self.start_time + self.configuration.internal_auction_time();
        let public_auction_end_time =
            public_auction_start_time + self.configuration.public_auction_time();
        if block_time >= self.start_time && block_time < public_auction_start_time {
            AuctionState::Internal
        } else if block_time >= public_auction_start_time && block_time < public_auction_end_time {
            AuctionState::Public
        } else {
            AuctionState::None
        }
    }

    /// Gets a reference to the job configuration.
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
