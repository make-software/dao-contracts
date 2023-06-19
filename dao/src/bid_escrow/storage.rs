//! Submodules storing Job and Bid data.

use crate::bid_escrow::bid::Bid;
use crate::bid_escrow::job::Job;
use crate::bid_escrow::job_offer::JobOffer;
use crate::bid_escrow::types::{BidId, JobId, JobOfferId};
use crate::configuration::Configuration;
use crate::utils::Error;
use crate::voting::types::VotingId;
use odra::types::Address;
use odra::{List, Mapping, Sequence, UnwrapOrRevert, Variable};

/// Stores [Bid]-related variables and mappings.
#[odra::module]
pub struct BidStorage {
    job_offers: Mapping<JobOfferId, JobOffer>,
    job_offers_count: Sequence<JobOfferId>,
    bids: Mapping<BidId, Bid>,
    job_offers_bids: Mapping<JobOfferId, List<BidId>>,
    bids_count: Sequence<BidId>,
    active_job_offers_ids: Variable<Vec<JobOfferId>>,
    worker_bids: Mapping<(Address, JobOfferId), Option<BidId>>,
}

impl BidStorage {
    /// Writes an job offer to the storage.
    pub fn store_job_offer(&mut self, offer: JobOffer) {
        let offer_id = offer.job_offer_id;
        self.job_offers.set(&offer_id, offer);
    }

    /// Updates the value under the `offer_if` key.
    pub fn update_job_offer(&mut self, offer_id: &JobOfferId, offer: JobOffer) {
        self.job_offers.set(offer_id, offer);
    }

    /// Writes a pair [JobOfferId]-[BidId] to the storage.
    pub fn store_bid_id(&mut self, offer_id: JobOfferId, bid_id: BidId) {
        let mut job_offers_bids = self.job_offers_bids.get_instance(&offer_id);
        job_offers_bids.push(bid_id);
    }

    /// Writes the [Bid] to the storage.
    pub fn store_bid(&mut self, bid: Bid) {
        self.bids.set(&bid.bid_id.clone(), bid);
    }

    /// Gets the total number of [JobOffer]s.
    pub fn get_job_offer(&self, job_offer_id: &JobOfferId) -> Option<JobOffer> {
        self.job_offers.get(job_offer_id)
    }

    /// Gets the [JobOffer] with a given id or reverts with [JobOfferNotFound](casper_dao_utils::Error::JobOfferNotFound).
    pub fn get_job_offer_or_revert(&self, job_offer_id: &JobOfferId) -> JobOffer {
        self.get_job_offer(job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound)
    }

    /// Gets the [Bid] with a given id or `None`.
    pub fn get_bid(&self, bid_id: &BidId) -> Option<Bid> {
        self.bids.get(bid_id)
    }

