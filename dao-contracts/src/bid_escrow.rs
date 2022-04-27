use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance, CLTyped, ToBytes, FromBytes},
    casper_env::{caller, revert},
    Address, Mapping, VecMapping, Variable, Error,
};
use casper_types::{runtime_args, RuntimeArgs, U256};

use crate::{
    action::Action,
    voting::{voting::Voting, Ballot, Choice, GovernanceVoting, VotingId, ReputationAmount},
};

use delegate::delegate;

type BidId = u32;
type Description = String;

#[derive(CLTyped, ToBytes, FromBytes, Default)]
struct Job {
    bid_id: BidId,
    description: Description,
    result: Description,
    required_stake_for_va: ReputationAmount,
    job_poster: Option<Address>,
    worker: Option<Address>,
    accepted: bool,
}

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address);
    fn pick_bid(
        &mut self,
        worker: Address,
        description: Description,
        required_stake_for_va: ReputationAmount
    );
    fn accept_bid(&mut self, bid_id: BidId);
    fn cancel_bid_for_va_worker(&mut self, bid_id: BidId);
    fn submit_result(&mut self, bid_id: BidId, result: Description);
}

#[derive(Instance)]
pub struct BidEscrowContract {
    voting: GovernanceVoting,
    jobs: Mapping<BidId, Job>,
    jobs_count: Variable<BidId>,
}

impl BidEscrowContractInterface for BidEscrowContract {
    fn pick_bid(&mut self,worker:Address,description:Description,required_stake_for_va:ReputationAmount) {
        if worker == caller() {
            revert(Error::CannotPostJobForSelf)
        }

        let bid_id = self.next_bid_id();

        let job = Job {
            bid_id,
            description,
            result: Description::new(),
            required_stake_for_va,
            job_poster: Some(caller()),
            worker: Some(worker),
            accepted: !BidEscrowContract::is_kycd(worker),
        };

        self.jobs.set(&bid_id, job);
    }


    fn accept_bid(&mut self,bid_id:BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if job.accepted {
            revert(Error::InvalidContext);
        }

        if job.worker == Some(caller()) {
            job.accepted = true;
            self.jobs.set(&bid_id, job);
        } else {
            revert(Error::InvalidContext);
        }
    }


    fn cancel_bid_for_va_worker(&mut self,bid_id:BidId) {
        todo!()
    }


    fn submit_result(&mut self,bid_id:BidId,result:Description) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        job.result = result;
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