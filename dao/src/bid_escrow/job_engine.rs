//! Job Engine module.
use crate::bid_escrow::bid::{Bid, ReclaimBidRequest};
use crate::bid_escrow::events::{
    BidEscrowVotingCreated, JobCancelled, JobDone, JobRejected, JobSubmitted, TransferReason,
};
use crate::bid_escrow::job::{Job, ReclaimJobRequest, SubmitJobProofRequest, WorkerType};
use crate::bid_escrow::storage::{BidStorage, JobStorage};
use crate::bid_escrow::types::JobId;
use crate::configuration::Configuration;
use crate::modules::refs::ContractRefs;
use crate::utils::types::DocumentHash;
use crate::utils::{withdraw, Error};
use crate::voting::ballot::Choice;
use crate::voting::cspr_redistribution::{
    redistribute_cspr_to_all_vas, redistribute_to_governance,
};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::voting_state_machine::{VotingResult, VotingSummary, VotingType};
use crate::voting::voting_engine::VotingEngine;
use odra::contract_env::{attached_value, caller, get_block_time, revert};
use odra::prelude::{collections::BTreeMap, vec, vec::Vec};
use odra::types::Address;
use odra::types::{event::OdraEvent, Balance};
use odra::UnwrapOrRevert;

/// Manages Jobs lifecycle.
#[odra::module(events = [JobSubmitted, JobRejected, JobCancelled, JobDone, BidEscrowVotingCreated])]
pub struct JobEngine {
    job_storage: JobStorage,
    bid_storage: BidStorage,
    refs: ContractRefs,
    #[odra(using = "refs")]
    voting_engine: VotingEngine,
}

#[odra::module]
impl JobEngine {
    delegate! {
        to self.job_storage {
            /// Returns the total number of jobs.
            pub fn jobs_count(&self) -> u32;
            /// Gets the [job](Job) with a given id or `None`.
            pub fn get_job(&self, job_id: JobId) -> Option<Job>;
        }
    }

