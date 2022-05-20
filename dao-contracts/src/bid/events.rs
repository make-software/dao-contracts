use crate::bid::job::Job;
use casper_dao_utils::{casper_dao_macros::Event, Address, BlockTime};
use casper_types::U512;

use crate::voting::ReputationAmount;

use super::types::{BidId, Description};

#[derive(Debug, PartialEq, Event)]
pub struct JobCreated {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub description: Description,
    pub finish_time: BlockTime,
    pub required_stake: Option<ReputationAmount>,
    pub cspr_amount: U512,
}

impl JobCreated {
    pub fn new(job: &Job) -> JobCreated {
        JobCreated {
            bid_id: job.bid_id(),
            job_poster: job.poster(),
            worker: job.worker(),
            description: job.description().clone(),
            finish_time: job.finish_time(),
            required_stake: job.required_stake(),
            cspr_amount: job.cspr_amount(),
        }
    }
}

#[derive(Debug, PartialEq, Event)]
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

#[derive(Debug, PartialEq, Event)]
pub struct JobSubmitted {
    pub bid_id: BidId,
    pub job_poster: Address,
    pub worker: Address,
    pub result: Description,
}

impl JobSubmitted {
    pub fn new(job: &Job) -> JobSubmitted {
        let result = match job.result() {
            None => Description::default(),
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

#[derive(Debug, PartialEq, Event)]
pub struct JobCancelled {
    pub bid_id: BidId,
    pub caller: Address,
    pub job_poster: Address,
    pub worker: Address,
    pub reason: Description,
    pub cspr_amount: U512,
}

impl JobCancelled {
    pub fn new(job: &Job, caller: Address, reason: Description) -> JobCancelled {
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

#[derive(Debug, PartialEq, Event)]
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

#[derive(Debug, PartialEq, Event)]
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
