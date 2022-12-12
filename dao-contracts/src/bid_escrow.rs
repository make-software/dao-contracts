use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::{
        contract_api::system::{
            get_purse_balance,
            transfer_from_purse_to_account,
            transfer_from_purse_to_purse,
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, get_block_time, revert},
    Address,
    BlockTime,
    DocumentHash,
    Error,
    Mapping,
    SequenceGenerator,
    VecMapping,
};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::{
    escrow::{
        job::{Job, WorkerType},
        types::BidId,
    },
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting_state_machine::{VotingResult, VotingStateMachine},
        Ballot,
        Choice,
        VotingEngine,
        VotingId,
    },
    Configuration,
    ConfigurationBuilder,
    ReputationContractCaller,
    ReputationContractInterface,
    VaNftContractCaller,
    VaNftContractInterface,
};

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    /// Initializes the module with [Addresses](Address) of [Reputation Token](crate::ReputationContract), [Variable Repo](crate::VariableRepositoryContract)
    /// KYC Token and VA Token
    ///
    /// # Events
    /// Emits [`VotingContractCreated`](crate::voting::voting_engine::events::VotingContractCreated)
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
    /// Emits [`JobOfferCreated`](crate::escrow::events::JobOfferCreated)
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
    /// Emits [`BidSubmitted`](crate::escrow::events::BidSubmitted)
    fn submit_bid(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    );
    /// Worker cancels a Bid for a Job
    /// Parameters:
    /// bid_id - Bid Id
    ///
    /// Bid can be cancelled only after VABidAcceptanceTimeout time has passed after submitting a Bid
    fn cancel_bid(&mut self, bid_id: BidId);
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
    /// Emits [`VotingCreated`](crate::voting::voting_engine::events::VotingCreated)
    ///
    /// # Errors
    /// Throws [`JobAlreadySubmitted`](Error::JobAlreadySubmitted) if job was already submitted
    /// Throws [`NotAuthorizedToSubmitResult`](Error::NotAuthorizedToSubmitResult) if one of the constraints for
    /// job submission is not met.
    fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash);
    /// Casts a vote over a job
    /// # Events
    /// Emits [`BallotCast`](crate::voting::voting_engine::events::BallotCast)

    /// # Errors
    /// Throws [`CannotVoteOnOwnJob`](Error::CannotVoteOnOwnJob) if the voter is either of Job Poster or Worker
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// Returns a job with given JobId
    fn get_job(&self, job_id: JobId) -> Option<Job>;
    /// Returns a JobOffer with given JobOfferId
    fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;
    /// Returns a Bid with given BidId
    fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
    /// Finishes voting stage. Depending on stage, the voting can be converted to a formal one, end
    /// with a refund or pay the worker.
    /// # Events
    /// Emits [`VotingEnded`](crate::voting::voting_engine::events::VotingEnded), [`VotingCreated`](crate::voting::voting_engine::events::VotingCreated)
    /// # Errors
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn finish_voting(&mut self, job_id: JobId);
    /// see [VotingEngine](VotingEngine)
    fn variable_repo_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine)
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine)
    fn get_voting(
        &self,
        voting_id: VotingId,
    ) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine)
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine)
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;

    /// Returns the CSPR balance of the contract
    fn get_cspr_balance(&self) -> U512;

    fn job_offers_count(&self) -> u32;

    fn jobs_count(&self) -> u32;

    fn bids_count(&self) -> u32;

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId);

    fn slash_all_active_job_offers(&mut self, bidder: Address);

    fn slash_bid(&mut self, bid_id: BidId);

    // Whitelisting set.
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;

    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
}

