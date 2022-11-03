use casper_dao_utils::{
    casper_contract::{
        contract_api::system::{
            get_purse_balance, transfer_from_purse_to_account, transfer_from_purse_to_purse,
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, get_block_time, revert},
    Address, BlockTime, DocumentHash, Error, Mapping, SequenceGenerator,
};
use casper_types::{URef, U256, U512};

use crate::{
    bid::{
        events::{JobAccepted, JobCreated, JobSubmitted},
        job::Job,
        types::BidId,
    },
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting::{Voting, VotingResult},
        Ballot, Choice, GovernanceVoting,
    },
    ReputationContractCaller, ReputationContractInterface, VariableRepositoryContractCaller,
    VotingConfigurationBuilder,
};

use casper_dao_utils::casper_env::self_address;
use delegate::delegate;

use crate::bid::events::{JobCancelled, JobDone, JobOfferCreated, JobRejected};
use crate::voting::VotingId;

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

    /// Job Poster post a new Job Offer
    /// Parameters:
    /// expected_timeframe - Expected timeframe for completing a Job
    /// budget - Maximum budget for a Job
    /// Alongside Job Offer, Job Poster also sends DOS Fee in CSPR
    ///
    /// # Events
    /// Emits [`JobOfferCreated`](crate::bid::events::JobOfferCreated)
    fn post_job_offer(&mut self, expected_timeframe: BlockTime, budget: U512, purse: URef);

    /// Worker submits a Bid for a Job
    /// Parameters:
    /// time - proposed timeframe for completing a Job
    /// payment - proposed payment for a Job
    /// reputation_stake - reputation stake for a Job if Worker is an Internal Worker
    /// onboard - if Worker is an External Worker, then Worker can request to be onboarded after
    /// completing a Job
    /// purse: purse containing stake from External Worker
    ///
    /// # Events
    /// Emits [`BidSubmitted`](crate::bid::events::BidSubmitted)
    fn submit_bid(&mut self, job_id: BidId, time: BlockTime, payment: U512, reputation_stake: U256, onboard: bool, purse: Option<URef>);

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
        document_hash: DocumentHash,
        time: BlockTime,
        required_stake: Option<U256>,
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
    /// Cancel job. This can be done by anyone when Worker did not submit Job Result and after Grace
    /// Period has passed.
    /// It refunds cspr to job creator.
    /// # Events
    /// Emits [`JobCancelled`](JobCancelled)
    ///
    /// # Errors
    /// Throws [`CannotCancelJob`](Error::CannotCancelJob) if one of the constraints for
    /// job cancellation is not met.
    fn cancel_job(&mut self, bid_id: BidId, reason: DocumentHash);
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
    fn submit_result(&mut self, bid_id: BidId, result: DocumentHash);
    /// Casts a vote over a job
    /// # Events
    /// Emits [`BallotCast`](crate::voting::governance_voting::events::BallotCast)

    /// # Errors
    /// Throws [`CannotVoteOnOwnJob`](Error::CannotVoteOnOwnJob) if the voter is either of Job Poster or Worker
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn vote(&mut self, job_id: JobId, choice: Choice, stake: U256);
    /// Returns a job with given JobId
    fn get_job(&self, job_id: JobId) -> Option<Job>;
    /// Returns a JobOffer with given JobOfferId
    fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;
    /// Returns a Bid with given BidId
    fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
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
    fn get_voting(&self, voting_id: VotingId) -> Option<Voting>;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_ballot(&self, voting_id: VotingId, address: Address) -> Option<Ballot>;
    /// see [GovernanceVoting](GovernanceVoting)
    fn get_voter(&self, voting_id: VotingId, at: u32) -> Option<Address>;
    /// Returns the CSPR balance of the contract
    fn get_cspr_balance(&self) -> U512;
}

#[derive(Instance)]
pub struct BidEscrowContract {
    voting: GovernanceVoting,
    kyc: KycInfo,
    onboarding: OnboardingInfo,
    jobs: Mapping<JobId, Job>,
    jobs_count: SequenceGenerator<JobId>,
    job_offers: Mapping<JobOfferId, JobOffer>,
    job_offers_count: SequenceGenerator<JobOfferId>,
    bids: Mapping<BidId, Bid>,
    bids_count: SequenceGenerator<BidId>,
}

