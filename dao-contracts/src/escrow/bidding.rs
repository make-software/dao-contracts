use std::{borrow::Borrow, rc::Rc};

use casper_dao_utils::{
    casper_dao_macros::Instance,
    casper_env::{caller, get_block_time},
    cspr,
    Address,
    BlockTime,
};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::{
    escrow::{
        bid::{Bid, CancelBidRequest, ShortenedBid, SubmitBidRequest},
        job_offer::{CancelJobOfferRequest, JobOffer, PostJobOfferRequest},
        storage::BidStorage,
        types::{BidId, JobOfferId},
    },
    refs::{ContractRefs, ContractRefsWithKycStorage},
    voting::{kyc_info::KycInfo, onboarding_info::OnboardingInfo},
    Configuration,
    ConfigurationBuilder,
    ReputationContractInterface,
};

#[derive(Instance)]
pub struct Bidding {
    pub bid_storage: BidStorage,
    kyc: KycInfo,
    onboarding_info: OnboardingInfo,
    refs: ContractRefsWithKycStorage,
}

impl Bidding {
    delegate! {
        to self.bid_storage {
            pub fn job_offers_count(&self) -> u32;
            pub fn bids_count(&self) -> u32;
            pub fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;
            pub fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
        }
    }

    pub fn post_job_offer(&mut self, expected_timeframe: BlockTime, max_budget: U512, purse: URef) {
        let caller = caller();
        let configuration = self.configuration();

        let request = PostJobOfferRequest {
            job_offer_id: self.bid_storage.next_job_offer_id(),
            job_poster_kyced: self.kyc.is_kycd(&caller),
            job_poster: caller,
            max_budget,
            expected_timeframe,
            dos_fee: cspr::deposit(purse),
            start_time: get_block_time(),
            configuration,
        };

        let job_offer = JobOffer::new(request);
        self.bid_storage.store_job_offer(job_offer);
        // TODO: Emit event
        // JobOfferCreated::new(&job_offer).emit();
    }

    pub fn submit_bid(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    ) {
        let worker = caller();
        let job_offer: JobOffer = self.bid_storage.get_job_offer_or_revert(job_offer_id);
        let bid_id = self.bid_storage.next_bid_id();
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

        self.bid_storage.store_bid(bid);
        self.bid_storage.store_bid_id(job_offer_id, bid_id);
        // TODO: Implement Event
        // BidCreated::new(&bid).emit();
    }

    pub fn cancel_bid(&mut self, bid_id: BidId) {
        let caller = caller();
        let mut bid = self.bid_storage.get_bid_or_revert(bid_id);
        let job_offer = self.bid_storage.get_job_offer_or_revert(bid.job_offer_id);

        let cancel_bid_request = CancelBidRequest {
            caller,
            job_offer_status: job_offer.status,
            block_time: get_block_time(),
            va_bid_acceptance_timeout: job_offer.configuration.va_bid_acceptance_timeout(),
        };

        bid.cancel(cancel_bid_request);

        self.unstake_cspr_or_reputation_for_bid(&bid);

        // TODO: Implement Event
        self.bid_storage.store_bid(bid);
    }

    pub fn cancel_job_offer(&mut self, job_offer_id: JobOfferId) {
        let mut job_offer = self.bid_storage.get_job_offer_or_revert(job_offer_id);
        let cancel_job_offer_request = CancelJobOfferRequest {
            caller: caller(),
            block_time: get_block_time(),
        };

        job_offer.cancel(&cancel_job_offer_request);

        self.cancel_all_bids(job_offer_id);
        self.return_job_offer_poster_dos_fee(job_offer_id);

        self.bid_storage.update_job_offer(job_offer_id, job_offer);
    }
}

impl Bidding {
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
                let cspr_stake = cspr::deposit(purse);
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
                cspr::withdraw(bid.worker, cspr_stake);
            }
        }
    }

    pub(crate) fn cancel_all_bids(&mut self, job_offer_id: JobOfferId) {
        let bids_amount = self.bid_storage.get_bids_count(job_offer_id);
        let mut bids = Vec::<ShortenedBid>::new();
        for i in 0..bids_amount {
            let mut bid = self.bid_storage.get_nth_bid(job_offer_id, i);
            if let Some(cspr) = bid.cspr_stake {
                cspr::withdraw(bid.worker, cspr);
            }
            bids.push(bid.borrow().into());
            bid.cancel_without_validation();
            self.bid_storage.store_bid(bid);
        }
        self.refs.reputation_token().bulk_unstake_bid(bids);
    }

    pub(crate) fn return_job_offer_poster_dos_fee(&mut self, job_offer_id: JobOfferId) {
        let job_offer = self.bid_storage.get_job_offer_or_revert(job_offer_id);
        cspr::withdraw(job_offer.job_poster, job_offer.dos_fee);
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
}