    /// Validates the correctness of proof submission.
    /// If the submission is correct, the [`Job Storage`](JobStorage) is updated, the Voting process starts.
    ///
    /// # Errors
    /// If a proof has been submitted before, reverts with [`Error::JobAlreadySubmitted`].
    pub fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash) {
        let mut job = self.job_storage.get_job_or_revert(job_id);
        let job_offer = self
            .bid_storage
            .get_job_offer_or_revert(&job.job_offer_id());
        let mut voting_configuration = job_offer.configuration().clone();
        let bid = self.bid_storage.get_bid_or_revert(&job.bid_id());
        let worker = caller();

        job.submit_proof(SubmitJobProofRequest {
            proof,
            caller: worker,
        });

        JobSubmitted::new(&job).emit();

        self.unstake_reputation_for_use_in_voting(
            &bid,
            &job,
            voting_configuration.informal_stake_reputation(),
        );

        let stake_for_voting =
            Self::calculate_stake_for_voting(&mut job, &mut voting_configuration);

        if job.is_unbound() && bid.onboard {
            voting_configuration.bind_ballot_for_successful_voting(job.worker());
        }

        let (voting_info, mut voting) =
            self.voting_engine
                .create_voting(worker, Balance::zero(), voting_configuration.clone());

        BidEscrowVotingCreated::new(&job, worker, voting_info.voting_id, &voting_configuration)
            .emit();

        job.set_voting_id(voting_info.voting_id);

        self.voting_engine.cast_ballot(
            worker,
            Choice::InFavor,
            stake_for_voting,
            job.is_unbound(),
            &mut voting,
            &voting_configuration,
        );

        self.job_storage.store_job(job);
        self.voting_engine.set_voting(voting);
        self.job_storage
            .store_job_for_voting(voting_info.voting_id, job_id);
    }

    /// Updates the old [Bid] and [Job], the job is assigned to a new `Worker`. The rest goes the same
    /// as regular proof submission. See [submit_job_proof()][Self::submit_job_proof].
    /// The old `Worker` who didn't submit the proof in time, is getting slashed.
    ///
    /// See the Grace Period section in the module [description](crate::bid_escrow).
    pub fn submit_job_proof_during_grace_period(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: Balance,
        onboard: bool,
    ) {
        let cspr_stake = {
            let attached_value = attached_value();
            if attached_value.is_zero() {
                None
            } else {
                Some(attached_value)
            }
        };

        let new_worker = caller();
        let caller = new_worker;
        let block_time = get_block_time();

        let mut old_job: Job = self.job_storage.get_job_or_revert(job_id);
        let mut old_bid = self
            .bid_storage
            .get_bid(&old_job.bid_id())
            .unwrap_or_revert_with(Error::BidNotFound);
        let configuration = self.bid_storage.get_job_offer_configuration(&old_job);

        // redistribute original cspr stake
        if let Some(cspr_stake) = old_bid.cspr_stake {
            let left = redistribute_to_governance(cspr_stake, &configuration);
            redistribute_cspr_to_all_vas(left, &self.refs);
        }

        // burn original reputation stake
        self.burn_reputation_stake(&old_bid);

        // slash original worker
        if !self.refs.va_token().balance_of(&old_bid.worker).is_zero() {
            self.slash_worker(&old_job);
        }

        let reclaim_bid_request = ReclaimBidRequest {
            new_bid_id: self.bid_storage.next_bid_id(),
            caller,
            cspr_stake,
            reputation_stake,
            new_worker,
            new_worker_va: !self.refs.va_token().balance_of(&new_worker).is_zero(),
            new_worker_kyced: !self.refs.kyc_token().balance_of(&new_worker).is_zero(),
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
        if new_bid.reputation_stake > Balance::zero() {
            self.refs
                .reputation_token()
                .stake(new_bid.worker, new_bid.reputation_stake);
        }

        // Update old job and bid.
        self.job_storage.remove_from_active_jobs(old_job.job_id());
        self.job_storage.store_job(old_job);
        self.bid_storage.store_bid(old_bid);

        // Store new job and bid.
        self.job_storage.add_to_active_jobs(new_job.job_id());
        self.job_storage.store_job(new_job);
        self.bid_storage.store_bid(new_bid);

        // continue as normal
        self.submit_job_proof(new_job_id, proof);
    }

    /// Terminates the Voting process and slashes the `Worker`.
    ///
    /// * the bid stake is redistributed along the VAs' and the multisig wallet.
    /// * `DOS Fee` is returned to the `Job Poster`.
    ///
    /// # Error
    /// If the state in which the process cannot be canceled, the execution reverts with
    /// [Error::CannotCancelJob] or [Error::JobCannotBeYetCanceled].
    pub fn cancel_job(&mut self, job_id: JobId) {
        let job = self.job_storage.get_job_or_revert(job_id);
        let caller = caller();
        if let Err(e) = job.validate_cancel(get_block_time(), caller) {
            revert(e);
        }

        let bid = self
            .bid_storage
            .get_bid(&job.bid_id())
            .unwrap_or_revert_with(Error::BidNotFound);

        self.burn_reputation_stake(&bid);

        if !self.refs.va_token().balance_of(&job.worker()).is_zero() {
            self.slash_worker(&job);
        }

        self.raw_cancel_job(job, &bid, caller);
    }

    /// Records vote in [Voting](crate::voting::voting_engine::voting_state_machine::VotingStateMachine).
    ///
    /// # Error
    /// * [`Error::CannotVoteOnOwnJob`].
    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: Balance,
    ) {
        let caller = caller();
        let job = self.job_storage.get_job_by_voting_id(voting_id);

        if caller == job.poster() || caller == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting_engine
            .vote(caller, voting_id, voting_type, choice, stake);
    }

    /// Ends the current voting phase and redistributes funds.
    ///
    /// Interacts with [`Reputation Token Contract`](crate::core_contracts::ReputationContract) to
    /// redistribute reputation.
    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary {
        let job = self.job_storage.get_job_by_voting_id(voting_id);
        let job_offer = self
            .bid_storage
            .get_job_offer_or_revert(&job.job_offer_id());
        let voting_summary = self.voting_engine.finish_voting(voting_id, voting_type);
        match voting_summary.voting_type() {
            VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor | VotingResult::Against => {
                    if !job_offer.configuration.informal_stake_reputation() {
                        let bid = self
                            .bid_storage
                            .get_bid(&job.bid_id())
                            .unwrap_or_revert_with(Error::BidNotFound);
                        self.refs
                            .reputation_token()
                            .unstake(bid.worker, bid.reputation_stake);
                    }
                }
                VotingResult::QuorumNotReached => {
                    self.return_job_poster_payment_and_dos_fee(&job);
                    self.return_external_worker_cspr_stake(&job);
                }
                VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
            },
            VotingType::Formal => {
                match voting_summary.result() {
                    VotingResult::InFavor => {
                        match job.worker_type() {
                            WorkerType::Internal => {
                                self.mint_and_redistribute_reputation_for_internal_worker(&job);
                                self.redistribute_cspr_internal_worker(
                                    &job,
                                    job_offer.configuration(),
                                );
                                self.return_job_poster_dos_fee(&job);
                            }
                            WorkerType::ExternalToVA => {
                                // Make user VA.
                                self.refs.va_token().mint(job.worker());

                                self.return_external_worker_cspr_stake(&job);
                                self.burn_external_worker_reputation(&job);
                                self.mint_and_redistribute_reputation_for_internal_worker(&job);
                                self.redistribute_cspr_internal_worker(
                                    &job,
                                    job_offer.configuration(),
                                );
                                self.return_job_poster_dos_fee(&job);
                            }
                            WorkerType::External => {
                                self.mint_and_redistribute_reputation_for_external_worker(&job);
                                self.redistribute_cspr_external_worker(
                                    &job,
                                    job_offer.configuration(),
                                );
                                self.return_job_poster_dos_fee(&job);
                                self.return_external_worker_cspr_stake(&job);
                            }
                        };
                        JobDone::new(&job, caller()).emit();
                    }
                    VotingResult::Against => {
                        match job.worker_type() {
                            WorkerType::Internal => {
                                self.return_job_poster_payment_and_dos_fee(&job);
                                self.slash_worker(&job);
                            }
                            WorkerType::ExternalToVA | WorkerType::External => {
                                self.return_job_poster_payment_and_dos_fee(&job);
                                self.redistribute_cspr_external_worker_failed(
                                    &job,
                                    job_offer.configuration(),
                                );
                            }
                        }
                        JobRejected::new(&job, caller()).emit();
                    }
                    VotingResult::QuorumNotReached => {
                        self.return_job_poster_payment_and_dos_fee(&job);
                        self.return_external_worker_cspr_stake(&job);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
            }
        }

        self.job_storage.store_job(job);
        voting_summary
    }

    /// Slashes the voter and cancels all his active jobs.
    /// Returns the lists of slashed jobs, canceled votings and affected votings.
    pub fn slash_voter(&mut self, voter: Address) -> (Vec<JobId>, Vec<VotingId>, Vec<VotingId>) {
        let slashed_votings = self.voting_engine.slash_voter(voter);
        let mut slashed_jobs = vec![];
        for job_id in self.job_storage.get_active_jobs() {
            let job = self.job_storage.get_job_or_revert(job_id);
            if job.worker() == voter {
                let bid = self.bid_storage.get_bid_or_revert(&job.bid_id());
                self.raw_cancel_job(job, &bid, caller());
                slashed_jobs.push(job_id);
            }
        }
        (
            slashed_jobs,
            slashed_votings.cancelled_votings,
            slashed_votings.affected_votings,
        )
    }
}

