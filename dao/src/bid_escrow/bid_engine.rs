//! Bid engine module.
//! It is responsible for handling all Bid-related operations.
//! It stores all Bids and Jobs.
extern crate alloc;

use crate::bid_escrow::bid::{Bid, BidStatus, CancelBidRequest, SubmitBidRequest};
#[allow(unused_imports)]
use crate::bid_escrow::events::CSPRTransfer;
use crate::bid_escrow::events::{
    BidCancelled, BidSubmitted, JobCreated, JobOfferCreated, TransferReason,
};
use crate::bid_escrow::job::{Job, PickBidRequest};
use crate::bid_escrow::job_offer::{CancelJobOfferRequest, JobOffer, PostJobOfferRequest};
use crate::bid_escrow::storage::{BidStorage, JobStorage};
use crate::bid_escrow::types::{BidId, JobOfferId};
use crate::configuration::{Configuration, ConfigurationBuilder};
use crate::modules::refs::ContractRefs;
use crate::utils::withdraw;
use alloc::rc::Rc;
use odra::contract_env::{caller, get_block_time};
use odra::prelude::{vec, vec::Vec};
use odra::types::{event::OdraEvent, Address, Balance, BlockTime};

/// Manages the Bidding process.
#[odra::module(events = [JobCreated, JobOfferCreated, BidSubmitted, BidCancelled, CSPRTransfer])]
pub struct BidEngine {
    bid_storage: BidStorage,
    job_storage: JobStorage,
    refs: ContractRefs,
}

impl BidEngine {
    /// Gets the total number of [JobOffer]s.
    pub fn job_offers_count(&self) -> u32 {
        self.bid_storage.job_offers_count()
    }

    /// Gets the total number of [Bid]s.
    pub fn bids_count(&self) -> u32 {
        self.bid_storage.bids_count()
    }

    /// Gets the [JobOffer] with a given id or `None`.
    pub fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer> {
        self.bid_storage.get_job_offer(&job_offer_id)
    }

    /// Gets the [JobOffer] with a given id or reverts with [JobOfferNotFound](crate::utils::Error::JobOfferNotFound).
    pub fn get_job_offer_or_revert(&self, job_offer_id: &JobOfferId) -> JobOffer {
        self.bid_storage.get_job_offer_or_revert(job_offer_id)
    }

    /// Gets the [Bid] with a given id or `None`.
    pub fn get_bid(&self, bid_id: BidId) -> Option<Bid> {
        self.bid_storage.get_bid(&bid_id)
    }

    /// Gets the [Bid] with a given id or reverts with [BidNotFound](crate::utils::Error::BidNotFound).
    pub fn get_bid_or_revert(&self, bid_id: &BidId) -> Bid {
        self.bid_storage.get_bid_or_revert(bid_id)
    }

    /// Increments bid counter.
    pub fn next_bid_id(&mut self) -> BidId {
        self.bid_storage.next_bid_id()
    }

    /// Writes the [Bid] to the storage.
    pub fn store_bid(&mut self, bid: Bid) {
        self.bid_storage.store_bid(bid)
    }

    /// Gets the [Configuration] of the [Job].
    pub fn get_job_offer_configuration(&self, job: &Job) -> Configuration {
        self.bid_storage.get_job_offer_configuration(job)
    }

    pub fn post_job_offer(
        &mut self,
        expected_timeframe: BlockTime,
        budget: Balance,
        dos_fee: Balance,
    ) {
        let caller = caller();
        let configuration = self.configuration();

        let request = PostJobOfferRequest {
            job_offer_id: self.bid_storage.next_job_offer_id(),
            job_poster_kyced: !self.refs.kyc_token().balance_of(&caller).is_zero(),
            job_poster: caller,
            max_budget: budget,
            expected_timeframe,
            dos_fee,
            start_time: get_block_time(),
            configuration,
        };

        let job_offer = JobOffer::new(request);

        JobOfferCreated::new(&job_offer).emit();
        self.bid_storage
            .add_to_active_offers(job_offer.job_offer_id);
        self.bid_storage.store_job_offer(job_offer);
    }

    pub fn submit_bid(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: Balance,
        reputation_stake: Balance,
        onboard: bool,
        cspr_stake: Option<Balance>,
    ) {
        let worker = caller();

        let job_offer: JobOffer = self.bid_storage.get_job_offer_or_revert(&job_offer_id);
        let bid_id = self.bid_storage.next_bid_id();
        let block_time = get_block_time();

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
            worker_kyced: !self.refs.kyc_token().balance_of(&worker).is_zero(),
            worker_is_va: !self.refs.va_token().balance_of(&worker).is_zero(),
            job_poster: job_offer.job_poster,
            max_budget: job_offer.max_budget,
            auction_state: job_offer.auction_state(block_time),
            va_can_bid_on_public_auction: job_offer.configuration.va_can_bid_on_public_auction(),
        };