#[derive(Instance)]
pub struct BidEscrowContract {
    voting: VotingEngine,
    kyc: KycInfo,
    onboarding: OnboardingInfo,
    jobs: Mapping<JobId, Job>,
    jobs_for_voting: Mapping<VotingId, JobId>,
    jobs_count: SequenceGenerator<JobId>,
    job_offers: Mapping<JobOfferId, JobOffer>,
    active_job_offers_ids: Mapping<Address, Vec<JobOfferId>>,
    pub job_offers_count: SequenceGenerator<JobOfferId>,
    bids: Mapping<BidId, Bid>,
    job_offers_bids: VecMapping<JobOfferId, BidId>,
    bids_count: SequenceGenerator<BidId>,
    access_control: AccessControl,
}

impl BidEscrowContractInterface for BidEscrowContract {
    delegate! {
        to self.voting {
            fn variable_repo_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
        }

        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }
    }

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
        self.access_control.init(caller());
    }

    fn post_job_offer(&mut self, expected_timeframe: BlockTime, max_budget: U512, purse: URef) {
        let job_poster = caller();
        if !self.kyc.is_kycd(&job_poster) {
            revert(Error::JobPosterNotKycd);
        }
        let voting_configuration = ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
        )
        .bid_escrow()
        .build();

        let dos_fee = self.deposit_dos_fee(purse, &voting_configuration);

        let job_offer_id = self.next_job_offer_id();
        let job_offer = JobOffer::new(
            job_offer_id,
            job_poster,
            expected_timeframe,
            max_budget,
            dos_fee,
            get_block_time(),
            voting_configuration,
        );
        self.job_offers.set(&job_offer_id, job_offer);

        let mut job_offers = self
            .active_job_offers_ids
            .get(&job_poster)
            .unwrap_or_default();
        job_offers.push(job_offer_id);
        self.active_job_offers_ids.set(&job_poster, job_offers);
        // JobOfferCreated::new(&job_offer).emit();
    }

    fn submit_bid(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    ) {
        let worker = caller();

        if !self.kyc.is_kycd(&worker) {
            revert(Error::WorkerNotKycd);
        }

        let job_offer: JobOffer = self
            .get_job_offer(job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound);

        if job_offer.job_poster == worker {
            revert(Error::CannotBidOnOwnJob);
        }

        let is_va = self.onboarding.is_onboarded(&worker);

        if onboard && is_va {
            revert(Error::VaOnboardedAlready);
        }

        let bid_validation = job_offer.validate_bid(get_block_time(), self.is_va(worker), payment);

        if let Err(error) = bid_validation {
            revert(error);
        }

        // TODO: Implement rest of constraints

        let bid_id = self.next_bid_id();

        let cspr_stake = match purse {
            None => {
                self.reputation_token()
                    .stake_bid(worker, bid_id, reputation_stake);
                None
            }
            Some(purse) => {
                let cspr_stake = self.deposit(purse);
                Some(cspr_stake)
            }
        };

        let bid = Bid::new(
            bid_id,
            get_block_time(),
            job_offer_id,
            time,
            payment,
            reputation_stake,
            cspr_stake,
            onboard,
            worker,
        );

        self.bids.set(&bid_id, bid);
        self.job_offers_bids.add(job_offer_id, bid_id);

        // TODO: Implement Event
        // BidCreated::new(&bid).emit();
    }

    fn cancel_bid(&mut self, bid_id: BidId) {
        let worker = caller();
        let mut bid = self
            .get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound);

        if bid.worker != worker {
            revert(Error::CannotCancelNotOwnedBid);
        }

        let job_offer = self
            .get_job_offer(bid.job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound);

        if job_offer.status != JobOfferStatus::Created {
            revert(Error::CannotCancelBidOnCompletedJobOffer);
        }

        if get_block_time() < bid.timestamp + job_offer.configuration.va_bid_acceptance_timeout() {
            revert(Error::CannotCancelBidBeforeAcceptanceTimeout);
        }

        bid.reclaim();

        match bid.cspr_stake {
            None => {
                self.reputation_token().unstake_bid(bid.worker, bid_id);
            }
            Some(cspr_stake) => {
                self.withdraw(bid.worker, cspr_stake);
            }
        }

        // TODO: Implement Event
        self.bids.set(&bid_id, bid);
    }

    fn pick_bid(&mut self, job_offer_id: u32, bid_id: u32, purse: URef) {
        let job_offer = self
            .get_job_offer(job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound);
        let job_poster = caller();

        if job_offer.job_poster != job_poster {
            revert(Error::OnlyJobPosterCanPickABid);
        }

        let mut bid = self
            .get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound);

        let cspr_amount = self.deposit(purse);

        if cspr_amount != bid.proposed_payment {
            revert(Error::PurseBalanceMismatch)
        }

        // TODO: Unstake all bidders for the given job offer.
        self.unstake_not_picked(job_offer_id, bid_id);

        let worker_type = if self.is_va(bid.worker) {
            WorkerType::Internal
        } else if bid.onboard {
            WorkerType::ExternalToVA
        } else {
            WorkerType::External
        };

        let finish_time = get_block_time() + bid.proposed_timeframe;
        let job_id = self.next_job_id();

        let job = Job::new(
            job_id,
            bid_id,
            job_offer_id,
            finish_time,
            bid.worker,
            worker_type,
            job_poster,
            bid.proposed_payment,
            bid.reputation_stake,
            bid.cspr_stake.unwrap_or_default(),
        );

        self.jobs.set(&job_id, job);

        bid.pick();
        self.bids.set(&bid_id, bid);

        // TODO: Filter in place.
        let job_offers: Vec<JobOfferId> = self
            .active_job_offers_ids
            .get(&job_poster)
            .unwrap_or_default();
        let job_offers: Vec<JobOfferId> = job_offers
            .iter()
            .filter(|id| id == &&job_offer_id)
            .cloned()
            .collect();
        self.active_job_offers_ids.set(&job_poster, job_offers);
        // TODO: Emit event.
    }

    fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash) {
        let mut job = self
            .get_job(job_id)
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

        let job_offer = self.job_offer(job.job_offer_id());

        if job.stake() != U512::zero() && job_offer.configuration.informal_stake_reputation() {
            self.reputation_token().unstake_bid(worker, job.bid_id());
        }

        let voting_configuration = ConfigurationBuilder::new(
            self.voting.variable_repo_address(),
            self.voting.va_token_address(),
            )
            .only_va_can_create(false)
            .is_bid_escrow(true)
            .build();

        let stake = if job.external_worker_cspr_stake().is_zero() {
            job.stake()
        } else {
            voting_configuration.apply_reputation_conversion_rate_to(job.external_worker_cspr_stake())
        };

        let voting_id = self
            .voting
            .create_voting(worker, U512::zero(), voting_configuration);

        self.jobs_for_voting.set(&voting_id, job_id);
        
        let is_unbounded = job.worker_type() != &WorkerType::Internal;
        self.voting.cast_ballot(
            worker,
            voting_id,
            Choice::InFavor,
            stake,
            is_unbounded,
            self.voting
                .get_voting(voting_id)
                .unwrap_or_revert_with(Error::VotingDoesNotExist),
        );

        job.set_voting_id(voting_id);

        self.jobs.set(&job_id, job);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        let job = self.jobs.get_or_revert(&self.job_id_by_voting_id(voting_id));
        let caller = caller();

        if caller == job.poster() || caller == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting.vote(caller, voting_id, voting_type, choice, stake);
    }

    fn get_job(&self, bid_id: BidId) -> Option<Job> {
        self.jobs.get_or_none(&bid_id)
    }

    fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer> {
        self.job_offers.get_or_none(&job_offer_id)
    }

    fn get_bid(&self, bid_id: BidId) -> Option<Bid> {
        self.bids.get_or_none(&bid_id)
    }

    fn finish_voting(&mut self, job_id: JobId) {
        let job = self.jobs.get_or_revert(&job_id);
        let job_offer = self.job_offer(job.job_offer_id());
        let voting_id = job
            .voting_id()
            .unwrap_or_revert_with(Error::VotingNotStarted);
        let voting_summary = self
            .voting
            .finish_voting_without_token_redistribution(voting_id);
        match voting_summary.voting_type() {
            VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor => {
                    if !job_offer.configuration.informal_stake_reputation() {
                        self.reputation_token()
                            .unstake_bid(job.worker(), job.bid_id());
                    }
                    self.create_formal_voting(voting_id);
                }
                VotingResult::Against => {
                    if !job_offer.configuration.informal_stake_reputation() {
                        self.reputation_token()
                            .unstake_bid(job.worker(), job.bid_id());
                    }
                    self.create_formal_voting(voting_id);
                }
                VotingResult::QuorumNotReached => {
                    if job_offer.configuration.informal_stake_reputation() {
                        self.voting.return_reputation_of_yes_voters(voting_id, VotingType::Informal);
                        self.voting.return_reputation_of_no_voters(voting_id, VotingType::Informal);
                    }
                    self.return_job_poster_payment_and_dos_fee(&job);
                    self.return_external_worker_cspr_stake(&job);
                }
                VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
            },
            VotingType::Formal => {
                match voting_summary.result() {
                    VotingResult::InFavor => match job.worker_type() {
                        WorkerType::Internal => {
                            self.voting.return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_no_voters(voting_id, VotingType::Formal);
                            self.mint_and_redistribute_reputation_for_internal_worker(&job);
                            self.redistribute_cspr_internal_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                        }
                        WorkerType::ExternalToVA => {
                            // Make user VA.
                            self.va_token().mint(job.worker());

                            self.return_external_worker_cspr_stake(&job);
                            // Bound ballot for worker.
                            self.voting.bound_ballot(voting_id, job.worker(), VotingType::Formal);

                            self.voting.return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_no_voters(voting_id, VotingType::Formal);
                            self.mint_and_redistribute_reputation_for_internal_worker(&job);
                            self.burn_external_worker_reputation(&job);
                            self.redistribute_cspr_internal_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                        }
                        WorkerType::External => {
                            self.voting.return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_no_voters(voting_id, VotingType::Formal);
                            self.mint_and_redistribute_reputation_for_external_worker(&job);
                            self.redistribute_cspr_external_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                            self.return_external_worker_cspr_stake(&job);
                        }
                    },
                    VotingResult::Against => match job.worker_type() {
                        WorkerType::Internal => {
                            self.voting.return_reputation_of_no_voters(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_yes_voters(voting_id, VotingType::Formal);
                            self.return_job_poster_payment_and_dos_fee(&job);
                            self.slash_worker(&job);
                        }
                        WorkerType::ExternalToVA | WorkerType::External => {
                            self.voting.return_reputation_of_no_voters(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_yes_voters(voting_id, VotingType::Formal);
                            self.return_job_poster_payment_and_dos_fee(&job);
                            self.redistribute_cspr_external_worker_failed(&job);
                        }
                    },
                    VotingResult::QuorumNotReached => {
                        self.voting.return_reputation_of_yes_voters(voting_id, VotingType::Formal);
                        self.voting.return_reputation_of_no_voters(voting_id, VotingType::Formal);
                        self.return_job_poster_payment_and_dos_fee(&job);
                        self.return_external_worker_cspr_stake(&job);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
            }
        }

        self.jobs.set(&job_id, job);
    }

    fn get_cspr_balance(&self) -> U512 {
        get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
    }

    fn get_voting(
        &self,
        voting_id: VotingId,
    ) -> Option<VotingStateMachine> {
        self.voting.get_voting(voting_id)
    }

    fn job_offers_count(&self) -> u32 {
        self.job_offers_count.get_current_value()
    }

    fn jobs_count(&self) -> u32 {
        self.jobs_count.get_current_value()
    }

    fn bids_count(&self) -> u32 {
        self.bids_count.get_current_value()
    }

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }

    fn slash_all_active_job_offers(&mut self, bidder: Address) {
        self.access_control.ensure_whitelisted();
        // Cancel job offers created by the bidder.
        let job_offer_ids = self.active_job_offers_ids.get(&bidder).unwrap_or_default();
        for job_offer_id in job_offer_ids {
            self.cancel_job_offer(job_offer_id);
        }
        self.active_job_offers_ids.set(&bidder, vec![]);
    }

    fn slash_bid(&mut self, bid_id: BidId) {
        self.access_control.ensure_whitelisted();

        let mut bid = self
            .get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound);

        let job_offer = self
            .get_job_offer(bid.job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound);

        if job_offer.status != JobOfferStatus::Created {
            revert(Error::CannotCancelBidOnCompletedJobOffer);
        }

        bid.cancel();

        self.reputation_token().unstake_bid(bid.worker, bid_id);

        // TODO: Implement Event
        self.bids.set(&bid_id, bid);
    }

    fn slash_voter(&mut self, _voter: Address, _voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        unimplemented!()
    }
}