    /// Gets the [Bid] with a given id or reverts with [Error::BidNotFound].
    pub fn get_bid_or_revert(&self, bid_id: &BidId) -> Bid {
        self.get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound)
    }

    /// Gets the nth [Bid] for the [JobOffer] with a given id or reverts with [Error::BidNotFound].
    pub fn get_nth_bid(&self, offer_id: &JobOfferId, n: u32) -> Bid {
        let bid_ids = self.job_offers_bids.get_instance(offer_id);

        let bid_id = bid_ids.get(n).unwrap_or_revert_with(Error::BidNotFound);

        self.get_bid_or_revert(&bid_id)
    }

    /// Gets the total number of [JobOffer]s.
    pub fn job_offers_count(&self) -> u32 {
        self.job_offers_count.get_current_value()
    }

    /// Gets the total number of [Bid]s.
    pub fn bids_count(&self) -> u32 {
        self.bids_count.get_current_value()
    }

    /// Increments bid counter.
    pub fn next_bid_id(&mut self) -> BidId {
        self.bids_count.next_value()
    }

    /// Increments job offers counter.
    pub fn next_job_offer_id(&mut self) -> JobOfferId {
        self.job_offers_count.next_value()
    }

    /// Gets the total number of [JobOffer]s.
    pub fn get_bids_count(&self, offer_id: &JobOfferId) -> u32 {
        self.job_offers_bids.get_instance(offer_id).len()
    }

    /// Gets the [Configuration] of the [Job].
    pub fn get_job_offer_configuration(&self, job: &Job) -> Configuration {
        let job_offer = self.get_job_offer_or_revert(&job.job_offer_id());
        job_offer.configuration
    }

    pub fn add_to_active_offers(&mut self, job_offer_id: JobOfferId) {
        let mut active_list = self.active_job_offers_ids.get_or_default();
        active_list.push(job_offer_id);
        self.active_job_offers_ids.set(active_list);
    }

    pub fn remove_from_active_offers(&mut self, job_offer_id: JobOfferId) {
        let mut active_list = self.active_job_offers_ids.get_or_default();
        active_list.retain(|&id| id != job_offer_id);
        self.active_job_offers_ids.set(active_list);
    }

    pub fn get_active_offers(&self) -> Vec<JobOfferId> {
        self.active_job_offers_ids.get_or_default()
    }

    pub fn add_to_active_bids(&mut self, worker: Address, job_offer_id: JobOfferId, bid_id: BidId) {
        self.worker_bids.set(&(worker, job_offer_id), Some(bid_id));
    }

    pub fn remove_from_active_bids(&mut self, worker: Address, job_offer_id: JobOfferId) {
        self.worker_bids.set(&(worker, job_offer_id), None);
    }

    pub fn get_active_bid_id(&self, worker: Address, job_offer_id: JobOfferId) -> Option<BidId> {
        self.worker_bids.get_or_default(&(worker, job_offer_id))
    }
}

/// Stores [Job]-related variables and mappings.
#[odra::module]
pub struct JobStorage {
    jobs: Mapping<JobId, Job>,
    jobs_for_voting: Mapping<VotingId, JobId>,
    jobs_count: Sequence<JobId>,
    active_jobs: Variable<Vec<JobId>>,
}

impl JobStorage {
    /// Links voting with a job.
    pub fn store_job_for_voting(&mut self, voting_id: VotingId, job_id: JobId) {
        self.jobs_for_voting.set(&voting_id, job_id);
    }

    /// Gets the [Job] with a given id or `None`.
    pub fn get_job(&self, job_id: JobId) -> Option<Job> {
        self.jobs.get(&job_id)
    }

    /// Gets the [Job] matching to a given id or reverts with [VotingIdNotFound](casper_dao_utils::Error::VotingIdNotFound).
    pub fn get_job_by_voting_id(&self, voting_id: VotingId) -> Job {
        let job_id = self
            .jobs_for_voting
            .get(&voting_id)
            .unwrap_or_revert_with(Error::VotingIdNotFound);

        self.get_job_or_revert(job_id)
    }

    /// Gets the [Job] with a given id or reverts with [MappingItemNotAvailable](casper_dao_utils::Error::MappingItemNotAvailable).
    pub fn get_job_or_revert(&self, job_id: JobId) -> Job {
        self.jobs
            .get(&job_id)
            .unwrap_or_revert_with(Error::JobNotFound)
    }

    /// Writes a [Job] to the storage.
    pub fn store_job(&mut self, job: Job) {
        self.jobs.set(&job.job_id(), job);
    }

    /// Gets the current value of jobs counter.
    pub fn jobs_count(&self) -> u32 {
        self.jobs_count.get_current_value()
    }

    /// Increments jobs counter.
    pub fn next_job_id(&mut self) -> JobId {
        self.jobs_count.next_value()
    }

    /// Adds a job to the list of active jobs.
    pub fn add_to_active_jobs(&mut self, job_id: JobId) {
        let mut active_list = self.active_jobs.get_or_default();
        active_list.push(job_id);
        self.active_jobs.set(active_list);
    }

    /// Removes a job from the list of active jobs.
    pub fn remove_from_active_jobs(&mut self, job_id: JobId) {
        let mut active_list = self.active_jobs.get_or_default();
        active_list.retain(|&id| id != job_id);
        self.active_jobs.set(active_list);
    }

    /// Returns all active jobs.
    pub fn get_active_jobs(&self) -> Vec<JobId> {
        self.active_jobs.get_or_default()
    }
}
