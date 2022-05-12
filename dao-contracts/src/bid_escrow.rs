use casper_dao_utils::{
    casper_contract::{
        contract_api::system::{
            get_purse_balance, transfer_from_purse_to_account, transfer_from_purse_to_purse,
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, emit, get_block_time, revert},
    Address, BlockTime, Error, Mapping, Variable,
};
use casper_types::{RuntimeArgs, URef, U256, U512};

use crate::{
    bid::{
        events::{JobAccepted, JobCreated, JobSubmitted},
        job::Job,
        types::{BidId, Description},
    },
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting::{Voting, VotingResult},
        Ballot, Choice, GovernanceVoting, ReputationAmount, VotingId,
    },
};

use delegate::delegate;

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );
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
    fn vote(&mut self, bid_id: BidId, voting_id: VotingId, choice: Choice, stake: U256);
    fn get_job(&self, bid_id: BidId) -> Option<Job>;
    fn finish_voting(&mut self, bid_id: BidId, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
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
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.voting.init(variable_repo, reputation_token);
        self.kyc.init(kyc_token);
        self.onboarding.init(va_token);
    }

    fn pick_bid(
        &mut self,
        worker: Address,
        description: Description,
        time: BlockTime,
        required_stake: Option<ReputationAmount>,
        purse: URef,
    ) {
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
            bid_id,
            description.clone(),
            caller(),
            worker,
            finish_time,
            required_stake,
            cspr_amount,
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

    fn accept_job(&mut self, bid_id: BidId) {
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
        self.voting.create_escrow_voting(job.poster());
        self.jobs.set(&bid_id, job);
    }

    fn vote(&mut self, bid_id: BidId, voting_id: VotingId, choice: Choice, stake: U256) {
        let job = self.jobs.get_or_revert(&bid_id);
        if caller() == job.poster() || caller() == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_job(&self, bid_id: BidId) -> Option<Job> {
        self.jobs.get_or_none(&bid_id)
    }

    fn finish_voting(&mut self, bid_id: BidId, voting_id: VotingId) {
        let voting_summary = self.voting.finish_voting(voting_id);
        if voting_summary.is_formal() {
            match voting_summary.result() {
                VotingResult::InFavor => self.job_completed(bid_id),
                VotingResult::Against => self.job_not_completed(bid_id),
                VotingResult::QuorumNotReached => self.job_not_completed(bid_id),
            }
        } else if voting_summary.result() != VotingResult::InFavor {
            self.job_not_completed(bid_id);
        }
    }

    delegate! {
        to self.voting {
            fn get_dust_amount(&self) -> U256;
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_voting(&self, voting_id: U256) -> Option<Voting>;
            fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
            fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
        }
    }
}

impl BidEscrowContract {
    fn next_bid_id(&mut self) -> BidId {
        let bid_id = self.jobs_count.get().unwrap_or_default();
        self.jobs_count.set(bid_id + 1);
        bid_id
    }

    fn deposit(&mut self, cargo_purse: URef) -> U512 {
        let main_purse = casper_env::contract_main_purse();
        let amount = get_purse_balance(cargo_purse).unwrap_or_revert();
        transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None).unwrap_or_revert();
        amount
    }

    fn withdraw(&mut self, address: Address, amount: U512) {
        let main_purse = casper_env::contract_main_purse();
        transfer_from_purse_to_account(
            main_purse,
            *address
                .as_account_hash()
                .unwrap_or_revert_with(Error::InvalidAddress),
            amount,
            None,
        )
        .unwrap_or_revert_with(Error::TransferError);
    }

    fn job_completed(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        self.withdraw(job.worker(), job.cspr_amount());
        job.complete();
        self.jobs.set(&bid_id, job);
    }

    fn job_not_completed(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        self.withdraw(job.poster(), job.cspr_amount());
        job.mark_as_not_completed();
        self.jobs.set(&bid_id, job);
    }
}

#[cfg(feature = "test-support")]
impl BidEscrowContractTest {
    pub fn pick_bid_with_cspr_amount(
        &mut self,
        worker: Address,
        description: Description,
        time: BlockTime,
        required_stake: Option<ReputationAmount>,
        cspr_amount: U512,
    ) {
        use casper_types::runtime_args;
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
