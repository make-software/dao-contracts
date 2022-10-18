use crate::bid::job::Job;
use casper_dao_utils::{casper_dao_macros::Event, Address, BlockTime, DocumentHash};
use casper_types::{U256, U512};

use super::types::BidId;

#[derive(Debug, PartialEq, Eq, Event)]
pub struct JobCreated {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub document_hash: DocumentHash,
    pub finish_time: BlockTime,
    pub required_stake: Option<U256>,
    pub cspr_amount: U512,
}

impl JobCreated {
    pub fn new(job: &Job) -> JobCreated {
        JobCreated {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            document_hash: job.document_hash().clone(),
            finish_time: job.finish_time(),
            required_stake: job.required_stake(),
            cspr_amount: job.cspr_amount(),
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
            cspr_amount: job.cspr_amount(),
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
            cspr_amount: job.cspr_amount(),
        }
    }
}