impl BidEscrowContractInterface for BidEscrowContract {
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.voting.init(variable_repo, reputation_token, va_token);
        self.kyc.init(kyc_token);
        self.onboarding.init(va_token);
    }

    fn post_job_offer(&mut self, expected_timeframe: BlockTime, max_budget: U512, purse: URef) {
        if !self.kyc.is_kycd(&caller()) {
            revert(Error::JobPosterNotKycd);
        }

        self.deposit_dos_fee(purse);

        let job_offer_id = self.next_job_offer_id();
        let job_offer = JobOffer::new(job_offer_id, caller(), expected_timeframe, max_budget);

        self.job_offers.set(&job_offer_id, job_offer);

        // JobOfferCreated::new(&job_offer).emit();
    }

    fn submit_bid(&mut self, job_id: BidId, time: BlockTime, payment: U512, reputation_stake: U256, onboard: bool, purse: Option<URef>) {
        if !self.kyc.is_kycd(&caller()) {
            revert(Error::WorkerNotKycd);
        }

        // let job_offer = self.get_job_offer(job_id).unwrap_or_else(|| revert(Error::JobOfferNotFound));

        // if job_offer.job_poster == caller() {
        //     revert(Error::CannotBidOnOwnJob);
        // }

        // if payment > job_offer.max_budget {
        //     revert(Error::PaymentExceedsMaxBudget);
        // }

        // // TODO: Implement rest of constraints

        // let bid_id = self.next_bid_id();
        // let bid = Bid::new(bid_id, job_id, caller(), time, payment, reputation_stake);

        // self.bids.set(&bid_id, bid);

        // BidCreated::new(&bid).emit();
    }

    fn pick_bid(
        &mut self,
        worker: Address,
        document_hash: DocumentHash,
        time: BlockTime,
        required_stake: Option<U256>,
        purse: URef,
    ) {
        let cspr_amount = self.deposit(purse);
        let caller = caller();

        if worker == caller {
            revert(Error::CannotPostJobForSelf);
        }

        if !self.kyc.is_kycd(&caller) {
            revert(Error::JobPosterNotKycd);
        }

        if !self.kyc.is_kycd(&worker) {
            revert(Error::WorkerNotKycd);
        }

        if !self.onboarding.is_onboarded(&worker) && required_stake.is_some() {
            revert(Error::NotOnboardedWorkerCannotStakeReputation);
        }

        let bid_id = self.next_bid_id();
        let finish_time = get_block_time() + time;

        let mut job = Job::new(
            bid_id,
            document_hash,
            caller,
            worker,
            finish_time,
            required_stake,
            cspr_amount,
        );

        JobCreated::new(&job).emit();

        if !self.onboarding.is_onboarded(&worker) {
            job.accept(job.worker(), get_block_time())
                .unwrap_or_revert();
            JobAccepted::new(&job).emit();
        }

        self.jobs.set(&bid_id, job);
    }

    fn accept_job(&mut self, bid_id: BidId) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        if job.required_stake().is_some() {
            ReputationContractCaller::at(self.voting.get_reputation_token_address()).transfer_from(
                job.worker(),
                self_address(),
                job.required_stake().unwrap_or_default(),
            );
        }

        job.accept(caller(), get_block_time()).unwrap_or_revert();

        JobAccepted::new(&job).emit();

        self.jobs.set(&bid_id, job);
    }

    fn cancel_job(&mut self, bid_id: BidId, reason: DocumentHash) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        let caller = caller();

        job.cancel(caller).unwrap_or_revert();
        self.refund(&job);

        JobCancelled::new(&job, caller, reason).emit();

        self.jobs.set(&bid_id, job);
    }

    fn submit_result(&mut self, bid_id: BidId, result: DocumentHash) {
        let mut job = self.jobs.get_or_revert(&bid_id);

        job.submit(caller(), get_block_time(), result)
            .unwrap_or_revert();

        JobSubmitted::new(&job).emit();

        let voting_configuration = VotingConfigurationBuilder::defaults(&self.voting)
            .cast_first_vote(false)
            .create_minimum_reputation(U256::zero())
            .only_va_can_create(false)
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

        let caller = caller();

        if caller == job.poster() || caller == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting.vote(caller, voting_id, choice, stake);
    }

    fn get_job(&self, bid_id: BidId) -> Option<Job> {
        self.jobs.get_or_none(&bid_id)
    }

    delegate! {
        to self.voting {
            fn get_dust_amount(&self) -> U256;
            fn get_variable_repo_address(&self) -> Address;
            fn get_reputation_token_address(&self) -> Address;
            fn get_voting(&self, voting_id: VotingId) -> Option<Voting>;
            fn get_ballot(&self, voting_id: VotingId, address: Address) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, at: u32) -> Option<Address>;
        }
    }

    fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer> {
        self.job_offers.get_or_none(&job_offer_id)
    }

    fn get_bid(&self, bid_id: BidId) -> Option<Bid> {
        self.bids.get_or_none(&bid_id)
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
                    self.job_rejected(&mut job);
                }
                VotingResult::QuorumNotReached => {
                    self.job_rejected(&mut job);
                }
            },
            crate::voting::voting::VotingType::Formal => match voting_summary.result() {
                VotingResult::InFavor => {
                    self.job_done(&mut job);
                }
                VotingResult::Against => {
                    self.job_rejected(&mut job);
                }
                VotingResult::QuorumNotReached => {
                    self.job_rejected(&mut job);
                }
            },
        }

        self.jobs.set(&bid_id, job);
    }

    fn get_cspr_balance(&self) -> U512 {
        get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
    }
}

