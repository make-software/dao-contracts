//! BidEscrow-related events.
use casper_dao_utils::{casper_dao_macros::Event, Address, BlockTime, DocumentHash};
use casper_types::U512;

use super::types::BidId;
use crate::bid_escrow::{job::Job, job_offer::JobOffer, types::JobOfferId};

/// Informs a new [Job Offer](crate::bid_escrow::job_offer::JobOffer) has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobOfferCreated {
    /// The offer id.
    job_offer_id: JobOfferId,
    /// The address of an account that created the offer.
    job_poster: Address,
    /// Max CSPR amount to be paid to the `Worker`.
    max_budget: U512,
    /// Offer validity time.
    expected_timeframe: BlockTime,
}

impl JobOfferCreated {
    /// Creates a new event.
    pub fn new(job_offer: &JobOffer) -> Self {
        JobOfferCreated {
            job_offer_id: job_offer.job_offer_id,
            job_poster: job_offer.job_poster,
            max_budget: job_offer.max_budget,
            expected_timeframe: job_offer.expected_timeframe,
        }
    }
}

/// Informs a new [Bid](crate::bid_escrow::bid::Bid) has been placed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct BidSubmitted {
    bid_id: BidId,
    job_offer_id: JobOfferId,
    worker: Address,
    onboard: bool,
    proposed_timeframe: BlockTime,
    proposed_payment: U512,
    reputation_stake: Option<U512>,
    cspr_stake: Option<U512>,
}

impl BidSubmitted {
    /// Creates a new event.
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

/// Informs a new [Job](crate::bid_escrow::job::Job) has been created.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCreated {
    bid_id: BidId,
    job_poster: Address,
    worker: Address,
    finish_time: BlockTime,
    payment: U512,
}

impl JobCreated {
    /// Creates a new event.
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

/// Informs a new [Job](crate::bid_escrow::job::Job) has been accepted by the `Job Poster`.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobAccepted {
    bid_id: BidId,
    job_poster: Address,
    worker: Address,
}

impl JobAccepted {
    /// Creates a new event.
    pub fn new(job: &Job) -> JobAccepted {
        JobAccepted {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
        }
    }
}

/// Informs the [Job](crate::bid_escrow::job::Job) proof has been submitted by the `Worker`.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobSubmitted {
    bid_id: BidId,
    job_poster: Address,
    worker: Address,
    result: DocumentHash,
}

impl JobSubmitted {
    /// Creates a new event.
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

/// Informs the [Job](crate::bid_escrow::job::Job) has been canceled.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCancelled {
    bid_id: BidId,
    caller: Address,
    job_poster: Address,
    worker: Address,
    reason: DocumentHash,
    cspr_amount: U512,
}

impl JobCancelled {
    /// Creates a new event.
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

/// Informs `Voting` on the [Job](crate::bid_escrow::job::Job) passed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobDone {
    bid_id: BidId,
    caller: Address,
    job_poster: Address,
    worker: Address,
    cspr_amount: U512,
}

impl JobDone {
    /// Creates a new event.
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

/// Informs `Voting` on the [Job](crate::bid_escrow::job::Job) failed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobRejected {
    bid_id: BidId,
    caller: Address,
    job_poster: Address,
    worker: Address,
    cspr_amount: U512,
}

impl JobRejected {
    /// Creates a new event.
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