impl JobEngine {
    /// Calculates the stake for voting - either the reputation staked, or the cspr staked converted to reputation
    fn calculate_stake_for_voting(
        job: &mut Job,
        voting_configuration: &mut Configuration,
    ) -> Balance {
        if job.external_worker_cspr_stake().is_zero() {
            job.get_stake()
        } else {
            voting_configuration
                .apply_reputation_conversion_rate_to(job.external_worker_cspr_stake())
        }
    }

    fn burn_reputation_stake(&self, bid: &Bid) {
        if bid.reputation_stake > Balance::zero() {
            self.refs
                .reputation_token()
                .unstake(bid.worker, bid.reputation_stake);
            self.refs
                .reputation_token()
                .burn(bid.worker, bid.reputation_stake);
        }
    }

    /// Unstakes reputation from bid, so it can be used in voting.
    fn unstake_reputation_for_use_in_voting(
        &self,
        bid: &Bid,
        job: &Job,
        informal_stake_reputation: bool,
    ) {
        if informal_stake_reputation && !job.get_stake().is_zero() {
            self.refs
                .reputation_token()
                .unstake(bid.worker, job.get_stake());
        }
    }

    fn slash_worker(&self, job: &Job) {
        let config = self.bid_storage.get_job_offer_configuration(job);
        let worker_balance = self.refs.reputation_token().balance_of(job.worker());
        let amount_to_burn = config.apply_default_reputation_slash_to(worker_balance);
        self.refs
            .reputation_token()
            .burn(job.worker(), amount_to_burn);
    }

    fn return_job_poster_payment_and_dos_fee(&mut self, job: &Job) {
        let job_offer = self
            .bid_storage
            .get_job_offer_or_revert(&job.job_offer_id());
        withdraw(
            &job.poster(),
            job.payment() + job_offer.dos_fee,
            TransferReason::JobPaymentAndDOSFeeReturn,
        );
    }

    fn return_external_worker_cspr_stake(&mut self, job: &Job) {
        withdraw(
            &job.worker(),
            job.external_worker_cspr_stake(),
            TransferReason::BidStakeReturn,
        );
    }

