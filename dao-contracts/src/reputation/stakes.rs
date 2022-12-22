use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::revert,
    Address,
    Error,
    Mapping,
};
use casper_types::U512;

use super::balances::BalanceStorage;
use crate::{
    escrow::{bid::Bid, types::BidId},
    voting::{Ballot, VotingId},
};

#[derive(Instance)]
pub struct StakeInfo {
    total_stake: Mapping<Address, U512>,
    bids: Mapping<Address, Vec<(Address, BidId)>>,
    votings: Mapping<Address, Vec<(Address, VotingId)>>,
    #[scoped = "contract"]
    access_control: AccessControl,
    #[scoped = "contract"]
    reputation_storage: BalanceStorage,
}

impl StakeInfo {
    pub fn get_stake(&self, address: Address) -> U512 {
        self.total_stake.get(&address).unwrap_or_default()
    }

    pub fn get_bids(&self, address: &Address) -> Vec<(Address, BidId)> {
        self.bids.get(address).unwrap_or_default()
    }

    pub fn get_votings(&self, address: &Address) -> Vec<(Address, VotingId)> {
        self.votings.get(address).unwrap_or_default()
    }

    pub fn stake_voting(
        &mut self,
        voter_contract: Address,
        ballot: Ballot,
    ) {
        self.access_control.ensure_whitelisted();

        let Ballot {
            voter,
            stake,
            voting_id,
            ..
        } = ballot;
        if stake.is_zero() {
            revert(Error::ZeroStake);
        }
        if stake
            > self.reputation_storage
                .balance_of(voter)
                .saturating_sub(self.get_stake(voter))
        {
            revert(Error::InsufficientBalance);
        }
        self.inc_total_stake(voter, stake);
        self.push_voting_id(&voter, (voter_contract, voting_id));

        // TODO: Emit Stake event.
    }

    pub fn unstake_voting(&mut self, voter_contract: Address, ballot: Ballot) {
        self.access_control.ensure_whitelisted();

        // Decrement total stake.
        self.dec_total_stake(ballot.voter, ballot.stake);
        self.remove_voting_id(&ballot.voter, (voter_contract, ballot.voting_id));
    }

    pub fn stake_bid(
        &mut self,
        voter_contract: Address,
        bidder: Address,
        bid_id: BidId,
        stake: U512,
    ) {
        self.access_control.ensure_whitelisted();

        if stake.is_zero() {
            revert(Error::ZeroStake);
        }
        if stake
            > self.reputation_storage
                .balance_of(bidder)
                .saturating_sub(self.get_stake(bidder))
        {
            revert(Error::InsufficientBalance);
        }
        self.inc_total_stake(bidder, stake);
        self.push_bid_id(&bidder, (voter_contract, bid_id));
        // TODO: Emit Stake event.
    }

    pub fn unstake_bid(&mut self, voter_contract: Address, bid: Bid) {
        self.access_control.ensure_whitelisted();

        // Decrement total stake.
        self.dec_total_stake(bid.worker, bid.reputation_stake);
        self.remove_bid_id(&bid.worker, (voter_contract, bid.bid_id));
    }

    fn inc_total_stake(&mut self, account: Address, amount: U512) {
        let new_value = self.get_stake(account) + amount;
        self.total_stake.set(&account, new_value);
    }

    fn dec_total_stake(&mut self, account: Address, amount: U512) {
        let new_value = self
            .get_stake(account)
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::ZeroStake);
        self.total_stake.set(&account, new_value);
    }

    pub fn push_bid_id(&mut self, address: &Address, record: (Address, BidId)) {
        let mut records = self.bids.get(address).unwrap_or_default();
        records.push(record);
        self.bids.set(address, records);
    }

    pub fn push_voting_id(&mut self, address: &Address, record: (Address, BidId)) {
        let mut records = self.votings.get(address).unwrap_or_default();
        records.push(record);
        self.votings.set(address, records);
    }

    pub fn remove_bid_id(&mut self, address: &Address, record: (Address, BidId)) {
        let mut records = self.bids.get(address).unwrap_or_default();
        if let Some(position) = records.iter().position(|r| r == &record) {
            records.remove(position);
        }
        self.bids.set(address, records);
    }

    pub fn remove_voting_id(&mut self, address: &Address, record: (Address, BidId)) {
        let mut records = self.votings.get(address).unwrap_or_default();
        if let Some(position) = records.iter().position(|r| r == &record) {
            records.remove(position);
        }
        self.votings.set(address, records);
    }
}