        let bid = Bid::new(submit_bid_request);

        let cspr_stake =
            self.stake_cspr_or_reputation_for_bid(reputation_stake, cspr_stake, worker);

        self.bid_storage.store_bid(bid);
        self.bid_storage.store_bid_id(job_offer_id, bid_id);
        self.bid_storage
            .add_to_active_bids(worker, job_offer_id, bid_id);

        let reputation_stake = if reputation_stake.is_zero() {
            None
        } else {
            Some(reputation_stake)
        };

        BidSubmitted::new(
            bid_id,
            job_offer_id,
            worker,
            onboard,
            time,
            payment,
            reputation_stake,
            cspr_stake,
        )
        .emit();
    }

    pub fn cancel_bid(&mut self, bid_id: BidId) {
        let caller = caller();
        let mut bid = self.bid_storage.get_bid_or_revert(&bid_id);
        let job_offer = self.bid_storage.get_job_offer_or_revert(&bid.job_offer_id);

        let cancel_bid_request = CancelBidRequest {
            caller,
            job_offer_status: job_offer.status,
            block_time: get_block_time(),
            va_bid_acceptance_timeout: job_offer.configuration.va_bid_acceptance_timeout(),
        };

        bid.cancel(cancel_bid_request);

        self.unstake_cspr_or_reputation_for_bid(&bid);

        BidCancelled::new(bid_id, caller, bid.job_offer_id).emit();

        self.bid_storage
            .remove_from_active_bids(caller, bid.job_offer_id);
        self.bid_storage.store_bid(bid);
    }

    /// Invalidates the [`Job Offer`](JobOffer), returns `DOS Fee` to the `Job Poster`, returns funds to `Bidders`.
    ///
    /// If a Job with the given id does not exists, contract execution stop with [`Error::JobOfferNotFound`].
    ///
    /// Executes validations: [`HasPermissionsToCancelJobOffer`] and [`CanJobOfferBeCancelled`].
    ///
    /// [`HasPermissionsToCancelJobOffer`]: crate::rules::validation::bid_escrow::HasPermissionsToCancelJobOffer
    /// [`CanJobOfferBeCancelled`]: crate::rules::validation::bid_escrow::CanJobOfferBeCancelled
    /// [`Error::JobOfferNotFound`]: crate::utils::Error::JobOfferNotFound
    pub fn cancel_job_offer(&mut self, job_offer_id: JobOfferId) {
        let mut job_offer = self.bid_storage.get_job_offer_or_revert(&job_offer_id);
        let cancel_job_offer_request = CancelJobOfferRequest {
            caller: caller(),
            block_time: get_block_time(),
        };
        job_offer.cancel(&cancel_job_offer_request);

        self.cancel_all_bids(&job_offer_id);
        self.return_job_offer_poster_dos_fee(&job_offer_id);

        self.bid_storage.update_job_offer(&job_offer_id, job_offer);
        self.bid_storage.remove_from_active_offers(job_offer_id);
    }

    pub fn pick_bid(&mut self, job_offer_id: JobOfferId, bid_id: BidId, cspr_amount: Balance) {
        let mut job_offer = self.bid_storage.get_job_offer_or_revert(&job_offer_id);
        let mut bid = self.bid_storage.get_bid_or_revert(&bid_id);
        let job_id = self.job_storage.next_job_id();

        self.unstake_not_picked(&job_offer_id, &bid_id);
        let pick_bid_request = PickBidRequest {
            job_id,
            job_offer_id,
            bid_id,
            caller: caller(),
            poster: job_offer.job_poster,
            worker: bid.worker,
            is_worker_va: !self.refs.va_token().balance_of(&bid.worker).is_zero(),
            onboard: bid.onboard,
            block_time: get_block_time(),
            timeframe: bid.proposed_timeframe,
            payment: bid.proposed_payment,
            transferred_cspr: cspr_amount,
            stake: bid.reputation_stake,
            external_worker_cspr_stake: bid.cspr_stake.unwrap_or_default(),
        };

        let job = Job::new(&pick_bid_request);

        bid.picked(&pick_bid_request);

        job_offer.in_progress(&pick_bid_request);

        JobCreated::new(&job).emit();

        self.bid_storage.store_bid(bid);
        self.bid_storage.store_job_offer(job_offer);
        self.bid_storage.remove_from_active_offers(job_offer_id);
        self.job_storage.store_job(job);
        self.job_storage.add_to_active_jobs(job_id);
    }

    pub fn slash_voter(&mut self, voter: Address) -> (Vec<JobOfferId>, Vec<BidId>) {
        let mut slashed_job_offers = vec![];
        let mut slashed_bids = vec![];
        for job_offer_id in self.bid_storage.get_active_offers() {
            let job_offer = self.bid_storage.get_job_offer_or_revert(&job_offer_id);
            if voter == job_offer.job_poster {
                self.slash_job_offer(job_offer);
                slashed_job_offers.push(job_offer_id);
            } else {
                self.slash_bid(voter, job_offer_id);
                slashed_bids.push(job_offer_id);
            }
        }
        (slashed_job_offers, slashed_bids)
    }
}

