use crate::bid::types::JobOfferId;
use casper_dao_utils::casper_dao_macros::{CLTyped, FromBytes, ToBytes};
use casper_dao_utils::{Address, BlockTime};
use casper_types::bytesrepr::ToBytes;
use casper_types::U512;

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub enum JobOfferStatus {
    Created,
    Selected,
    Cancelled,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct JobOffer {
    pub job_offer_id: JobOfferId,
    pub job_poster: Address,
    pub max_budget: U512,
    pub expected_timeframe: BlockTime,
    pub status: JobOfferStatus,
}

impl JobOffer {
    pub fn new(
        offer_id: JobOfferId,
        job_poster: Address,
        expected_timeframe: BlockTime,
        max_budget: U512,
    ) -> Self {
        JobOffer {
            job_offer_id: offer_id,
            job_poster,
            max_budget,
            expected_timeframe,
            status: JobOfferStatus::Created,
        }
    }
}
