use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{self, revert},
    Address,
    Error,
    Mapping,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
    U512,
};

use super::balances::BalanceStorage;
use crate::{
    escrow::{bid::ShortenedBid, types::BidId},
    voting::{VotingId, ShortenedBallot},
};

/// A module that stores information about stakes.
/// 
/// 
#[derive(Instance)]
pub struct StakesStorage {
    total_stake: Mapping<Address, U512>,
    bids: Mapping<Address, Vec<(Address, BidId)>>,
    votings: Mapping<Address, Vec<(Address, VotingId)>>,
    #[scoped = "contract"]
    access_control: AccessControl,
    #[scoped = "contract"]
    reputation_storage: BalanceStorage,
}

impl StakesStorage {
    /// Decreases the voter's stake and total stake.
    /// 
    /// # Arguments
    ///
    /// * `ballot` - a short version of ballot that has been casted.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn stake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot) {
        self.access_control.ensure_whitelisted();

        let ShortenedBallot { voter, stake} = ballot;
        self.assert_stake(stake);
        self.assert_balance(voter, stake);

        let voter_contract = casper_env::caller();
        self.inc_total_stake(voter, stake);
        self.votings
            .push_record(&voter, (voter_contract, voting_id));

        // TODO: Emit Stake event.
    }

    /// Decreases the voter's stake and total stake.
    /// 
    /// # Arguments
    ///
    /// * `ballot`- a short version of ballot that has been casted.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn unstake_voting(&mut self, voting_id: VotingId, ballot: ShortenedBallot) {
        self.access_control.ensure_whitelisted();

        let voter_contract = casper_env::caller();
        // Decrement total stake.
        self.dec_total_stake(ballot.voter, ballot.stake);
        self.votings
            .remove_record(&ballot.voter, (voter_contract, voting_id));
    }

    /// Decreases all the voters' stake in voting with the given id, and their total stake.
    /// 
    /// # Arguments
    ///
    /// * `voting_id` - the id of voting to unstake tokens.
    /// * `ballots`- a vector of short version of ballots that has been casted.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn bulk_unstake_voting(&mut self, voting_id: VotingId, ballots: Vec<ShortenedBallot>) {
        self.access_control.ensure_whitelisted();

        let voter_contract = casper_env::caller();

        for ballot in ballots {
            self.dec_total_stake(ballot.voter, ballot.stake);
            self.votings.remove_record(&ballot.voter, (voter_contract, voting_id));
        }
    }

    /// Increases the voter's stake and total stake.
    /// 
    /// # Arguments
    ///
    /// * `bid` - a short version of the bid that has been offered.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn stake_bid(&mut self, bid: ShortenedBid) {
        self.access_control.ensure_whitelisted();
        let ShortenedBid { worker, reputation_stake, bid_id } = bid;

        self.assert_balance(worker, reputation_stake);
        self.assert_stake(reputation_stake);

        let voter_contract = casper_env::caller();
        self.inc_total_stake(worker, reputation_stake);
        self.bids.push_record(&worker, (voter_contract, bid_id));
        // TODO: Emit Stake event.
    }

    /// Decreases the voter's stake and total stake.
    /// 
    /// # Arguments
    ///
    /// * `bid` - the original bid that has been offered.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn unstake_bid(&mut self, bid: ShortenedBid) {
        self.access_control.ensure_whitelisted();

        let voter_contract = casper_env::caller();
        // Decrement total stake.
        self.dec_total_stake(bid.worker, bid.reputation_stake);
        self.bids
            .remove_record(&bid.worker, (voter_contract, bid.bid_id));
    }


    pub fn bulk_unstake_bid(&mut self, bids: Vec<ShortenedBid>) {
        self.access_control.ensure_whitelisted();

        let voter_contract = casper_env::caller();

        for bid in bids {
            // Decrement total stake.
            self.dec_total_stake(bid.worker, bid.reputation_stake);
            self.bids
                .remove_record(&bid.worker, (voter_contract, bid.bid_id));
        }
    }


    /// Returns the total stake of the given account.
    pub fn get_stake(&self, address: Address) -> U512 {
        self.total_stake.get(&address).unwrap_or_default()
    }

    /// Returns all the bids placed by the given account. 
    /// 
    /// A returned vector is a tuple of (BidEscrow contract address, bid id).
    pub fn get_bids(&self, address: &Address) -> Vec<(Address, BidId)> {
        self.bids.get(address).unwrap_or_default()
    }

    /// Returns all the voting the given account participated in. 
    /// 
    /// A returned vector is a tuple of (voting contract address, voting id).
    pub fn get_votings(&self, address: &Address) -> Vec<(Address, VotingId)> {
        self.votings.get(address).unwrap_or_default()
    }
}

impl StakesStorage {
    fn assert_balance(&self, address: Address, stake: U512) {
        let user_stake = self.get_stake(address);
        let available_balance = self
            .reputation_storage
            .balance_of(address)
            .saturating_sub(user_stake);

        if available_balance < stake {
            revert(Error::InsufficientBalance)
        }
    }

    fn assert_stake(&self, stake: U512) {
        if stake.is_zero() {
            revert(Error::ZeroStake)
        }
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
}

trait UpdatableVec<K, R> {
    fn push_record(&self, key: &K, record: R);
    fn remove_record(&self, key: &K, record: R);
}

impl<Key, Record> UpdatableVec<Key, Record> for Mapping<Key, Vec<Record>>
where
    Key: ToBytes + FromBytes + CLTyped,
    Record: ToBytes + FromBytes + CLTyped + PartialEq,
{
    fn push_record(&self, key: &Key, record: Record) {
        let mut records = self.get(key).unwrap_or_default();
        records.push(record);
        self.set(key, records);
    }

    fn remove_record(&self, key: &Key, record: Record) {
        let mut records = self.get(key).unwrap_or_default();
        if let Some(position) = records.iter().position(|r| r == &record) {
            records.remove(position);
        }
        self.set(key, records);
    }
}