    fn mint_and_redistribute_reputation_for_internal_worker(&mut self, job: &Job) {
        let configuration = self.bid_storage.get_job_offer_configuration(job);

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
        let configuration = self.bid_storage.get_job_offer_configuration(job);
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

    fn mint_reputation_for_voters(&mut self, job: &Job, amount: Balance) {
        let voting = self
            .voting_engine
            .get_voting(
                job.voting_id()
                    .unwrap_or_revert_with(Error::VotingIdNotFound),
            )
            .unwrap_or_revert_with(Error::VotingDoesNotExist);

        let mut mints = BTreeMap::new();

        for i in 0..self
            .voting_engine
            .voters(voting.voting_id(), VotingType::Formal)
            .len()
        {
            let ballot =
                self.voting_engine
                    .get_ballot_at(voting.voting_id(), VotingType::Formal, i);
            if ballot.unbound || ballot.canceled {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bound_stake();
            mints.insert(ballot.voter, to_transfer);
        }

        if !mints.is_empty() {
            self.refs
                .reputation_token()
                .bulk_mint_burn(mints, BTreeMap::new());
        }
    }

    fn burn_external_worker_reputation(&self, job: &Job) {
        let config = self.bid_storage.get_job_offer_configuration(job);

        let stake = config.apply_reputation_conversion_rate_to(job.external_worker_cspr_stake());
        self.refs.reputation_token().burn(job.worker(), stake);
    }

    fn redistribute_cspr_internal_worker(&mut self, job: &Job, configuration: &Configuration) {
        let to_redistribute = redistribute_to_governance(job.payment(), configuration);
        let redistribute_to_all_vas = configuration.distribute_payment_to_non_voters();

        // For VA's
        if redistribute_to_all_vas {
            redistribute_cspr_to_all_vas(to_redistribute, &self.refs);
        } else {
            self.redistribute_cspr_to_voters(job, to_redistribute);
        }
    }

    fn redistribute_cspr_external_worker(&mut self, job: &Job, configuration: &Configuration) {
        let total_left = redistribute_to_governance(job.payment(), configuration);
        let config = self.bid_storage.get_job_offer_configuration(job);
        let to_redistribute = config.apply_default_policing_rate_to(total_left);
        let to_worker = total_left - to_redistribute;

        // For External Worker
        withdraw(&job.worker(), to_worker, TransferReason::Redistribution);

        let redistribute_to_all_vas = configuration.distribute_payment_to_non_voters();

        // For VA's
        if redistribute_to_all_vas {
            redistribute_cspr_to_all_vas(to_redistribute, &self.refs);
        } else {
            self.redistribute_cspr_to_voters(job, to_redistribute);
        }
    }

    fn redistribute_cspr_external_worker_failed(
        &mut self,
        job: &Job,
        configuration: &Configuration,
    ) {
        let total_left =
            redistribute_to_governance(job.external_worker_cspr_stake(), configuration);

        // For VA's
        let all_balances = self.refs.reputation_token().all_balances();
        let total_supply = all_balances.total_supply();

        for (address, balance) in all_balances.balances() {
            let amount = total_left * *balance / total_supply;
            withdraw(address, amount, TransferReason::Redistribution);
        }
    }

    fn return_job_poster_dos_fee(&mut self, job: &Job) {
        let job_offer = self
            .bid_storage
            .get_job_offer_or_revert(&job.job_offer_id());
        withdraw(
            &job.poster(),
            job_offer.dos_fee,
            TransferReason::DOSFeeReturn,
        );
    }

    fn redistribute_cspr_to_voters(&mut self, job: &Job, to_redistribute: Balance) {
        let voting_id = job
            .voting_id()
            .unwrap_or_revert_with(Error::VotingDoesNotExist);
        let all_voters = self.voting_engine.all_voters(voting_id, VotingType::Formal);

        let balances = self.refs.reputation_token().partial_balances(all_voters);
        let partial_supply = balances.total_supply();
        for (address, balance) in balances.balances() {
            let amount = to_redistribute * *balance / partial_supply;
            withdraw(address, amount, TransferReason::Redistribution)
        }
    }

    fn raw_cancel_job(&mut self, mut job: Job, bid: &Bid, caller: Address) {
        let configuration = self.bid_storage.get_job_offer_configuration(&job);

        self.return_job_poster_payment_and_dos_fee(&job);

        // redistribute cspr stake
        if let Some(cspr_stake) = bid.cspr_stake {
            let left = redistribute_to_governance(cspr_stake, &configuration);
            redistribute_cspr_to_all_vas(left, &self.refs);
        }

        job.cancel();
        JobCancelled::new(&job, caller).emit();

        self.job_storage.remove_from_active_jobs(job.job_id());
        self.job_storage.store_job(job);
    }
}
