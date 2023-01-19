use std::borrow::Borrow;

use casper_dao_utils::{
    casper_contract::{contract_api::runtime::revert, unwrap_or_revert::UnwrapOrRevert},
    casper_dao_macros::Instance,
    casper_env::{caller, get_block_time},
    cspr,
    Address,
    DocumentHash,
    Error,
};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::{
    bid_escrow::{
        bid::{Bid, ReclaimBidRequest},
        job::{Job, ReclaimJobRequest, SubmitJobProofRequest, WorkerType},
        storage::{BidStorage, JobStorage},
        types::JobId,
    },
    refs::{ContractRefs, ContractRefsWithKycStorage},
    voting::{
        submodules::{KycInfo, OnboardingInfo},
        voting_state_machine::{VotingResult, VotingType},
        Choice,
        VotingEngine,
        VotingId,
    },
    reputation::ReputationContractInterface,
    va_nft::VaNftContractInterface,
};

#[derive(Instance)]
pub struct JobEngine {
    #[scoped = "contract"]
    job_storage: JobStorage,
    #[scoped = "contract"]
    bid_storage: BidStorage,
    #[scoped = "contract"]
    refs: ContractRefsWithKycStorage,
    #[scoped = "contract"]
    pub voting: VotingEngine,
    #[scoped = "contract"]
    onboarding: OnboardingInfo,
    #[scoped = "contract"]
    kyc: KycInfo,
}

impl JobEngine {
    delegate! {
        to self.job_storage {
            pub fn jobs_count(&self) -> u32;
            pub fn get_job(&self, job_id: JobId) -> Option<Job>;
        }
    }

    pub fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash) {
        let mut job = self.job_storage.get_job_or_revert(job_id);
        let job_offer = self.bid_storage.get_job_offer_or_revert(job.job_offer_id());
        let mut voting_configuration = job_offer.configuration().clone();
        let bid = self.bid_storage.get_bid_or_revert(job.bid_id());
        let worker = caller();

        let submit_proof_request = SubmitJobProofRequest { proof };

        job.submit_proof(submit_proof_request);
        // TODO: Emit event.

        if job_offer.configuration.informal_stake_reputation() && !job.stake().is_zero() {
            let bid = self.bid_storage.get_bid(job.bid_id()).unwrap();
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

        let is_unbound = job.worker_type() != &WorkerType::Internal;
        if is_unbound && bid.onboard {
            voting_configuration.bind_ballot_for_successful_voting(job.worker());
        }

        let voting_info = self
            .voting
            .create_voting(worker, U512::zero(), voting_configuration);

        self.job_storage
            .store_job_for_voting(voting_info.voting_id, job_id);

        // TODO: Do it without reloading voting.
        let mut voting = self.voting.get_voting_or_revert(voting_info.voting_id);

        self.voting.cast_ballot(
            worker,
            voting_info.voting_id,
            Choice::InFavor,
            stake,
            is_unbound,
            &mut voting,
        );

        job.set_voting_id(voting_info.voting_id);

        self.job_storage.store_job(job);
        self.voting.set_voting(voting);
    }

    pub fn submit_job_proof_during_grace_period(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    ) {
        let cspr_stake = purse.map(cspr::deposit);
        let new_worker = caller();
        let caller = new_worker;
        let block_time = get_block_time();

        let mut old_job: Job = self.job_storage.get_job_or_revert(job_id);
        let mut old_bid = self
            .bid_storage
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

        if self.onboarding.is_onboarded(&old_bid.worker) {
            self.slash_worker(&old_job);
        }

        let reclaim_bid_request = ReclaimBidRequest {
            new_bid_id: self.bid_storage.next_bid_id(),
            caller,
            cspr_stake,
            reputation_stake,
            new_worker,
            new_worker_va: self.onboarding.is_onboarded(&new_worker),
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
        self.bid_storage.store_bid(old_bid);
        self.job_storage.store_job(new_job);
        self.bid_storage.store_bid(new_bid);

        // continue as normal
        self.submit_job_proof(new_job_id, proof);
    }

    pub fn cancel_job(&mut self, job_id: JobId) {
        let mut job = self.job_storage.get_job_or_revert(job_id);
        let caller = caller();

        if let Err(e) = job.validate_cancel(get_block_time()) {
            revert(e);
        }

        self.return_job_poster_payment_and_dos_fee(&job);

        let bid = self.bid_storage.get_bid(job.bid_id()).unwrap_or_revert();

        // redistribute cspr stake
        if let Some(cspr_stake) = bid.cspr_stake {
            let left = self.redistribute_to_governance(&job, cspr_stake);
            self.redistribute_cspr_to_all_vas(left);
        }

        // burn reputation stake
        self.burn_reputation_stake(&bid);

        // slash worker
        if self.onboarding.is_onboarded(&bid.worker) {
            self.slash_worker(&job);
        }

        job.cancel(caller).unwrap_or_else(|e| revert(e));
        self.job_storage.store_job(job);
    }

    pub fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: VotingType,
        choice: Choice,
        stake: U512,
    ) {
        let caller = caller();
        let job = self.job_storage.get_job_by_voting_id(voting_id);

        if caller == job.poster() || caller == job.worker() {
            revert(Error::CannotVoteOnOwnJob);
        }
        self.voting
            .vote(caller, voting_id, voting_type, choice, stake);
    }

    pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) {
        let job = self.job_storage.get_job_by_voting_id(voting_id);
        let job_offer = self.bid_storage.get_job_offer_or_revert(job.job_offer_id());
        let voting_summary = self.voting.finish_voting(voting_id, voting_type);
        match voting_summary.voting_type() {
            VotingType::Informal => match voting_summary.result() {
                VotingResult::InFavor | VotingResult::Against => {
                    if !job_offer.configuration.informal_stake_reputation() {
                        let bid = self.bid_storage.get_bid(job.bid_id()).unwrap_or_revert();
                        self.refs
                            .reputation_token()
                            .unstake_bid(bid.borrow().into());
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
                    VotingResult::InFavor => match job.worker_type() {
                        WorkerType::Internal => {
                            self.mint_and_redistribute_reputation_for_internal_worker(&job);
                            self.redistribute_cspr_internal_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                        }
                        WorkerType::ExternalToVA => {
                            // Make user VA.
                            self.refs.va_token().mint(job.worker());

                            self.return_external_worker_cspr_stake(&job);
                            self.burn_external_worker_reputation(&job);
                            self.mint_and_redistribute_reputation_for_internal_worker(&job);
                            self.redistribute_cspr_internal_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                        }
                        WorkerType::External => {
                            self.mint_and_redistribute_reputation_for_external_worker(&job);
                            self.redistribute_cspr_external_worker(&job);
                            self.return_job_poster_dos_fee(&job);
                            self.return_external_worker_cspr_stake(&job);
                        }
                    },
                    VotingResult::Against => match job.worker_type() {
                        WorkerType::Internal => {
                            self.return_job_poster_payment_and_dos_fee(&job);
                            self.slash_worker(&job);
                        }
                        WorkerType::ExternalToVA | WorkerType::External => {
                            self.return_job_poster_payment_and_dos_fee(&job);
                            self.redistribute_cspr_external_worker_failed(&job);
                        }
                    },
                    VotingResult::QuorumNotReached => {
                        self.return_job_poster_payment_and_dos_fee(&job);
                        self.return_external_worker_cspr_stake(&job);
                    }
                    VotingResult::Canceled => revert(Error::VotingAlreadyCanceled),
                }
            }
        }

        self.job_storage.store_job(job);
    }
}

