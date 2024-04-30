use crate::common::params::Account;
use crate::common::DaoWorld;
use dao::bid_escrow::bid::Bid;
use dao::bid_escrow::types::{BidId, JobOfferId};
use dao::utils::Error;
use odra::test_env;
use odra::types::{Balance, BlockTime};

impl DaoWorld {
    pub fn get_job_offer_id(&self, job_poster: &Account) -> Option<&JobOfferId> {
        let job_poster = self.get_address(job_poster);
        self.offers.get(&job_poster)
    }

    #[allow(clippy::too_many_arguments)]
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
        let bidder = self.get_address(&bidder);

        test_env::set_caller(bidder);
        match cspr_stake {
            None => self
                .bid_escrow
                .submit_bid(offer_id, timeframe, budget, stake, onboarding, None),
            Some(cspr_stake) => self.bid_escrow.with_tokens(cspr_stake).submit_bid(
                offer_id,
                timeframe,
                budget,
                stake,
                onboarding,
                Some(cspr_stake),
            ),
        };
        let bid_id = self.bid_escrow.bids_count();
        self.bids.insert((offer_id, bidder), bid_id);
    }

    pub fn get_bid(&self, offer_id: JobOfferId, poster: Account) -> Option<Bid> {
        let poster = self.get_address(&poster);
        let bid_id = self.bids.get(&(offer_id, poster))?;

        self.bid_escrow.get_bid(*bid_id)
    }

    pub fn cancel_bid(&mut self, worker: Account, job_offer_id: JobOfferId, bid_id: BidId) {
        let worker = self.get_address(&worker);
        test_env::set_caller(worker);
        let bids_count = self.bid_escrow.bids_count();
        self.bid_escrow.cancel_bid(bid_id);
        let after_bids_count = self.bid_escrow.bids_count();
        if after_bids_count < bids_count {
            self.bids.remove(&(job_offer_id, worker));
        }
    }

    pub fn post_offer(
        &mut self,
        poster: Account,
        timeframe: BlockTime,
        maximum_budget: Balance,
        dos_fee: Balance,
    ) -> Result<JobOfferId, Error> {
        let poster = self.get_address(&poster);

        test_env::set_caller(poster);
        self.bid_escrow
            .with_tokens(dos_fee)
            .post_job_offer(timeframe, maximum_budget, dos_fee);

        let offer_id = self.bid_escrow.job_offers_count();
        self.offers.insert(poster, offer_id);
        Ok(offer_id)
    }

    pub fn pick_bid(&mut self, job_poster: Account, worker: Account) {
        let job_poster = self.get_address(&job_poster);
        let worker = self.get_address(&worker);
        let job_offer_id = self.offers.get(&job_poster).expect("Job Offer not found.");
        let bid_id = self
            .bids
            .get(&(*job_offer_id, worker))
            .expect("Bid id not found.");
        let bid = self.bid_escrow.get_bid(*bid_id).expect("Bid not found.");
        test_env::set_caller(job_poster);
        self.bid_escrow.with_tokens(bid.proposed_payment).pick_bid(
            *job_offer_id,
            *bid_id,
            bid.proposed_payment,
        );
    }

    pub fn pick_bid_without_enough_payment(&mut self, job_poster: Account, worker: Account) {
        let job_poster = self.get_address(&job_poster);
        let worker = self.get_address(&worker);
        let job_offer_id = self.offers.get(&job_poster).expect("Job Offer not found.");
        let bid_id = self
            .bids
            .get(&(*job_offer_id, worker))
            .expect("Bid id not found.");
        let bid = self.bid_escrow.get_bid(*bid_id).expect("Bid not found.");
        let payment = bid.proposed_payment - Balance::one();
        test_env::assert_exception(Error::PurseBalanceMismatch, || {
            test_env::set_caller(job_poster);
            self.bid_escrow.with_tokens(payment).pick_bid(
                *job_offer_id,
                *bid_id,
                bid.proposed_payment,
            );
        });
    }

    // pub fn slash_all_active_job_offers(&mut self, bidder: Account) {
    //     let bidder = self.get_address(&bidder);
    //     self.bid_escrow.slash_all_active_job_offers(bidder);
    // }

    // pub fn slash_bid(&mut self, bid_id: u32) {
    //     self.bid_escrow.slash_bid(bid_id);
    // }
}
