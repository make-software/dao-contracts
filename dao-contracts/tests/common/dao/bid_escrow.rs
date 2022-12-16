use casper_dao_contracts::escrow::{
    bid::Bid,
    types::{BidId, JobOfferId},
};
use casper_dao_utils::{casper_contract::contract_api::runtime::print, BlockTime, TestContract};

use crate::common::{
    params::{Account, Balance},
    DaoWorld,
};

impl DaoWorld {
    pub fn get_bid(&self, offer_id: JobOfferId, poster: Account) -> Option<Bid> {
        let poster = self.get_address(&poster);
        let bid_id = self.bids.get(&(offer_id, poster))?;

        self.bid_escrow.get_bid(*bid_id)
    }

    pub fn get_job_offer_id(&self, job_poster: &Account) -> Option<&JobOfferId> {
        let job_poster = self.get_address(job_poster);
        self.offers.get(&job_poster)
    }

    pub fn post_bid(
        &mut self,
        offer_id: JobOfferId,
        bidder: Account,
        timeframe: BlockTime,
        budget: Balance,
        stake: Balance,
        onboarding: bool,
        cspr_stake: Option<Balance>,
    ) {
        let _bids_count = self.bid_escrow.bids_count();
        let bidder = self.get_address(&bidder);

        let result = match cspr_stake {
            None => self
                .bid_escrow
                .as_account(bidder)
                .submit_bid(offer_id, timeframe, *budget, *stake, onboarding, None),
            Some(cspr_stake) => self
                .bid_escrow
                .as_account(bidder)
                .submit_bid_with_cspr_amount(
                    offer_id,
                    timeframe,
                    *budget,
                    *stake,
                    onboarding,
                    *cspr_stake,
                ),
        };

        if result.is_ok() {
            let bid_id = self.bid_escrow.bids_count();
            self.bids.insert((offer_id, bidder), bid_id);
        }
    }

    pub fn cancel_bid(&mut self, worker: Account, job_offer_id: JobOfferId, bid_id: BidId) {
        let worker = self.get_address(&worker);
        let result = self.bid_escrow.as_account(worker).cancel_bid(bid_id);
        if result.is_ok() {
            self.bids.remove(&(job_offer_id, worker));
        } else {
            dbg!(result);
        }
    }

    pub fn post_offer(
        &mut self,
        poster: Account,
        timeframe: BlockTime,
        maximum_budget: Balance,
        dos_fee: Balance,
    ) -> Result<JobOfferId, casper_dao_utils::Error> {
        let poster = self.get_address(&poster);

        self.bid_escrow
            .as_account(poster)
            .post_job_offer_with_cspr_amount(timeframe, *maximum_budget, *dos_fee)?;

        let offer_id = self.bid_escrow.job_offers_count();
        self.offers.insert(poster, offer_id);
        Ok(offer_id)
    }

    pub fn pick_bid(&mut self, job_poster: Account, worker: Account) {
        let job_poster = self.get_address(&job_poster);
        let worker = self.get_address(&worker);

        let job_offer_id = self.offers.get(&job_poster).unwrap();
        let bid_id = self.bids.get(&(*job_offer_id, worker)).unwrap();
        let bid = self.bid_escrow.get_bid(*bid_id).unwrap();

        self.bid_escrow
            .as_account(job_poster)
            .pick_bid_with_cspr_amount(*job_offer_id, *bid_id, bid.proposed_payment)
            .unwrap();
    }

    pub fn slash_all_active_job_offers(&mut self, bidder: Account) {
        let bidder = self.get_address(&bidder);
        self.bid_escrow
            .slash_all_active_job_offers(bidder)
            .expect("Can't cancel bidder.");
    }

    pub fn slash_bid(&mut self, bid_id: u32) {
        self.bid_escrow.slash_bid(bid_id).expect("Can't slash bid");
    }
}