impl BidEscrowContract {
    fn next_bid_id(&mut self) -> BidId {
        let bid_id = self.jobs_count.get_current_value();
        self.jobs_count.next_value();
        bid_id
    }

    fn next_job_offer_id(&mut self) -> JobOfferId {
        let job_offer_id = self.job_offers_count.get_current_value();
        self.job_offers_count.next_value();
        job_offer_id
    }

    fn next_job_id(&mut self) -> JobId {
        let job_id = self.jobs_count.get_current_value();
        self.jobs_count.next_value();
        job_id
    }

    fn deposit(&mut self, cargo_purse: URef) -> U512 {
        let main_purse = casper_env::contract_main_purse();
        let amount = get_purse_balance(cargo_purse).unwrap_or_revert();
        transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None).unwrap_or_revert();
        amount
    }

    /// Deposits a dos fee into the contract, checking the constraints
    fn deposit_dos_fee(&mut self, cargo_purse: URef) -> U512 {
        let main_purse = casper_env::contract_main_purse();
        let amount = get_purse_balance(cargo_purse).unwrap_or_revert();
        if amount < self.minimum_dos_fee() {
            revert(Error::DosFeeTooLow);
        }
        transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None).unwrap_or_revert();
        amount
    }

    fn minimum_dos_fee(&mut self) -> U512 {
        // TODO: Implement using external contract and Governance Variable
        U512::from(100_000_000_000u64)
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

    fn job_done(&mut self, job: &mut Job) {
        self.pay_for_job(job);
        if job.required_stake().is_some() {
            ReputationContractCaller::at(self.voting.get_reputation_token_address()).transfer_from(
                self_address(),
                job.worker(),
                job.required_stake().unwrap_or_default(),
            );
        }
        job.complete();
        JobDone::new(job, caller()).emit();
    }

    fn job_rejected(&mut self, job: &mut Job) {
        if job.required_stake().is_some() {
            ReputationContractCaller::at(self.voting.get_reputation_token_address())
                .burn(self_address(), job.required_stake().unwrap_or_default());
        }
        self.refund(job);
        job.not_completed();
        JobRejected::new(job, caller()).emit();
    }

    fn refund(&mut self, job: &Job) {
        self.withdraw(job.poster(), job.cspr_amount());
    }

    fn mint_and_redistribute_reputation(&mut self, job: &Job) {
        let reputation_to_mint =
            VariableRepositoryContractCaller::at(self.voting.get_variable_repo_address())
                .reputation_to_mint(job.cspr_amount());
        let reputation_to_redistribute =
            VariableRepositoryContractCaller::at(self.voting.get_variable_repo_address())
                .reputation_to_redistribute(reputation_to_mint);

        // Worker
        ReputationContractCaller::at(self.voting.get_reputation_token_address()).mint(
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
                ReputationContractCaller::at(self.voting.get_reputation_token_address())
                    .mint(ballot.voter, to_transfer);
            }
        }
    }
}

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;
use crate::bid::bid::Bid;
use crate::bid::job_offer::{JobOffer, JobOfferStatus};
use crate::bid::types::{JobId, JobOfferId};

#[cfg(feature = "test-support")]
impl BidEscrowContractTest {
    pub fn pick_bid_with_cspr_amount(
        &mut self,
        worker: Address,
        document_hash: DocumentHash,
        time: BlockTime,
        required_stake: Option<U256>,
        cspr_amount: U512,
    ) {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "pick_bid.wasm",
            runtime_args! {
                "token_address" => self.address(),
                "cspr_amount" => cspr_amount,
                "worker" => worker,
                "document_hash" => document_hash,
                "time" => time,
                "required_stake" => required_stake,
                "amount" => cspr_amount,
            },
        );
    }
}