impl JobEngine {
    fn redistribute_to_governance(&mut self, job: &Job, payment: U512) -> U512 {
        let configuration = self.bid_storage.get_job_offer_configuration(job);

        let governance_wallet: Address = configuration.bid_escrow_wallet_address();
        let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(payment);
        cspr::withdraw(governance_wallet, governance_wallet_payment);

        payment - governance_wallet_payment
    }

    fn redistribute_cspr_to_all_vas(&mut self, to_redistribute: U512) {
        let all_balances = self.refs.reputation_token().all_balances();
        let total_supply = all_balances.total_supply();

        for (address, balance) in all_balances.balances() {
            let amount = to_redistribute * balance / total_supply;
            cspr::withdraw(*address, amount);
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

    fn slash_worker(&self, job: &Job) {
        let config = self.bid_storage.get_job_offer_configuration(job);
        let worker_balance = self.refs.reputation_token().balance_of(job.worker());
        let amount_to_burn = config.apply_default_reputation_slash_to(worker_balance);
        self.refs
            .reputation_token()
            .burn(job.worker(), amount_to_burn);
    }

    fn return_job_poster_payment_and_dos_fee(&mut self, job: &Job) {
        let job_offer = self.bid_storage.get_job_offer_or_revert(job.job_offer_id());
        cspr::withdraw(job.poster(), job.payment() + job_offer.dos_fee);
    }

    fn return_external_worker_cspr_stake(&mut self, job: &Job) {
        cspr::withdraw(job.worker(), job.external_worker_cspr_stake());
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
            if ballot.unbound || ballot.canceled {
                continue;
            }
            let to_transfer = ballot.stake * amount / voting.total_bound_stake();
            self.refs.reputation_token().mint(ballot.voter, to_transfer);
        }
    }

    fn burn_external_worker_reputation(&self, job: &Job) {
        let config = self.bid_storage.get_job_offer_configuration(job);

        let stake = config.apply_reputation_conversion_rate_to(job.external_worker_cspr_stake());
        self.refs.reputation_token().burn(job.worker(), stake);
    }

    fn redistribute_cspr_internal_worker(&mut self, job: &Job) {
        let to_redistribute = self.redistribute_to_governance(job, job.payment());
        let redistribute_to_all_vas = self
            .bid_storage
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
        let config = self.bid_storage.get_job_offer_configuration(job);
        let to_redistribute = config.apply_default_policing_rate_to(total_left);
        let to_worker = total_left - to_redistribute;

        // For External Worker
        cspr::withdraw(job.worker(), to_worker);

        let redistribute_to_all_vas = self
            .bid_storage
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
            cspr::withdraw(*address, amount);
        }
    }

    fn return_job_poster_dos_fee(&mut self, job: &Job) {
        let job_offer = self.bid_storage.get_job_offer_or_revert(job.job_offer_id());
        cspr::withdraw(job.poster(), job_offer.dos_fee);
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
            cspr::withdraw(*address, amount);
        }
    }
}
