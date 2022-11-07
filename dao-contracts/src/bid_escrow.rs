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
        events::{JobCancelled, JobDone, JobRejected},
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
use delegate::delegate;

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
    fn submit_bid(
        &mut self,
        job_id: BidId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U256,
        onboard: bool,
        purse: Option<URef>,
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
    fn pick_bid(&mut self, job_offer_id: u32, bid_id: u32, purse: URef);
    /// Submits a job proof. This is called by a Worker or any KYC'd user during Grace Period.
    /// This starts a new voting over the result.
    /// # Events
    /// Emits [`JobProofSubmitted`](JobProofSubmitted)
    ///
    /// Emits [`VotingCreated`](crate::voting::governance_voting::events::VotingCreated)
    ///
    /// # Errors
    /// Throws [`JobAlreadySubmitted`](Error::JobAlreadySubmitted) if job was already submitted
    /// Throws [`NotAuthorizedToSubmitResult`](Error::NotAuthorizedToSubmitResult) if one of the constraints for
    /// job submission is not met.
    fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash);
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
    fn finish_voting(&mut self, job_id: JobId);
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

    fn submit_bid(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U256,
        onboard: bool,
        purse: Option<URef>,
    ) {
        let worker = caller();

        if !self.kyc.is_kycd(&worker) {
            revert(Error::WorkerNotKycd);
        }

        let job_offer = self
            .get_job_offer(job_offer_id.clone())
            .unwrap_or_revert_with(Error::JobOfferNotFound);

        if job_offer.job_poster == worker {
            revert(Error::CannotBidOnOwnJob);
        }

        if payment > job_offer.max_budget {
            revert(Error::PaymentExceedsMaxBudget);
        }

        // TODO: Implement rest of constraints

        let bid_id = self.next_bid_id();

        let cspr_stake = match purse {
            None => None,
            Some(purse) => {
                let cspr_stake = self.deposit(purse);
                Some(cspr_stake)
            }
        };
        let bid = Bid::new(
            bid_id,
            job_offer_id,
            time,
            payment,
            reputation_stake,
            cspr_stake,
            onboard,
            worker,
        );

        self.bids.set(&bid_id, bid);

        // TODO: Implement Event
        // BidCreated::new(&bid).emit();
    }

    fn pick_bid(&mut self, job_offer_id: u32, bid_id: u32, purse: URef) {
        let job_offer = self
            .get_job_offer(job_offer_id.clone())
            .unwrap_or_revert_with(Error::JobOfferNotFound);
        let job_poster = caller();

        if job_offer.job_poster != job_poster {
            revert(Error::OnlyJobPosterCanPickABid);
        }

        let bid = self
            .get_bid(bid_id.clone())
            .unwrap_or_revert_with(Error::BidNotFound);

        let cspr_amount = self.deposit(purse);

        if cspr_amount != bid.proposed_payment {
            revert(Error::PurseBalanceMismatch)
        }

        let finish_time = get_block_time() + bid.proposed_timeframe;
        let job_id = self.next_job_id();

        let job = Job::new(
            job_id,
            bid_id,
            job_offer_id,
            finish_time,
            bid.worker,
            job_poster,
            bid.proposed_payment,
            bid.reputation_stake,
        );

        self.jobs.set(&job_id, job);

        // TODO: Emit event.
    }

    fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash) {
        let mut job = self
            .get_job(job_id.clone())
            .unwrap_or_revert_with(Error::JobNotFound);
        let worker = caller();

        // if job.worker() != worker {
        //     if !job.is_grace_period(get_block_time()) {
        //         revert(Error::OnlyWorkerCanSubmitProof);
        //     } else {
        //         if self.kyc.is_kycd(&worker) {
        //             revert(Error::WorkerNotKycd);
        //         }
        //     }
        //
        //     // TODO: Implement Worker switching during Grace Period
        // }

        job.submit_proof(proof);
        // TODO: Emit event.

        let voting_configuration = VotingConfigurationBuilder::defaults(&self.voting)
            .cast_first_vote(true)
            .create_minimum_reputation(U256::zero())
            .only_va_can_create(false)
            .build();

        let voting_id = self
            .voting
            .create_voting(worker, job.stake(), voting_configuration);

        job.set_informal_voting_id(Some(voting_id));

        self.jobs.set(&job_id, job);
    }

    // fn pick_bid(
    //     &mut self,
    //     worker: Address,
    //     document_hash: DocumentHash,
    //     time: BlockTime,
    //     required_stake: Option<U256>,
    //     purse: URef,
    // ) {

    //     if !self.kyc.is_kycd(&caller) {
    //         revert(Error::JobPosterNotKycd);
    //     }

    //     if !self.kyc.is_kycd(&worker) {
    //         revert(Error::WorkerNotKycd);
    //     }

    //     if !self.onboarding.is_onboarded(&worker) && required_stake.is_some() {
    //         revert(Error::NotOnboardedWorkerCannotStakeReputation);
    //     }

    //     JobCreated::new(&job).emit();

    //     if !self.onboarding.is_onboarded(&worker) {
    //         job.accept(job.worker(), get_block_time())
    //             .unwrap_or_revert();
    //         JobAccepted::new(&job).emit();
    //     }

    //     self.jobs.set(&bid_id, job);
    // }

    fn cancel_job(&mut self, bid_id: BidId, reason: DocumentHash) {
        let mut job = self.jobs.get_or_revert(&bid_id);
        let caller = caller();

        job.cancel(caller).unwrap_or_revert();
        self.refund(&job);

        JobCancelled::new(&job, caller, reason).emit();

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

    fn finish_voting(&mut self, job_id: JobId) {
        let mut job = self.jobs.get_or_revert(&job_id);
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

        self.jobs.set(&job_id, job);
    }

    fn get_cspr_balance(&self) -> U512 {
        get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
    }
}

impl BidEscrowContract {
    fn next_bid_id(&mut self) -> BidId {
        self.bids_count.next_value()
    }

    fn next_job_offer_id(&mut self) -> JobOfferId {
        self.job_offers_count.next_value()
    }

    fn next_job_id(&mut self) -> JobId {
        self.jobs_count.next_value()
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
        self.withdraw(job.worker(), job.payment());
        self.mint_and_redistribute_reputation(job);
    }

    fn job_done(&mut self, job: &mut Job) {
        self.pay_for_job(job);
        job.complete();
        JobDone::new(job, caller()).emit();
    }

    fn job_rejected(&mut self, job: &mut Job) {
        self.refund(job);
        job.not_completed();
        JobRejected::new(job, caller()).emit();
    }

    fn refund(&mut self, job: &Job) {
        self.withdraw(job.poster(), job.payment());
    }

    fn mint_and_redistribute_reputation(&mut self, job: &Job) {
        let reputation_to_mint =
            VariableRepositoryContractCaller::at(self.voting.get_variable_repo_address())
                .reputation_to_mint(job.payment());
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

use crate::bid::bid::Bid;
use crate::bid::job_offer::JobOffer;
use crate::bid::types::{JobId, JobOfferId};
#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

#[cfg(feature = "test-support")]
impl BidEscrowContractTest {
    pub fn pick_bid_with_cspr_amount(&mut self, job_offer_id: u32, bid_id: u32, cspr_amount: U512) {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "pick_bid.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "job_offer_id" => job_offer_id,
                "bid_id" => bid_id,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        );
    }

    pub fn post_job_offer_with_cspr_amount(
        &mut self,
        expected_timeframe: BlockTime,
        budget: U512,
        cspr_amount: U512,
    ) {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "post_job_offer.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "expected_timeframe" => expected_timeframe,
                "budget" => budget,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        );
    }
}
