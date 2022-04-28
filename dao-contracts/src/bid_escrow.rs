use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert},
    Address, Mapping,  Variable, Error, casper_contract::unwrap_or_revert::UnwrapOrRevert,
};

use crate::{
    voting::{GovernanceVoting, ReputationAmount},
    bid::{job::{Job, JobStatus}, types::{BidId, Description}},
};

use delegate::delegate;

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address);
    fn pick_bid(
        &mut self,
        worker: Address,
        description: Description,
        required_stake_for_va: Option<ReputationAmount>
    );
    fn accept_bid(&mut self, bid_id: BidId);
    fn cancel_bid(&mut self, bid_id: BidId);
    fn submit_result(&mut self, bid_id: BidId, result: Description);
}

#[derive(Instance)]
pub struct BidEscrowContract {
    voting: GovernanceVoting,
    jobs: Mapping<BidId, Job>,
    jobs_count: Variable<BidId>,
}

impl BidEscrowContractInterface for BidEscrowContract {
    fn pick_bid(&mut self, worker: Address, description: Description, required_stake_for_va: Option<ReputationAmount>) {
        if worker == caller() {
            revert(Error::CannotPostJobForSelf)
        }

        let bid_id = self.next_bid_id();

        let job = Job::new(
            bid_id, description, caller(), worker, required_stake_for_va
        );

        self.jobs.set(&bid_id, job);
    }


    fn accept_bid(&mut self,bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if *job.status() == JobStatus::Accepted {
            revert(Error::InvalidContext);
        }

        if *job.worker() == Some(caller()) {
            job.accept();
            self.jobs.set(&bid_id, job);
        } else {
            revert(Error::InvalidContext);
        }
    }

    fn cancel_bid(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if job.poster().unwrap_or_revert() != caller() {
            revert(Error::InvalidContext);
        }

        job.cancel()
    }

    fn submit_result(&mut self, bid_id: BidId, result: Description) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        job.set_result(result);
        self.jobs.set(&bid_id, job);
    }

    delegate! {
        to self.voting {
            fn init(&mut self, variable_repo: Address, reputation_token: Address);
        }
    }

}

impl BidEscrowContract {
    fn next_bid_id(&mut self) -> BidId {
        let bid_id = self.jobs_count.get();
        self.jobs_count.set(bid_id + 1);
        bid_id
    }

    fn is_kycd(address: Address) -> bool {
        true
    }
}