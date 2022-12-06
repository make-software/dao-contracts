use casper_dao_contracts::bid::{
    bid::Bid,
    types::{BidId, JobOfferId},
};
use casper_dao_utils::{BlockTime, TestContract};

use crate::common::{
    params::{Account, Balance, CsprBalance},
    DaoWorld,
};

impl DaoWorld {
    pub fn get_bid(&self, offer_id: JobOfferId, poster: Account) -> Option<Bid> {
        let poster = self.get_address(&poster);
        let bid_id = self.bids.get(&(offer_id, poster))?;

        self.bid_escrow.get_bid(*bid_id)
    }

    pub fn get_job_offer_id(&self, job_poster: &Account) -> Option<&JobOfferId> {
        let job_poster = self.get_address(&job_poster);
        self.offers.get(&job_poster)
    }

    pub fn post_bid(
        &mut self,
        offer_id: JobOfferId,
        bidder: Account,
        timeframe: BlockTime,
        budget: CsprBalance,
        stake: Balance,
        onboarding: bool,
        cspr_stake: Option<CsprBalance>,
    ) {
        let _bids_count = self.bid_escrow.bids_count();
        let bidder = self.get_address(&bidder);

        let result = match cspr_stake {
            None => self
                .bid_escrow
                .as_account(bidder)
                .submit_bid(0, timeframe, *budget, *stake, onboarding, None),
            Some(cspr_stake) => self
                .bid_escrow
                .as_account(bidder)
                .submit_bid_with_cspr_amount(
                    0,
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
        }
    }

    pub fn post_offer(
        &mut self,
        poster: Account,
        timeframe: BlockTime,
        maximum_budget: CsprBalance,
        dos_fee: CsprBalance,
    ) -> JobOfferId {
        let poster = self.get_address(&poster);

        self.bid_escrow
            .as_account(poster)
            .post_job_offer_with_cspr_amount(timeframe, *maximum_budget, *dos_fee)
            .unwrap();

        let offer_id = self.bid_escrow.job_offers_count();
        self.offers.insert(poster, offer_id);
        offer_id
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
}
