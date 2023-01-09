use std::rc::Rc;

use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::U512;

use crate::{
    escrow::{
        job::PickBidRequest,
        types::JobOfferId,
        validation::rules::{
            can_job_offer_be_cancelled::CanJobOfferBeCancelled,
            can_progress_job_offer::CanProgressJobOffer,
            has_permissions_to_cancel_job_offer::HasPermissionsToCancelJobOffer,
            is_dos_fee_enough::IsDosFeeEnough,
        },
    },
    rules::{builder::RulesBuilder, validation::is_user_kyced::IsUserKyced},
    Configuration,
};

#[derive(CLTyped, ToBytes, FromBytes, Debug, PartialEq)]
pub enum JobOfferStatus {
    Created,
    InProgress,
    Cancelled,
}

#[derive(PartialEq)]
pub enum AuctionState {
    None,
    Internal,
    Public,
}

pub struct PostJobOfferRequest {
    pub job_offer_id: JobOfferId,
    pub job_poster: Address,
    pub job_poster_kyced: bool,
    pub max_budget: U512,
    pub expected_timeframe: BlockTime,
    pub dos_fee: U512,
    pub start_time: BlockTime,
    pub configuration: Rc<Configuration>,
}

pub struct CancelJobOfferRequest {
    pub caller: Address,
    pub block_time: BlockTime,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct JobOffer {
    pub job_offer_id: JobOfferId,
    pub job_poster: Address,
    pub max_budget: U512,
    pub expected_timeframe: BlockTime,
    pub dos_fee: U512,
    pub status: JobOfferStatus,
    pub start_time: BlockTime,
    pub configuration: Configuration,
}

impl JobOffer {
    pub fn new(request: PostJobOfferRequest) -> JobOffer {
        RulesBuilder::new()
            .add_validation(IsUserKyced::create(request.job_poster_kyced))
            .add_validation(IsDosFeeEnough::create(
                request.configuration.clone(),
                request.dos_fee,
            ))
            .validate();

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

    pub fn in_progress(&mut self, request: &PickBidRequest) {
        RulesBuilder::new()
            .add_validation(CanProgressJobOffer::create(request.caller, self.job_poster))
            .validate();

        self.status = JobOfferStatus::InProgress;
    }

    pub fn cancel(&mut self, request: &CancelJobOfferRequest) {
        RulesBuilder::new()
            .add_validation(HasPermissionsToCancelJobOffer::create(
                request.caller,
                self.job_poster,
            ))
            .add_validation(CanJobOfferBeCancelled::create(
                self.auction_state(request.block_time),
            ))
            .validate();

        self.status = JobOfferStatus::Cancelled;
    }

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

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