impl BidEngine {
    fn stake_cspr_or_reputation_for_bid(
        &mut self,
        reputation_stake: Balance,
        cspr_stake: Option<Balance>,
        worker: Address,
    ) -> Option<Balance> {
        match cspr_stake {
            None => {
                self.refs.reputation_token().stake(worker, reputation_stake);
                None
            }
            Some(cspr_stake) => Some(cspr_stake),
        }
    }

    fn unstake_cspr_or_reputation_for_bid(&mut self, bid: &Bid) {
        match bid.cspr_stake {
            None => {
                self.refs
                    .reputation_token()
                    .unstake(bid.worker, bid.reputation_stake);
            }
            Some(cspr_stake) => {
                withdraw(&bid.worker, cspr_stake, TransferReason::BidStakeReturn);
            }
        }
    }

    pub fn cancel_all_bids(&mut self, job_offer_id: &JobOfferId) {
        let bids_amount = self.bid_storage.get_bids_count(job_offer_id);
        let mut unstakes: Vec<(Address, Balance)> = Vec::new();
        for i in 0..bids_amount {
            let mut bid = self.bid_storage.get_nth_bid(job_offer_id, i);
            if let Some(cspr) = bid.cspr_stake {
                withdraw(&bid.worker, cspr, TransferReason::BidStakeReturn);
            } else {
                unstakes.push((bid.worker, bid.reputation_stake));
            }
            bid.cancel_without_validation();
            self.bid_storage.store_bid(bid);
        }
        self.refs.reputation_token().bulk_unstake(unstakes);
    }

    pub fn return_job_offer_poster_dos_fee(&mut self, job_offer_id: &JobOfferId) {
        let job_offer = self.bid_storage.get_job_offer_or_revert(job_offer_id);
        withdraw(
            &job_offer.job_poster,
            job_offer.dos_fee,
            TransferReason::DOSFeeReturn,
        );
    }

    fn unstake_not_picked(&mut self, job_offer_id: &JobOfferId, bid_id: &BidId) {
        let bids_amount = self.bid_storage.get_bids_count(job_offer_id);
        let mut unstakes: Vec<(Address, Balance)> = Vec::new();
        for i in 0..bids_amount {
            let mut bid = self.bid_storage.get_nth_bid(job_offer_id, i);

            if bid.bid_id != *bid_id && bid.status == BidStatus::Created {
                if let Some(cspr) = bid.cspr_stake {
                    withdraw(&bid.worker, cspr, TransferReason::BidStakeReturn);
                } else {
                    unstakes.push((bid.worker, bid.reputation_stake));
                }
                bid.reject_without_validation();
                self.bid_storage.store_bid(bid);
            }
        }
        self.refs.reputation_token().bulk_unstake(unstakes);
    }

    /// Builds Configuration for a Bid Escrow Entities
    fn configuration(&self) -> Rc<Configuration> {
        Rc::new(
            ConfigurationBuilder::new(
                self.refs.va_token().total_supply(),
                &self.refs.variable_repository().all_variables(),
            )
            .set_is_bid_escrow(true)
            .only_va_can_create(false)
            .build(),
        )
    }

    fn slash_job_offer(&mut self, mut job_offer: JobOffer) {
        let job_offer_id = job_offer.job_offer_id;
        self.cancel_all_bids(&job_offer_id);
        self.return_job_offer_poster_dos_fee(&job_offer_id);
        self.bid_storage.remove_from_active_offers(job_offer_id);
        job_offer.slash();
        self.bid_storage.update_job_offer(&job_offer_id, job_offer);
    }

    fn slash_bid(&mut self, worker: Address, job_offer_id: JobOfferId) {
        let bid_id = self.bid_storage.get_active_bid_id(worker, job_offer_id);
        let bid_id = match bid_id {
            Some(bid_id) => bid_id,
            None => return,
        };
        let mut bid = self.bid_storage.get_bid_or_revert(&bid_id);
        self.unstake_cspr_or_reputation_for_bid(&bid);
        bid.cancel_without_validation();
        self.bid_storage.store_bid(bid);
        self.bid_storage
            .remove_from_active_bids(worker, job_offer_id);
    }
}
