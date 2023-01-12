use std::{borrow::Borrow, rc::Rc};

use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::{contract_api::system::get_purse_balance, unwrap_or_revert::UnwrapOrRevert},
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, get_block_time, revert},
    transfer,
    Address,
    BlockTime,
    DocumentHash,
    Error,
};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::{
    escrow::{
        bid::{Bid, BidStatus, ShortenedBid},
        job::{Job, WorkerType},
        job_offer::{JobOffer, JobOfferStatus, PostJobOfferRequest},
        storage::JobStorage,
        types::{BidId, JobId, JobOfferId},
    },
    refs::{ContractRefs, ContractRefsWithKycStorage},
    voting::{
        kyc_info::KycInfo,
        onboarding_info::OnboardingInfo,
        voting_state_machine::{VotingResult, VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingEngine,
        VotingId,
    },
    Configuration,
    ConfigurationBuilder,
    ReputationContractInterface,
    VaNftContractInterface,
};

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsWithKycStorage`] by writing addresses of [`Variable Repository`](crate::VariableRepositoryContract), 
    /// [`Reputation Token`](crate::ReputationContract), [`VA Token`](crate::VaNftContract), [`KYC Token`](crate::KycNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    fn init(
        &mut self,
        variable_repository: Address,
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
    fn submit_job_proof_during_grace_period(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    );
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
    fn finish_voting(&mut self, voting_id: VotingId);
    /// Returns the address of [Variable Repository](crate::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::ReputationContract) contract.
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine)
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
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

    fn cancel_job_offer(&mut self, job_offer_id: JobOfferId);

    fn cancel_job(&mut self, job_id: JobId);

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
    refs: ContractRefsWithKycStorage,
    voting: VotingEngine,
    kyc: KycInfo,
    onboarding_info: OnboardingInfo,
    access_control: AccessControl,
    job_storage: JobStorage,
}

impl BidEscrowContractInterface for BidEscrowContract {
    delegate! {
        to self.voting {
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

        to self.job_storage {
            fn job_offers_count(&self) -> u32;
            fn jobs_count(&self) -> u32;
            fn bids_count(&self) -> u32;
            fn get_job(&self, job_id: JobId) -> Option<Job>;
            fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;
            fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
        }

        to self.refs {
            fn variable_repository_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.refs
            .init(variable_repository, reputation_token, va_token, kyc_token);
        self.access_control.init(caller());
    }

    fn post_job_offer(&mut self, expected_timeframe: BlockTime, max_budget: U512, purse: URef) {
        let caller = caller();
        let configuration = self.configuration();

        let request = PostJobOfferRequest {
            job_offer_id: self.job_storage.next_job_offer_id(),
            job_poster_kyced: self.kyc.is_kycd(&caller),
            job_poster: caller,
            max_budget,
            expected_timeframe,
            dos_fee: transfer::deposit_cspr(purse),
            start_time: get_block_time(),
            configuration,
        };

        let job_offer = JobOffer::new(request);
        self.job_storage.store_job_offer(job_offer);
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
        let job_offer: JobOffer = self.job_storage.get_job_offer_or_revert(job_offer_id);
        let bid_id = self.job_storage.next_bid_id();
        let block_time = get_block_time();

        let cspr_stake =
            self.stake_cspr_or_reputation_for_bid(reputation_stake, purse, worker, bid_id);

        let submit_bid_request = SubmitBidRequest {
            bid_id,
            timestamp: block_time,
            job_offer_id,
            proposed_timeframe: time,
            proposed_payment: payment,
            reputation_stake,
            cspr_stake,
            onboard,
            worker,
            worker_kyced: self.kyc.is_kycd(&worker),
            worker_is_va: self.onboarding_info.is_onboarded(&worker),
            job_poster: job_offer.job_poster,
            max_budget: job_offer.max_budget,
            auction_state: job_offer.auction_state(block_time),
            va_can_bid_on_public_auction: job_offer.configuration.va_can_bid_on_public_auction(),
        };

        let bid = Bid::new(submit_bid_request);

        self.job_storage.store_bid(bid);
        self.job_storage.store_bid_id(job_offer_id, bid_id);
        // TODO: Implement Event
        // BidCreated::new(&bid).emit();
    }

    fn cancel_bid(&mut self, bid_id: BidId) {
        let caller = caller();
        let mut bid = self.job_storage.get_bid_or_revert(bid_id);
        let job_offer = self.job_storage.get_job_offer_or_revert(bid.job_offer_id);

        let cancel_bid_request = CancelBidRequest {
            caller,
            job_offer_status: job_offer.status,
            block_time: get_block_time(),
            va_bid_acceptance_timeout: job_offer.configuration.va_bid_acceptance_timeout(),
        };

        bid.cancel(cancel_bid_request);

        self.unstake_cspr_or_reputation_for_bid(&bid);

        // TODO: Implement Event
        self.job_storage.store_bid(bid);
    }

    fn pick_bid(&mut self, job_offer_id: u32, bid_id: u32, purse: URef) {
        let mut job_offer = self.job_storage.get_job_offer_or_revert(job_offer_id);
        let mut bid = self.job_storage.get_bid_or_revert(bid_id);
        let job_id = self.job_storage.next_job_id();

        self.unstake_not_picked(job_offer_id, bid_id);

        let pick_bid_request = PickBidRequest {
            job_id,
            job_offer_id,
            bid_id,
            caller: caller(),
            poster: job_offer.job_poster,
            worker: bid.worker,
            is_worker_va: self.is_va(bid.worker),
            onboard: bid.onboard,
            block_time: get_block_time(),
            timeframe: bid.proposed_timeframe,
            payment: bid.proposed_payment,
            transferred_cspr: transfer::deposit_cspr(purse),
            stake: bid.reputation_stake,
            external_worker_cspr_stake: bid.cspr_stake.unwrap_or_default(),
        };

        let job = Job::new(&pick_bid_request);

        bid.picked(&pick_bid_request);

        job_offer.in_progress(&pick_bid_request);

        self.job_storage.store_job(job);
        self.job_storage.store_bid(bid);
        self.job_storage
            .store_active_job_offer_id(&job_offer.job_poster, job_offer_id);
        self.job_storage.store_job_offer(job_offer);
        // TODO: Emit event.
    }

    fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash) {
        let mut job = self.job_storage.get_job_or_revert(job_id);
        let job_offer = self.job_storage.get_job_offer_or_revert(job.job_offer_id());
        let voting_configuration = job_offer.configuration();

        let worker = caller();

        let submit_proof_request = SubmitJobProofRequest { proof };

        job.submit_proof(submit_proof_request);
        // TODO: Emit event.

        if job_offer.configuration.informal_stake_reputation() && !job.stake().is_zero() {
            let bid = self.job_storage.get_bid(job.bid_id()).unwrap();
            self.refs
                .reputation_token()
                .unstake_bid(bid.borrow().into());
        }

        let stake = if job.external_worker_cspr_stake().is_zero() {
            job.stake()
        } else {
            voting_configuration
                .apply_reputation_conversion_rate_to(job.external_worker_cspr_stake())
        };

        let voting_info =
            self.voting
                .create_voting(worker, U512::zero(), voting_configuration.clone());

        self.job_storage
            .store_job_for_voting(voting_info.voting_id, job_id);

        // TODO: Do it without reloading voting.
        let mut voting = self.voting.get_voting_or_revert(voting_info.voting_id);

        let is_unbounded = job.worker_type() != &WorkerType::Internal;
        self.voting.cast_ballot(
            worker,
            voting_info.voting_id,
            Choice::InFavor,
            stake,
            is_unbounded,
            &mut voting,
        );

        job.set_voting_id(voting_info.voting_id);

        self.job_storage.store_job(job);
        self.voting.set_voting(voting);
    }

    fn submit_job_proof_during_grace_period(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    ) {
        let cspr_stake = purse.map(transfer::deposit_cspr);
        let new_worker = caller();
        let caller = new_worker;
        let block_time = get_block_time();

        let mut old_job: Job = self.job_storage.get_job_or_revert(job_id);
        let mut old_bid = self
            .job_storage
            .get_bid(old_job.bid_id())
            .unwrap_or_revert_with(Error::BidNotFound);

        // redistribute original cspr stake
        if let Some(cspr_stake) = old_bid.cspr_stake {
            let left = self.redistribute_to_governance(&old_job, cspr_stake);
            self.redistribute_cspr_to_all_vas(left);
        }

        // burn original reputation stake
        self.burn_reputation_stake(&old_bid);

        // slash original worker
        if self.is_va(old_bid.worker) {
            self.slash_worker(&old_job);
        }

        let reclaim_bid_request = ReclaimBidRequest {
            new_bid_id: self.job_storage.next_bid_id(),
            caller,
            cspr_stake,
            reputation_stake,
            new_worker,
            new_worker_va: self.is_va(new_worker),
            new_worker_kyced: self.kyc.is_kycd(&new_worker),
            job_poster: old_job.poster(),
            onboard,
            block_time,
            job_status: old_job.status(),
            job_finish_time: old_job.finish_time(),
        };

        let new_bid = old_bid.reclaim(&reclaim_bid_request);

        let reclaim_job_request = ReclaimJobRequest {
            new_job_id: self.job_storage.next_job_id(),
            new_bid_id: new_bid.bid_id(),
            proposed_timeframe: new_bid.proposed_timeframe,
            worker: new_bid.worker,
            cspr_stake,
            reputation_stake,
            onboard,
            block_time,
        };

        let new_job = old_job.reclaim(reclaim_job_request);

        let new_job_id = new_job.job_id();

        // Stake new bid
        if new_bid.reputation_stake > U512::zero() {
            self.refs
                .reputation_token()
                .stake_bid(new_bid.borrow().into());
        }

        self.job_storage.store_job(old_job);
        self.job_storage.store_bid(old_bid);
        self.job_storage.store_job(new_job);
        self.job_storage.store_bid(new_bid);

        // continue as normal
        self.submit_job_proof(new_job_id, proof);
    }

    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512) {
        let caller = caller();
        let job = self.job_storage.get_job_by_voting_id(voting_id);

        if caller == job.poster() || caller == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting
            .vote(caller, voting_id, voting_type, choice, stake);
    }

    fn finish_voting(&mut self, voting_id: VotingId) {
        let job = self.job_storage.get_job_by_voting_id(voting_id);
        let job_offer = self.job_storage.get_job_offer_or_revert(job.job_offer_id());
        let voting_summary = self
            .voting
            .finish_voting_without_token_redistribution(voting_id);

        match voting_summary.voting_type() {
            VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor => {
                    if !job_offer.configuration.informal_stake_reputation() {
                        let bid = self.job_storage.get_bid(job.bid_id()).unwrap_or_revert();
                        self.refs
                            .reputation_token()
                            .unstake_bid(bid.borrow().into());
                    }
                    self.create_formal_voting(voting_id);
                }
                VotingResult::Against => {
                    if !job_offer.configuration.informal_stake_reputation() {
                        let bid = self.job_storage.get_bid(job.bid_id()).unwrap_or_revert();
                        self.refs
                            .reputation_token()
                            .unstake_bid(bid.borrow().into());
                    }
                    self.create_formal_voting(voting_id);
                }
                VotingResult::QuorumNotReached => {
                    if job_offer.configuration.informal_stake_reputation() {
                        self.voting
                            .return_yes_voters_rep(voting_id, VotingType::Informal);
                        self.voting
                            .return_no_voters_rep(voting_id, VotingType::Informal);
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
                            self.voting
                                .return_yes_voters_rep(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_no_voters(
                                voting_id,
                                VotingType::Formal,
                            );
                            self.mint_and_redistribute_reputation_for_internal_worker(&job);
                            self.redistribute_cspr_internal_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                        }
                        WorkerType::ExternalToVA => {
                            // Make user VA.
                            self.refs.va_token().mint(job.worker());

                            self.return_external_worker_cspr_stake(&job);
                            // Bound ballot for worker.
                            self.voting
                                .bound_ballot(voting_id, job.worker(), VotingType::Formal);

                            self.voting
                                .return_yes_voters_rep(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_no_voters(
                                voting_id,
                                VotingType::Formal,
                            );
                            self.mint_and_redistribute_reputation_for_internal_worker(&job);
                            self.burn_external_worker_reputation(&job);
                            self.redistribute_cspr_internal_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                        }
                        WorkerType::External => {
                            self.voting
                                .return_yes_voters_rep(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_no_voters(
                                voting_id,
                                VotingType::Formal,
                            );
                            self.mint_and_redistribute_reputation_for_external_worker(&job);
                            self.redistribute_cspr_external_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                            self.return_external_worker_cspr_stake(&job);
                        }
                    },
                    VotingResult::Against => match job.worker_type() {
                        WorkerType::Internal => {
                            self.voting
                                .return_no_voters_rep(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_yes_voters(
                                voting_id,
                                VotingType::Formal,
                            );
                            self.return_job_poster_payment_and_dos_fee(&job);
                            self.slash_worker(&job);
                        }
                        WorkerType::ExternalToVA | WorkerType::External => {
                            self.voting
                                .return_no_voters_rep(voting_id, VotingType::Formal);
                            self.voting.redistribute_reputation_of_yes_voters(
                                voting_id,
                                VotingType::Formal,
                            );
                            self.return_job_poster_payment_and_dos_fee(&job);
                            self.redistribute_cspr_external_worker_failed(&job);
                        }
                    },
                    VotingResult::QuorumNotReached => {
                        self.voting
                            .return_yes_voters_rep(voting_id, VotingType::Formal);
                        self.voting
                            .return_no_voters_rep(voting_id, VotingType::Formal);
                        self.return_job_poster_payment_and_dos_fee(&job);
                        self.return_external_worker_cspr_stake(&job);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
            }
        }

        self.job_storage.store_job(job);
    }

    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine> {
        self.voting.get_voting(voting_id)
    }

    // TODO: move somewhere else
    fn get_cspr_balance(&self) -> U512 {
        get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
    }

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.voting.slash_voter(voter, voting_id);
    }

    fn cancel_job_offer(&mut self, job_offer_id: JobOfferId) {
        let mut job_offer = self.job_storage.get_job_offer_or_revert(job_offer_id);
        let cancel_job_offer_request = CancelJobOfferRequest {
            caller: caller(),
            block_time: get_block_time(),
        };

        job_offer.cancel(&cancel_job_offer_request);

        self.cancel_all_bids(job_offer_id);
        self.return_job_offer_poster_dos_fee(job_offer_id);

        self.job_storage.update_job_offer(job_offer_id, job_offer);
    }

    fn cancel_job(&mut self, job_id: JobId) {
        let mut job = self.job_storage.get_job_or_revert(job_id);
        let caller = caller();

        if let Err(e) = job.validate_cancel(get_block_time()) {
            revert(e);
        }

        self.return_job_poster_payment_and_dos_fee(&job);

        let bid = self.job_storage.get_bid(job.bid_id()).unwrap_or_revert();

        // redistribute cspr stake
        if let Some(cspr_stake) = bid.cspr_stake {
            let left = self.redistribute_to_governance(&job, cspr_stake);
            self.redistribute_cspr_to_all_vas(left);
        }

        // burn reputation stake
        self.burn_reputation_stake(&bid);

        // slash worker
        if self.is_va(bid.worker) {
            self.slash_worker(&job);
        }

        job.cancel(caller).unwrap_or_else(|e| revert(e));
        self.job_storage.store_job(job);
    }

    fn slash_all_active_job_offers(&mut self, bidder: Address) {
        self.access_control.ensure_whitelisted();
        // Cancel job offers created by the bidder.
        let job_offer_ids = self.job_storage.clear_active_job_offers_ids(&bidder);
        for job_offer_id in job_offer_ids {
            self.cancel_all_bids(job_offer_id);
            self.return_job_offer_poster_dos_fee(job_offer_id);
        }
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

        bid.cancel_without_validation();

        self.refs
            .reputation_token()
            .unstake_bid(bid.borrow().into());

        // TODO: Implement Event
        self.job_storage.store_bid(bid);
    }

    fn slash_voter(&mut self, _voter: Address, _voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        unimplemented!()
    }
}

impl BidEscrowContract {
    fn slash_worker(&self, job: &Job) {
        let config = self.job_storage.get_job_offer_configuration(job);
        let worker_balance = self.refs.reputation_token().balance_of(job.worker());
        let amount_to_burn = config.apply_default_reputation_slash_to(worker_balance);
        self.refs
            .reputation_token()
            .burn(job.worker(), amount_to_burn);
    }

    fn cancel_all_bids(&mut self, job_offer_id: JobOfferId) {
        let bids_amount = self.job_storage.get_bids_count(job_offer_id);
        let mut bids = Vec::<ShortenedBid>::new();
        for i in 0..bids_amount {
            let mut bid = self.job_storage.get_nth_bid(job_offer_id, i);
            if let Some(cspr) = bid.cspr_stake {
                transfer::withdraw_cspr(bid.worker, cspr);
            }
            bids.push(bid.borrow().into());
            bid.cancel_without_validation();
            self.job_storage.store_bid(bid);
        }
        self.refs.reputation_token().bulk_unstake_bid(bids);
    }

    fn redistribute_cspr_internal_worker(&mut self, job: &Job) {
        let to_redistribute = self.redistribute_to_governance(job, job.payment());
        let redistribute_to_all_vas = self
            .job_storage
            .get_job_offer_or_revert(job.job_offer_id())
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
        let config = self.job_storage.get_job_offer_configuration(job);
        let to_redistribute = config.apply_default_policing_rate_to(total_left);
        let to_worker = total_left - to_redistribute;

        // For External Worker
        transfer::withdraw_cspr(job.worker(), to_worker);

        let redistribute_to_all_vas = self
            .job_storage
            .get_job_offer_or_revert(job.job_offer_id())
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
        let all_balances = self.refs.reputation_token().all_balances();
        let total_supply = all_balances.total_supply();

        for (address, balance) in all_balances.balances() {
            let amount = total_left * balance / total_supply;
            transfer::withdraw_cspr(*address, amount);
        }
    }

    fn redistribute_to_governance(&mut self, job: &Job, payment: U512) -> U512 {
        let configuration = self.job_storage.get_job_offer_configuration(job);

        let governance_wallet: Address = configuration.bid_escrow_wallet_address();
        let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(payment);
        transfer::withdraw_cspr(governance_wallet, governance_wallet_payment);

        payment - governance_wallet_payment
    }

    fn redistribute_cspr_to_all_vas(&mut self, to_redistribute: U512) {
        let all_balances = self.refs.reputation_token().all_balances();
        let total_supply = all_balances.total_supply();

        for (address, balance) in all_balances.balances() {
            let amount = to_redistribute * balance / total_supply;
            transfer::withdraw_cspr(*address, amount);
        }
    }

    fn redistribute_cspr_to_voters(&mut self, job: &Job, to_redistribute: U512) {
        let voting_id = job
            .voting_id()
            .unwrap_or_revert_with(Error::VotingDoesNotExist);
        let all_voters = self.voting.all_voters(voting_id, VotingType::Formal);

        let balances = self.refs.reputation_token().partial_balances(all_voters);
        let partial_supply = balances.total_supply();
        for (address, balance) in balances.balances() {
            let amount = to_redistribute * balance / partial_supply;
            transfer::withdraw_cspr(*address, amount);
        }
    }

    fn return_job_poster_payment_and_dos_fee(&mut self, job: &Job) {
        let job_offer = self.job_storage.get_job_offer_or_revert(job.job_offer_id());
        transfer::withdraw_cspr(job.poster(), job.payment() + job_offer.dos_fee);
    }

    fn return_job_poster_dos_fee(&mut self, job: &Job) {
        let job_offer = self.job_storage.get_job_offer_or_revert(job.job_offer_id());
        transfer::withdraw_cspr(job.poster(), job_offer.dos_fee);
    }

    fn return_job_offer_poster_dos_fee(&mut self, job_offer_id: JobOfferId) {
        let job_offer = self.job_storage.get_job_offer_or_revert(job_offer_id);
        transfer::withdraw_cspr(job_offer.job_poster, job_offer.dos_fee);
    }

    fn return_external_worker_cspr_stake(&mut self, job: &Job) {
        transfer::withdraw_cspr(job.worker(), job.external_worker_cspr_stake());
    }

    fn mint_and_redistribute_reputation_for_internal_worker(&mut self, job: &Job) {
        let configuration = self.job_storage.get_job_offer_configuration(job);

        let reputation_to_mint = configuration.apply_reputation_conversion_rate_to(job.payment());
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        self.refs.reputation_token().mint(
            job.worker(),
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(job, reputation_to_redistribute);
    }

    fn mint_and_redistribute_reputation_for_external_worker(&mut self, job: &Job) {
        let configuration = self.job_storage.get_job_offer_configuration(job);
        let reputation_to_mint = configuration.apply_reputation_conversion_rate_to(job.payment());
        let reputation_to_redistribute =
            configuration.apply_default_policing_rate_to(reputation_to_mint);

        // Worker
        self.refs.reputation_token().mint_passive(
            job.worker(),
            reputation_to_mint - reputation_to_redistribute,
        );

        // Voters
        self.mint_reputation_for_voters(job, reputation_to_redistribute);
    }

    fn mint_reputation_for_voters(&mut self, job: &Job, amount: U512) {
        let voting = self
            .voting
            .get_voting(job.voting_id().unwrap_or_revert())
            .unwrap_or_revert();

        for i in 0..self
            .voting
            .voters()
            .len((voting.voting_id(), VotingType::Formal))
        {
            let ballot = self
                .voting
                .get_ballot_at(voting.voting_id(), VotingType::Formal, i);
            if ballot.unbounded {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bounded_stake();
            self.refs.reputation_token().mint(ballot.voter, to_transfer);
        }
    }

    fn is_va(&self, address: Address) -> bool {
        !self.refs.va_token().balance_of(address).is_zero()
    }

    fn unstake_not_picked(&mut self, job_offer_id: JobOfferId, bid_id: BidId) {
        let bids_amount = self.job_storage.get_bids_count(job_offer_id);
        let mut bids = Vec::<ShortenedBid>::new();
        for i in 0..bids_amount {
            let mut bid = self.job_storage.get_nth_bid(job_offer_id, i);

            if bid.bid_id != bid_id && bid.status == BidStatus::Created {
                if let Some(cspr) = bid.cspr_stake {
                    transfer::withdraw_cspr(bid.worker, cspr);
                }
                bids.push(bid.borrow().into());
                bid.reject_without_validation();
                self.job_storage.store_bid(bid);
            }
        }
        self.refs.reputation_token().bulk_unstake_bid(bids);
    }

    fn create_formal_voting(&mut self, voting_id: VotingId) {
        let mut voting = self.voting.get_voting_or_revert(voting_id);
        if voting.voting_configuration().informal_stake_reputation() {
            self.voting
                .unstake_all_reputation(voting_id, VotingType::Informal);
        }
        self.voting
            .recast_creators_ballot_from_informal_to_formal(&mut voting);
        self.voting.set_voting(voting);
    }

    fn burn_external_worker_reputation(&self, job: &Job) {
        let config = self.job_storage.get_job_offer_configuration(job);

        let stake = config.apply_reputation_conversion_rate_to(job.external_worker_cspr_stake());
        self.refs.reputation_token().burn(job.worker(), stake);
    }

    /// Builds Configuration for a Bid Escrow Entities
    fn configuration(&self) -> Rc<Configuration> {
        Rc::new(
            ConfigurationBuilder::new(&self.refs)
                .is_bid_escrow(true)
                .only_va_can_create(false)
                .build(),
        )
    }

    fn stake_cspr_or_reputation_for_bid(
        &mut self,
        reputation_stake: U512,
        purse: Option<URef>,
        worker: Address,
        bid_id: BidId,
    ) -> Option<U512> {
        match purse {
            None => {
                let bid = ShortenedBid::new(bid_id, reputation_stake, worker);
                self.refs.reputation_token().stake_bid(bid);
                None
            }
            Some(purse) => {
                let cspr_stake = transfer::deposit_cspr(purse);
                Some(cspr_stake)
            }
        }
    }

    fn unstake_cspr_or_reputation_for_bid(&mut self, bid: &Bid) {
        match bid.cspr_stake {
            None => {
                self.refs
                    .reputation_token()
                    .unstake_bid(bid.borrow().into());
            }
            Some(cspr_stake) => {
                transfer::withdraw_cspr(bid.worker, cspr_stake);
            }
        }
    }

    fn burn_reputation_stake(&mut self, bid: &Bid) {
        if bid.reputation_stake > U512::zero() {
            self.refs
                .reputation_token()
                .unstake_bid(bid.borrow().into());
            self.refs
                .reputation_token()
                .burn(bid.worker, bid.reputation_stake);
        }
    }
}

#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;

use crate::escrow::{
    bid::{CancelBidRequest, ReclaimBidRequest, SubmitBidRequest},
    job::{PickBidRequest, ReclaimJobRequest, SubmitJobProofRequest},
    job_offer::CancelJobOfferRequest,
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

    pub fn submit_job_proof_during_grace_period_with_cspr_amount(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: U512,
        onboard: bool,
        cspr_amount: U512,
    ) -> Result<(), Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "submit_job_proof_during_grace_period.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "job_id" => job_id,
                "proof" => proof,
                "reputation_stake" => reputation_stake,
                "onboard" => onboard,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }
}
