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
use casper_types::{URef, U256, U512};

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
        Ballot, Choice, GovernanceVoting, ReputationAmount,
    },
    VotingConfigurationBuilder, proxy::{variable_repo_proxy::VariableRepoContractProxy, reputation_proxy::ReputationContractProxy},
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
    fn vote(&mut self, bid_id: BidId, choice: Choice, stake: U256);
    fn get_job(&self, bid_id: BidId) -> Option<Job>;
    fn finish_voting(&mut self, bid_id: BidId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
    fn get_cspr_balance(&self) -> U512;
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

        let voting_configuration = VotingConfigurationBuilder::with_defaults(&self.voting)
            .with_cast_first_vote(false)
            .with_create_minimum_reputation(U256::zero())
            .build();

        let voting_id = self.voting
            .create_voting(job.poster(), U256::zero(), voting_configuration);

        job.set_informal_voting_id(Some(voting_id));
        self.jobs.set(&bid_id, job);
    }

    fn vote(&mut self, bid_id: BidId, choice: Choice, stake: U256) {
        let job = self.jobs.get_or_revert(&bid_id);
        let voting_id = job.current_voting_id().unwrap_or_revert_with(Error::VotingNotStarted);
        if caller() == job.poster() || caller() == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting.vote(caller(), voting_id, choice, stake);
    }

    fn get_job(&self, bid_id: BidId) -> Option<Job> {
        self.jobs.get_or_none(&bid_id)
    }

    fn finish_voting(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        let voting_id = job.current_voting_id().unwrap_or_revert_with(Error::VotingNotStarted);
        let voting_summary = self.voting.finish_voting(voting_id);
        match voting_summary.voting_type() {
            crate::voting::voting::VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor => {
                    job.set_formal_voting_id(voting_summary.formal_voting_id());
                    self.jobs.set(&bid_id, job);
                },
                VotingResult::Against => self.job_not_completed(bid_id),
                VotingResult::QuorumNotReached => self.job_not_completed(bid_id),
            },
            crate::voting::voting::VotingType::Formal => match voting_summary.result() {
                VotingResult::InFavor => self.job_completed(bid_id),
                VotingResult::Against => self.job_not_completed(bid_id),
                VotingResult::QuorumNotReached => self.job_not_completed(bid_id),
            },
        }
    }

    fn get_cspr_balance(&self) -> U512 {
        get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
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
        self.mint_and_redistribute_reputation(&job);
        self.jobs.set(&bid_id, job);
    }

    fn job_not_completed(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        self.withdraw(job.poster(), job.cspr_amount());
        job.mark_as_not_completed();
        self.jobs.set(&bid_id, job);
    }

    fn mint_and_redistribute_reputation(&mut self, job: &Job) {
        let reputation_to_mint = VariableRepoContractProxy::reputation_to_mint(self.voting.get_variable_repo_address(), job.cspr_amount());
        let reputation_to_redistribute = VariableRepoContractProxy::reputation_to_redistribute(self.voting.get_variable_repo_address(), reputation_to_mint);

        // Worker
        ReputationContractProxy::mint(self.voting.get_reputation_token_address(), job.worker(), reputation_to_mint - reputation_to_redistribute);

        // Voters
        self.mint_reputation_for_voters(job, reputation_to_redistribute);
    }

    fn mint_reputation_for_voters(&mut self, job: &Job, amount: U256) {
        let voting = self.voting.get_voting(job.formal_voting_id().unwrap_or_revert()).unwrap_or_revert();
        let result = voting.is_in_favor();
        for i in 0..self.voting.voters().len(voting.voting_id()) {
            let ballot = self.voting.get_ballot_at(voting.voting_id(), i);
            if ballot.choice.is_in_favor() == result {
                let to_transfer = ballot.stake * amount / voting.get_winning_stake();
                ReputationContractProxy::mint(self.voting.get_reputation_token_address(), ballot.voter, to_transfer);
            }
        }
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
        use casper_types::{runtime_args, RuntimeArgs};
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
