//! Submodules storing Job and Bid data.
use casper_dao_modules::sequence::SequenceGenerator;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    Address,
    Error,
    Mapping,
    VecMapping,
};

use super::{
    bid::Bid,
    job::Job,
    job_offer::JobOffer,
    types::{BidId, JobId, JobOfferId},
};
use crate::{config::Configuration, voting::VotingId};

/// Stores [Bid]-related variables and mappings.
#[derive(Instance)]
pub struct BidStorage {
    pub job_offers: Mapping<JobOfferId, JobOffer>,
    active_job_offers_ids: Mapping<Address, Vec<JobOfferId>>,
    job_offers_count: SequenceGenerator<JobOfferId>,
    bids: Mapping<BidId, Bid>,
    job_offers_bids: VecMapping<JobOfferId, BidId>,
    bids_count: SequenceGenerator<BidId>,
}

impl BidStorage {
    /// Writes an job offer to the storage.
    pub fn store_job_offer(&mut self, offer: JobOffer) {
        let poster = offer.job_poster;
        let offer_id = offer.job_offer_id;
        self.job_offers.set(&offer_id, offer);

        let mut job_offers = self.active_job_offers_ids.get(&poster).unwrap_or_default();
        job_offers.push(offer_id);
        self.active_job_offers_ids.set(&poster, job_offers);
    }

    /// Updates the value under the `offer_if` key.
    pub fn update_job_offer(&mut self, offer_id: JobOfferId, offer: JobOffer) {
        self.job_offers.set(&offer_id, offer);
    }

    /// Writes a pair [JobOfferId]-[BidId] to the storage.
    pub fn store_bid_id(&mut self, offer_id: JobOfferId, bid_id: BidId) {
        self.job_offers_bids.add(offer_id, bid_id);
    }

    /// Writes the [Bid] to the storage.
    pub fn store_bid(&mut self, bid: Bid) {
        self.bids.set(&bid.bid_id.clone(), bid);
    }

    /// Filters active job offer ids, remaining only the given offer id.
    pub fn store_active_job_offer_id(&mut self, poster: &Address, offer_id: JobOfferId) {
        // TODO: Filter in place.
        let offers: Vec<JobOfferId> = self.active_job_offers_ids.get(poster).unwrap_or_default();
        let offers: Vec<JobOfferId> = offers
            .iter()
            .filter(|id| id == &&offer_id)
            .cloned()
            .collect();
        self.active_job_offers_ids.set(poster, offers);
    }

    /// Removes from the storage all the active job offer ids of the Bidder.
    pub fn clear_active_job_offers_ids(&mut self, bidder: &Address) -> Vec<JobOfferId> {
        let job_offer_ids = self.active_job_offers_ids.get(bidder).unwrap_or_default();
        self.active_job_offers_ids.set(bidder, vec![]);
        job_offer_ids
    }

    /// Gets the total number of [JobOffer]s.
    pub fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer> {
        self.job_offers.get_or_none(&job_offer_id)
    }

    /// Gets the [JobOffer] with a given id or reverts with [JobOfferNotFound](casper_dao_utils::Error::JobOfferNotFound).
    pub fn get_job_offer_or_revert(&self, job_offer_id: JobOfferId) -> JobOffer {
        self.get_job_offer(job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound)
    }

    /// Gets the [Bid] with a given id or `None`.
    pub fn get_bid(&self, bid_id: BidId) -> Option<Bid> {
        self.bids.get_or_none(&bid_id)
    }

    /// Gets the [Bid] with a given id or reverts with [Error::BidNotFound].
    pub fn get_bid_or_revert(&self, bid_id: BidId) -> Bid {
        self.get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound)
    }

    /// Gets the nth [Bid] for the [JobOffer] with a given id or reverts with [Error::BidNotFound].
    pub fn get_nth_bid(&self, offer_id: JobOfferId, n: u32) -> Bid {
        let bid_id = self
            .job_offers_bids
            .get(offer_id, n)
            .unwrap_or_revert_with(Error::BidNotFound);

        self.get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound)
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
    pub fn get_bids_count(&self, offer_id: JobOfferId) -> u32 {
        self.job_offers_bids.len(offer_id)
    }

    /// Gets the [Configuration] of the [Job].
    pub fn get_job_offer_configuration(&self, job: &Job) -> Configuration {
        let job_offer = self.get_job_offer_or_revert(job.job_offer_id());
        job_offer.configuration
    }
}

/// Stores [Job]-related variables and mappings.
#[derive(Instance)]
pub struct JobStorage {
    jobs: Mapping<JobId, Job>,
    jobs_for_voting: Mapping<VotingId, JobId>,
    jobs_count: SequenceGenerator<JobId>,
}

impl JobStorage {
    /// Links voting with a job.
    pub fn store_job_for_voting(&mut self, voting_id: VotingId, job_id: JobId) {
        self.jobs_for_voting.set(&voting_id, job_id);
    }

    /// Gets the [Job] with a given id or `None`.
    pub fn get_job(&self, job_id: JobId) -> Option<Job> {
        self.jobs.get_or_none(&job_id)
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
        self.jobs.get_or_revert(&job_id)
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
}

#[cfg(test)]
mod test {
    use crate::bid_escrow::types::JobOfferId;

    #[test]
    fn asa() {
        let offers: Vec<JobOfferId> = vec![112, 13, 124];
        let offers: Vec<JobOfferId> = offers.iter().filter(|id| id == &&124).cloned().collect();

        dbg!(offers);
    }
}
