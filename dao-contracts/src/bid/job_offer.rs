use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
    Error::{
        InternalAuctionTimeExpired,
        PaymentExceedsMaxBudget,
        PublicAuctionNotStarted,
        PublicAuctionTimeExpired,
    },
};
use casper_types::{U512};

use crate::{bid::types::JobOfferId, DaoConfiguration, DaoConfigurationTrait};

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub enum JobOfferStatus {
    Created,
    Selected,
    Cancelled,
}

pub enum AuctionState {
    None,
    Internal,
    External,
}

pub struct JobOfferConfiguration {}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct JobOffer {
    pub job_offer_id: JobOfferId,
    pub job_poster: Address,
    pub max_budget: U512,
    pub expected_timeframe: BlockTime,
    pub dos_fee: U512,
    pub status: JobOfferStatus,
    pub start_time: BlockTime,
    pub dao_configuration: DaoConfiguration,
}

impl JobOffer {
    pub fn new(
        offer_id: JobOfferId,
        job_poster: Address,
        expected_timeframe: BlockTime,
        max_budget: U512,
        dos_fee: U512,
        block_time: BlockTime,
        dao_configuration: DaoConfiguration,
    ) -> Self {
        JobOffer {
            job_offer_id: offer_id,
            job_poster,
            max_budget,
            expected_timeframe,
            dos_fee,
            status: JobOfferStatus::Created,
            start_time: block_time,
            dao_configuration,
        }
    }

    pub fn auction_state(&self) -> AuctionState {
        todo!()
    }

    pub fn validate_bid(
        &self,
        block_time: BlockTime,
        worker_onboarded: bool,
        proposed_payment: U512,
    ) -> Result<(), casper_dao_utils::Error> {
        // Payment
        if proposed_payment > self.max_budget {
            return Err(PaymentExceedsMaxBudget);
        }

        // InternalAuction time
        if worker_onboarded
            && block_time > self.start_time + self.dao_configuration.internal_auction_time()
        {
            return Err(InternalAuctionTimeExpired);
        }

        if !worker_onboarded {
            if block_time
                > self.start_time
                    + self.dao_configuration.public_auction_time()
                    + self.dao_configuration.internal_auction_time()
            {
                return Err(PublicAuctionTimeExpired);
            }

            if block_time < self.start_time + self.dao_configuration.internal_auction_time() {
                return Err(PublicAuctionNotStarted);
            }
        }

        // PublicAuction time
        if (!worker_onboarded && !self.dao_configuration.va_can_bid_on_public_auction())
            && block_time < self.start_time + self.dao_configuration.internal_auction_time()
            && block_time
                > self.start_time
                    + self.dao_configuration.internal_auction_time()
                    + self.dao_configuration.public_auction_time()
        {
            return Err(PublicAuctionTimeExpired);
        }

        Ok(())
    }
}
