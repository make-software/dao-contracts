use crate::bid::job::Job;
use crate::bid::job_offer::JobOffer;
use crate::bid::types::JobOfferId;
use casper_dao_utils::{casper_dao_macros::Event, Address, BlockTime, DocumentHash};
use casper_types::U512;

use super::types::BidId;

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobOfferCreated {
    pub job_offer_id: JobOfferId,
    pub job_poster: Address,
    pub max_budget: U512,
    pub expected_timeframe: BlockTime,
}

impl JobOfferCreated {
    pub fn new(job_offer: &JobOffer) -> Self {
        JobOfferCreated {
            job_offer_id: job_offer.job_offer_id,
            job_poster: job_offer.job_poster,
            max_budget: job_offer.max_budget,
            expected_timeframe: job_offer.expected_timeframe,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct BidSubmitted {
    pub bid_id: BidId,
    pub job_offer_id: JobOfferId,
    pub worker: Address,
    pub onboard: bool,
    pub proposed_timeframe: BlockTime,
    pub proposed_payment: U512,
    pub reputation_stake: Option<U512>,
    pub cspr_stake: Option<U512>,
}

impl BidSubmitted {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bid_id: BidId,
        job_offer_id: JobOfferId,
        worker: Address,
        onboard: bool,
        proposed_timeframe: BlockTime,
        proposed_payment: U512,
        reputation_stake: Option<U512>,
        cspr_stake: Option<U512>,
    ) -> Self {
        BidSubmitted {
            bid_id,
            job_offer_id,
            worker,
            onboard,
            proposed_timeframe,
            proposed_payment,
            reputation_stake,
            cspr_stake,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCreated {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub finish_time: BlockTime,
    pub payment: U512,
}

impl JobCreated {
    pub fn new(job: &Job) -> JobCreated {
        JobCreated {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            finish_time: job.finish_time(),
            payment: job.payment(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobAccepted {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
}

impl JobAccepted {
    pub fn new(job: &Job) -> JobAccepted {
        JobAccepted {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobSubmitted {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub result: DocumentHash,
}

impl JobSubmitted {
    pub fn new(job: &Job) -> JobSubmitted {
        let result = match job.result() {
            None => DocumentHash::default(),
            Some(res) => res.clone(),
        };

        JobSubmitted {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            result,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCancelled {
    pub bid_id: BidId,
    pub caller: Address,
    pub job_poster: Address,
    pub worker: Address,
    pub reason: DocumentHash,
    pub cspr_amount: U512,
}

impl JobCancelled {
    pub fn new(job: &Job, caller: Address, reason: DocumentHash) -> JobCancelled {
        JobCancelled {
            bid_id: job.bid_id(),
            caller,
            job_poster: job.poster(),
            worker: job.worker(),
            reason,
            cspr_amount: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobDone {
    pub bid_id: BidId,
    pub caller: Address,
    pub job_poster: Address,
    pub worker: Address,
    pub cspr_amount: U512,
}

impl JobDone {
    pub fn new(job: &Job, caller: Address) -> JobDone {
        JobDone {
            bid_id: job.bid_id(),
            caller,
            job_poster: job.poster(),
            worker: job.worker(),
            cspr_amount: job.payment(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobRejected {
    pub bid_id: BidId,
    pub caller: Address,
    pub job_poster: Address,
    pub worker: Address,
    pub cspr_amount: U512,
}

impl JobRejected {
    pub fn new(job: &Job, caller: Address) -> JobRejected {
        JobRejected {
            bid_id: job.bid_id(),
            caller,
            job_poster: job.poster(),
            worker: job.worker(),
            cspr_amount: job.payment(),
        }
    }
}
