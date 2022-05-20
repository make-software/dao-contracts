use casper_dao_utils::{
    casper_contract::{
        contract_api::system::{
            get_purse_balance, transfer_from_purse_to_account, transfer_from_purse_to_purse,
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, get_block_time, revert},
    Address, BlockTime, Error, Mapping, Variable,
};
use casper_types::{URef, U256, U512};

use crate::{
    bid::{
        events::{JobAccepted, JobCreated, JobSubmitted},
        job::Job,
        types::{BidId, Description},
    },
    proxy::{
        reputation_proxy::ReputationContractProxy, variable_repo_proxy::VariableRepoContractProxy,
    },
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting::{Voting, VotingResult},
        Ballot, Choice, GovernanceVoting, ReputationAmount,
    },
    VotingConfigurationBuilder,
};

use delegate::delegate;

use crate::bid::events::JobCancelled;
#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    /// Initializes the module with [Addresses](Address) of [Reputation Token](crate::ReputationContract), [Variable Repo](crate::VariableRepositoryContract)
    /// KYC Token and VA Token
    ///
    /// # Events
    /// Emits [`VotingContractCreated`](crate::voting::governance_voting::events::VotingContractCreated)
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );
    /// Job poster picks a bid. This creates a new Job object and saves it in a storage.
    /// If worker is not onboarded, the job is accepted automatically.
    /// Otherwise, worker needs to accept job (see [accept_job](accept_job))
    ///
    /// # Events
    /// Emits [`JobCreated`](JobCreated)
    ///
    /// Emits [`JobAccepted`](JobAccepted)
    ///
    /// # Errors
    /// Throws [`CannotPostJobForSelf`](Error::CannotPostJobForSelf) when trying to create job for
    /// self
    ///
    /// Throws [`JobPosterNotKycd`](Error::JobPosterNotKycd) or [`Error::WorkerNotKycd`](Error::WorkerNotKycd)
    /// When either Job Poster or Worker has not completed the KYC process
    fn pick_bid(
        &mut self,
        worker: Address,
        description: Description,
        time: BlockTime,
        required_stake: Option<ReputationAmount>,
        purse: URef,
    );
    /// Worker accepts job. It can be done only for jobs with [`Created`](JobStatus::Created) status
    /// and if time for job acceptance has not yet passed.
    /// # Events
    /// Emits [`JobAccepted`](JobAccepted)
    ///
    /// # Errors
    /// Throws [`CannotAcceptJob`](Error::CannotAcceptJob) if one of the constraints for
    /// job acceptance is not met.
    fn accept_job(&mut self, bid_id: BidId);
    /// Cancel job. This can be done only by job creator if the job wasn't yet accepted.
    /// It refunds cspr to job creator.
    /// # Events
    /// Emits [`JobCancelled`](JobCancelled)
    ///
    /// # Errors
    /// Throws [`CannotCancelJob`](Error::CannotCancelJob) if one of the constraints for
    /// job cancellation is not met.
    fn cancel_job(&mut self, bid_id: BidId, reason: Description);
    /// Worker submits the result of the job. Job can also be submitted by a job poster after the
    /// time for work has passed.
    /// This starts a new voting over the result.
    /// # Events
    /// Emits [`JobSubmitted`](JobSubmitted)
    ///
    /// Emits [`VotingCreated`](crate::voting::governance_voting::events::VotingCreated)
    ///
    /// # Errors
    /// Throws [`JobAlreadySubmitted`](Error::JobAlreadySubmitted) if job was already submitted
    /// Throws [`NotAuthorizedToSubmitResult`](Error::NotAuthorizedToSubmitResult) if one of the constraints for
    /// job submission is not met.
    fn submit_result(&mut self, bid_id: BidId, result: Description);
    /// Casts a vote over a job
    /// # Events
    /// Emits [`BallotCast`](crate::voting::governance_voting::events::BallotCast)

    /// # Errors
    /// Throws [`CannotVoteOnOwnJob`](Error::CannotVoteOnOwnJob) if the voter is either of Job Poster or Worker
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn vote(&mut self, bid_id: BidId, choice: Choice, stake: U256);
    /// Returns a job with given BidId
    fn get_job(&self, bid_id: BidId) -> Option<Job>;
    /// Finishes voting stage. Depending on stage, the voting can be converted to a formal one, end
    /// with a refund or pay the worker.
    /// # Events
    /// Emits [`VotingEnded`](crate::voting::governance_voting::events::VotingEnded), [`VotingCreated`](crate::voting::governance_voting::events::VotingCreated)
    /// # Errors
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn finish_voting(&mut self, bid_id: BidId);
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_dust_amount(&self) -> U256;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_variable_repo_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_reputation_token_address(&self) -> Address;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_voting(&self, voting_id: U256) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_ballot(&self, voting_id: U256, address: Address) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_voter(&self, voting_id: U256, at: u32) -> Option<Address>;
    /// Returns the CSPR balance of the contract
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

        JobCreated {
            bid_id,
            job_poster: caller(),
            worker,
            description,
            finish_time,
            required_stake,
            cspr_amount,
        }
        .emit();

        if !self.onboarding.is_onboarded(&worker) {
            job.accept(job.worker(), get_block_time())
                .unwrap_or_revert();
            JobAccepted {
                bid_id,
                job_poster: job.poster(),
                worker: job.worker(),
            }
            .emit();
        }

        self.jobs.set(&bid_id, job);
    }

    fn accept_job(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        job.accept(caller(), get_block_time()).unwrap_or_revert();

        JobAccepted {
            bid_id,
            job_poster: job.poster(),
            worker: job.worker(),
        }
        .emit();

        self.jobs.set(&bid_id, job);
    }

    fn cancel_job(&mut self, bid_id: BidId, reason: Description) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        job.cancel(caller()).unwrap_or_revert();
        self.refund(&job);

        JobCancelled {
            bid_id,
            caller: caller(),
            job_poster: job.poster(),
            worker: job.worker(),
            reason,
        }
        .emit();

        self.jobs.set(&bid_id, job);
    }

    fn submit_result(&mut self, bid_id: BidId, result: Description) {
        let mut job = self.jobs.get_or_revert(&bid_id);

        job.submit(caller(), get_block_time(), &result)
            .unwrap_or_revert();

        JobSubmitted {
            bid_id,
            job_poster: job.poster(),
            worker: job.worker(),
            result,
        }
        .emit();

        let voting_configuration = VotingConfigurationBuilder::with_defaults(&self.voting)
            .with_cast_first_vote(false)
            .with_create_minimum_reputation(U256::zero())
            .build();

        let voting_id = self
            .voting
            .create_voting(job.poster(), U256::zero(), voting_configuration);

        job.set_informal_voting_id(Some(voting_id));
        self.jobs.set(&bid_id, job);
    }

    fn vote(&mut self, bid_id: BidId, choice: Choice, stake: U256) {
        let job = self.jobs.get_or_revert(&bid_id);
        let voting_id = job
            .current_voting_id()
            .unwrap_or_revert_with(Error::VotingNotStarted);
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
        let voting_id = job
            .current_voting_id()
            .unwrap_or_revert_with(Error::VotingNotStarted);
        let voting_summary = self.voting.finish_voting(voting_id);
        match voting_summary.voting_type() {
            crate::voting::voting::VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor => {
                    job.set_formal_voting_id(voting_summary.formal_voting_id());
                }
                VotingResult::Against => {
                    self.refund(&job);
                    job.not_completed();
                }
                VotingResult::QuorumNotReached => {
                    self.refund(&job);
                    job.not_completed();
                }
            },
            crate::voting::voting::VotingType::Formal => match voting_summary.result() {
                VotingResult::InFavor => {
                    self.pay_for_job(&job);
                    job.complete();
                }
                VotingResult::Against => {
                    self.refund(&job);
                    job.not_completed();
                }
                VotingResult::QuorumNotReached => {
                    self.refund(&job);
                    job.not_completed();
                }
            },
        }

        self.jobs.set(&bid_id, job);
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

    fn pay_for_job(&mut self, job: &Job) {
        self.withdraw(job.worker(), job.cspr_amount());
        self.mint_and_redistribute_reputation(job);
    }

    fn refund(&mut self, job: &Job) {
        self.withdraw(job.poster(), job.cspr_amount());
    }

    fn mint_and_redistribute_reputation(&mut self, job: &Job) {
        let reputation_to_mint = VariableRepoContractProxy::reputation_to_mint(
            self.voting.get_variable_repo_address(),
            job.cspr_amount(),
        );
        let reputation_to_redistribute = VariableRepoContractProxy::reputation_to_redistribute(
            self.voting.get_variable_repo_address(),
            reputation_to_mint,
        );

        // Worker
        ReputationContractProxy::mint(
            self.voting.get_reputation_token_address(),
            job.worker(),
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(job, reputation_to_redistribute);
    }

    fn mint_reputation_for_voters(&mut self, job: &Job, amount: U256) {
        let voting = self
            .voting
            .get_voting(job.formal_voting_id().unwrap_or_revert())
            .unwrap_or_revert();
        let result = voting.is_in_favor();
        for i in 0..self.voting.voters().len(voting.voting_id()) {
            let ballot = self.voting.get_ballot_at(voting.voting_id(), i);
            if ballot.choice.is_in_favor() == result {
                let to_transfer = ballot.stake * amount / voting.get_winning_stake();
                ReputationContractProxy::mint(
                    self.voting.get_reputation_token_address(),
                    ballot.voter,
                    to_transfer,
                );
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