impl BidEscrowContract {
    fn slash_worker(&self, job: &Job) {
        let config = self.get_job_offer_configuration(&job);
        let worker_balance = self.reputation_token().balance_of(job.worker());
        let amount_to_burn = config.apply_default_reputation_slash_to(worker_balance);
        self.reputation_token().burn(job.worker(), amount_to_burn);
    }

    fn cancel_job_offer(&mut self, job_offer_id: JobOfferId) {
        let bids_amount = self.job_offers_bids.len(job_offer_id);
        for i in 0..bids_amount {
            let unstake_bid_id = self
                .job_offers_bids
                .get(job_offer_id, i)
                .unwrap_or_revert_with(Error::BidNotFound);
            let mut bid = self
                .get_bid(unstake_bid_id)
                .unwrap_or_revert_with(Error::BidNotFound);

            match bid.cspr_stake {
                None => {
                    self.reputation_token()
                        .unstake_bid(bid.worker, unstake_bid_id);
                }
                Some(cspr_stake) => {
                    self.withdraw(bid.worker, cspr_stake);
                }
            }
            bid.cancel();
            self.bids.set(&unstake_bid_id, bid);
        }
        self.return_job_offer_poster_dos_fee(&job_offer_id);
    }

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
    fn deposit_dos_fee(&mut self, cargo_purse: URef, configuration: &Configuration) -> U512 {
        let main_purse = casper_env::contract_main_purse();
        let amount = get_purse_balance(cargo_purse).unwrap_or_revert();

        self.validate_dos_fee(amount, configuration);

        transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None).unwrap_or_revert();
        amount
    }

    fn validate_dos_fee(&self, dos_fee: U512, configuration: &Configuration) {
        let fiat_conversion_rate_address = configuration.fiat_conversion_rate_address();

        let fiat_value =
            casper_dao_utils::cspr_rate::convert_to_fiat(dos_fee, fiat_conversion_rate_address);
        if configuration.is_post_job_dos_fee_too_low(fiat_value) {
            revert(Error::DosFeeTooLow);
        };
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

    fn redistribute_cspr_internal_worker(&mut self, job: &Job) {
        let to_redistribute = self.redistribute_to_governance(job, job.payment());
        let redistribute_to_all_vas = self
            .job_offer(job.job_offer_id())
            .configuration
            .distribute_payment_to_non_voters();

        // For VA's
        if redistribute_to_all_vas {
            self.redistribute_cspr_to_all_vas(to_redistribute);
        } else {
            self.redistribute_cspr_to_voters(job, to_redistribute);
        }
    }

    fn redistribute_cspr_external_worker(&mut self, job: &Job) {
        let total_left = self.redistribute_to_governance(job, job.payment());
        let config = self.get_job_offer_configuration(job);
        let to_redistribute = config.apply_default_policing_rate_to(total_left);
        let to_worker = total_left - to_redistribute;

        // For External Worker
        self.withdraw(job.worker(), to_worker);

        let redistribute_to_all_vas = self
            .job_offer(job.job_offer_id())
            .configuration
            .distribute_payment_to_non_voters();

        // For VA's
        if redistribute_to_all_vas {
            self.redistribute_cspr_to_all_vas(to_redistribute);
        } else {
            self.redistribute_cspr_to_voters(job, to_redistribute);
        }
    }

    fn redistribute_cspr_external_worker_failed(&mut self, job: &Job) {
        let total_left = self.redistribute_to_governance(job, job.external_worker_cspr_stake());

        // For VA's
        let (total_supply, balances) = self.reputation_token().all_balances();
        for (address, balance) in balances.balances {
            let amount = total_left * balance / total_supply;
            self.withdraw(address, amount);
        }
    }

    fn redistribute_to_governance(&mut self, job: &Job, payment: U512) -> U512 {
        let configuration = self.get_job_offer_configuration(job);
        
        let governance_wallet: Address = configuration.bid_escrow_wallet_address();
        let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(payment);
        self.withdraw(governance_wallet, governance_wallet_payment);

        payment - governance_wallet_payment
    }

    fn job_offer(&self, job_offer_id: JobOfferId) -> JobOffer {
        self.job_offers
            .get(&job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound)
    }

    fn job_id_by_voting_id(&self, voting_id: VotingId) -> JobId {
        self.jobs_for_voting
            .get(&voting_id)
            .unwrap_or_revert_with(Error::VotingIdNotFound)
    }

    fn redistribute_cspr_to_all_vas(&mut self, to_redistribute: U512) {
        let (total_supply, balances) = self.reputation_token().all_balances();
        for (address, balance) in balances.balances {
            let amount = to_redistribute * balance / total_supply;
            self.withdraw(address, amount);
        }
    }

    fn redistribute_cspr_to_voters(&mut self, job: &Job, to_redistribute: U512) {
        let voting_id = job
            .voting_id()
            .unwrap_or_revert_with(Error::VotingDoesNotExist);
        let all_voters = self.voting.all_voters(voting_id, VotingType::Formal);
        let (partial_supply, balances) = self.reputation_token().partial_balances(all_voters);
        for (address, balance) in balances.balances {
            let amount = to_redistribute * balance / partial_supply;
            self.withdraw(address, amount);
        }
    }

    fn return_job_poster_payment_and_dos_fee(&mut self, job: &Job) {
        let job_offer = self.job_offers.get_or_revert(&job.job_offer_id());
        self.withdraw(job.poster(), job.payment() + job_offer.dos_fee);
    }

    fn return_job_poster_dos_fee(&mut self, job: &Job) {
        let job_offer = self.job_offers.get_or_revert(&job.job_offer_id());
        self.withdraw(job.poster(), job_offer.dos_fee);
    }

    fn return_job_offer_poster_dos_fee(&mut self, job_offer_id: &JobOfferId) {
        let job_offer = self.job_offers.get_or_revert(job_offer_id);
        self.withdraw(job_offer.job_poster, job_offer.dos_fee);
    }

    fn return_external_worker_cspr_stake(&mut self, job: &Job) {
        self.withdraw(job.worker(), job.external_worker_cspr_stake());
    }

    fn mint_and_redistribute_reputation_for_internal_worker(&mut self, job: &Job) {
        let configuration = self.get_job_offer_configuration(job);
        
        let reputation_to_mint = configuration.apply_reputation_conversion_rate_to(job.payment());
        let reputation_to_redistribute = configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        ReputationContractCaller::at(self.voting.reputation_token_address()).mint(
            job.worker(),
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(job, reputation_to_redistribute);
    }

    fn mint_and_redistribute_reputation_for_external_worker(&mut self, job: &Job) {
        let configuration = self.get_job_offer_configuration(job);
        let payment_reputation_to_mint = configuration.apply_reputation_conversion_rate_to(job.payment());
        let total = configuration.apply_default_policing_rate_to(payment_reputation_to_mint);
        self.mint_reputation_for_voters(job, total);
    }

    fn mint_reputation_for_voters(&mut self, job: &Job, amount: U512) {
        let voting = self
            .voting
            .get_voting(job.voting_id().unwrap_or_revert())
            .unwrap_or_revert();

        for i in 0..self.voting.voters().len((voting.voting_id(), VotingType::Formal)) {
            let ballot = self.voting.get_ballot_at(voting.voting_id(), VotingType::Formal, i);
            if ballot.unbounded {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bounded_stake();
            ReputationContractCaller::at(self.voting.reputation_token_address())
                .mint(ballot.voter, to_transfer);
        }
    }

    fn reputation_token(&self) -> ReputationContractCaller {
        ReputationContractCaller::at(self.voting.reputation_token_address())
    }

    fn va_token(&self) -> VaNftContractCaller {
        VaNftContractCaller::at(self.voting.va_token_address())
    }

    fn is_va(&self, address: Address) -> bool {
        !self.va_token().balance_of(address).is_zero()
    }

    fn unstake_not_picked(&mut self, job_offer_id: JobOfferId, bid_id: BidId) {
        let bids_amount = self.job_offers_bids.len(job_offer_id);
        for i in 0..bids_amount {
            let unstake_bid_id = self
                .job_offers_bids
                .get(job_offer_id, i)
                .unwrap_or_revert_with(Error::BidNotFound);
            let mut bid = self
                .get_bid(unstake_bid_id)
                .unwrap_or_revert_with(Error::BidNotFound);
            if unstake_bid_id != bid_id && bid.status == BidStatus::Created {
                self.reputation_token()
                    .unstake_bid(bid.worker, unstake_bid_id);
                bid.reject();
                self.bids.set(&unstake_bid_id, bid);
            }
        }
    }

    fn create_formal_voting(
        &mut self,
        voting_id: VotingId,
    ) {
        let voting = self
            .voting
            .get_voting(voting_id)
            .unwrap_or_revert_with(Error::VotingDoesNotExist);
        if voting.voting_configuration().informal_stake_reputation() {
            self.voting.unstake_all_reputation(voting_id, VotingType::Informal);
        }
        self.voting
            .recast_creators_ballot_from_informal_to_formal(voting_id);
    }

    fn burn_external_worker_reputation(&self, job: &Job) {
        let config = self.get_job_offer_configuration(job);

        let stake = config.apply_reputation_conversion_rate_to(job.external_worker_cspr_stake());
        self.reputation_token().burn(job.worker(), stake);
    }

    fn get_job_offer_configuration(&self, job: &Job) -> Configuration {
        let job_offer = self.job_offers.get(&job.job_offer_id()).unwrap_or_revert();
        job_offer.configuration
    }
}

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

use crate::{
    escrow::{
        bid::{Bid, BidStatus},
        job_offer::{JobOffer, JobOfferStatus},
        types::{JobId, JobOfferId},
    },
    voting::voting_state_machine::VotingType,
};

#[cfg(feature = "test-support")]
impl BidEscrowContractTest {
    pub fn pick_bid_with_cspr_amount(
        &mut self,
        job_offer_id: u32,
        bid_id: u32,
        cspr_amount: U512,
    ) -> Result<(), Error> {
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
        )
    }

    pub fn post_job_offer_with_cspr_amount(
        &mut self,
        expected_timeframe: BlockTime,
        budget: U512,
        cspr_amount: U512,
    ) -> Result<(), Error> {
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
        )
    }

    pub fn submit_bid_with_cspr_amount(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U512,
        onboard: bool,
        cspr_amount: U512,
    ) -> Result<(), Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "submit_bid.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "job_offer_id" => job_offer_id,
                "time" => time,
                "payment" => payment,
                "reputation_stake" => reputation_stake,
                "onboard" => onboard,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }
}
