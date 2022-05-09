use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert, emit, get_block_time, self, self_address},
    Address, Mapping,  Variable, Error, BlockTime, casper_contract::{contract_api::system::{get_purse_balance, transfer_from_purse_to_purse}, unwrap_or_revert::UnwrapOrRevert},
};
use casper_types::{URef, U512, U256, RuntimeArgs};

use crate::{
    voting::{GovernanceVoting, ReputationAmount, kyc_info::KycInfo, onboarding_info::OnboardingInfo},
    bid::{job::Job, types::{BidId, Description}, events::{JobCreated, JobAccepted, JobSubmitted}},
};

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    fn init(&mut self, variable_repo: Address, reputation_token: Address, kyc_token: Address, va_token: Address);
    fn pick_bid(
        &mut self,
        worker: Address,
        description: Description,
        time: BlockTime,
        required_stake: Option<ReputationAmount>,
        purse: URef,
    );
    fn accept_job(&mut self, bid_id: BidId);
    fn cancel_job(&mut self, bid_id: BidId);
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

    fn pick_bid(&mut self, worker: Address, description: Description, time: BlockTime, required_stake: Option<ReputationAmount>, purse: URef) {
        let cspr_amount = self.deposit(purse);

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
        let finish_time = get_block_time() + time;

        let mut job = Job::new(
            bid_id, description.clone(), caller(), worker, finish_time, required_stake, cspr_amount
        );

        emit(JobCreated {
            bid_id,
            job_poster: caller(),
            worker,
            description,
            finish_time,
            required_stake,
            cspr_amount,
        });

        if !self.onboarding.is_onboarded(&worker) {
            job.accept();
            emit(JobAccepted {
                bid_id,
                job_poster: job.poster(),
                worker: job.worker(),
            });
        }

        self.jobs.set(&bid_id, job);
    }


    fn accept_job(&mut self,bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);

        if job.can_accept(caller(), get_block_time()) {
            job.accept();
            emit(JobAccepted {
                bid_id,
                job_poster: job.poster(),
                worker: job.worker(),
            });
            self.jobs.set(&bid_id, job);
        } else {
            revert(Error::CannotAcceptJob);
        }
    }

    fn cancel_job(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if !job.can_cancel(caller()) {
            revert(Error::CannotCancelJob);
        }

        job.cancel();
        self.jobs.set(&bid_id, job);
    }

    fn submit_result(&mut self, bid_id: BidId, result: Description) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if !job.can_submit(caller(), get_block_time()) {
            revert(Error::NotAuthorizedToSubmitResult)
        }
        
        emit(JobSubmitted {
            bid_id,
            job_poster: job.poster(),
            worker: job.worker(),
            result: result.clone(),
        });

        job.submit(result);
        self.jobs.set(&bid_id, job);

       self.voting.create_voting(caller(), U256::zero(), self_address(), "redistribute_cspr".to_string(), RuntimeArgs::new());
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

    fn deposit(&mut self, cargo_purse: URef) -> U512 {
        let main_purse = casper_env::contract_main_purse();
        let amount = get_purse_balance(cargo_purse).unwrap_or_revert();
        transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None).unwrap_or_revert();
        amount
    }
}

#[cfg(feature = "test-support")]
impl BidEscrowContractTest {
    pub fn pick_bid_with_cspr_amount(&mut self, worker: Address, description: Description, time: BlockTime, required_stake: Option<ReputationAmount>, cspr_amount: U512) {
        use casper_types::{runtime_args};
        self.env.deploy_wasm_file(
            "pick_bid.wasm",
            runtime_args! {
                "token_address" => self.address(),
                "cspr_amount" => cspr_amount,
                "worker" => worker,
                "description" => description,
                "time" => time,
                "required_stake" => required_stake,
            },
        );
    }
}