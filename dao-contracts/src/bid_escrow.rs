use std::ops::Add;

use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert, emit},
    Address, Mapping,  Variable, Error, casper_contract::unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{RuntimeArgs, runtime_args};

use crate::{
    voting::{GovernanceVoting, ReputationAmount, kyc_info::KycInfo, onboarding_info::OnboardingInfo},
    bid::{job::{Job, JobStatus}, types::{BidId, Description}, events::JobCreated},
};

use delegate::delegate;

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, kyc_token: Address, va_token: Address);
    fn pick_bid(
        &mut self,
        worker: Address,
        description: Description,
        required_stake: Option<ReputationAmount>
    );
    fn accept_bid(&mut self, bid_id: BidId);
    fn cancel_bid(&mut self, bid_id: BidId);
    fn submit_result(&mut self, bid_id: BidId, result: Description);
    fn get_job(&self, bid_id: BidId) -> Option<Job>;
}

#[derive(Instance)]
pub struct BidEscrowContract {
    voting: GovernanceVoting,
    kyc: KycInfo,
    onboarding: OnboardingInfo,
    jobs: Mapping<BidId, Job>,
    jobs_count: Variable<BidId>,
}

impl BidEscrowContractInterface for BidEscrowContract {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, kyc_token: Address, va_token: Address) {
        self.voting.init(variable_repo, reputation_token);
        self.kyc.init(kyc_token);
        self.onboarding.init(va_token);
    }

    fn pick_bid(&mut self, worker: Address, description: Description, required_stake: Option<ReputationAmount>) {
        if worker == caller() {
            revert(Error::CannotPostJobForSelf);
        }

        if !self.kyc.is_kycd(&caller()) {
            revert(Error::JobPosterNotKycd);
        }

        if !self.kyc.is_kycd(&worker) {
            revert(Error::WorkerNotKycd);
        }

        let bid_id = self.next_bid_id();

        let mut job = Job::new(
            bid_id, description.clone(), caller(), worker, required_stake
        );

        // TODO Automatically accept for non VAs
        if !self.onboarding.is_onboarded(&worker) {
            job.accept();
        }

        self.jobs.set(&bid_id, job);

        emit(JobCreated {
            bid_id,
            job_poster: caller(),
            worker,
            description,
            required_stake,
        });
    }


    fn accept_bid(&mut self,bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if job.status() == JobStatus::Accepted {
            revert(Error::InvalidContext);
        }

        if job.worker() == caller() {
            job.accept();
            self.jobs.set(&bid_id, job);
        } else {
            revert(Error::InvalidContext);
        }
    }

    fn cancel_bid(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if job.poster() != caller() {
            revert(Error::InvalidContext);
        }

        job.cancel()
    }

    fn submit_result(&mut self, bid_id: BidId, result: Description) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        job.set_result(result);
        self.jobs.set(&bid_id, job);
    }

    fn get_job(&self, bid_id: BidId) -> Option<Job> {
        self.jobs.get_or_none(&bid_id)
    }
}

impl BidEscrowContract {
    fn next_bid_id(&mut self) -> BidId {
        let bid_id = self.jobs_count.get();
        self.jobs_count.set(bid_id + 1);
        bid_id
    }
}