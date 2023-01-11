use std::collections::{BTreeMap, BTreeSet};

use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, CLTyped, FromBytes, Instance, ToBytes},
    casper_env::{caller, emit, revert},
    Address,
    Error,
    Mapping,
    Variable,
};
use casper_types::{URef, U512};
use delegate::delegate;

use self::events::{Burn, Mint};
use super::passive_rep::PassiveReputation;
use crate::{
    escrow::types::BidId,
    voting::{Choice, VotingId},
};

// Interface of the Reputation Contract.
//
// It should be implemented by [`ReputationContract`], [`ReputationContractCaller`]
// and [`ReputationContractTest`].
#[casper_contract_interface]
pub trait ReputationContractInterface {
    /// Constructor method.
    ///
    /// It initializes contract elements:
    /// * Events dictionary.
    /// * Named keys of [`TokenWithStaking`], [`AccessControl`].
    /// * Set [`caller`] as the owner of the contract.
    /// * Add [`caller`] to the whitelist.
    ///
    /// It emits [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist) events.
    fn init(&mut self);

    /// Mint new tokens. Add `amount` of new tokens to the balance of the `recipient` and
    /// increment the total supply. Only whitelisted addresses are permitted to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Mint`](casper_dao_contracts::reputation::events::Mint) event.
    fn mint(&mut self, recipient: Address, amount: U512);

    /// Increases the balance of the passive reputation of the given address.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    fn mint_passive(&mut self, recipient: Address, amount: U512);

    /// Burn existing tokens. Remove `amount` of existing tokens from the balance of the `owner`
    /// and decrement the total supply. Only whitelisted addresses are permitted to call this
    /// method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Burn`](casper_dao_contracts::reputation::events::Burn) event.
    fn burn(&mut self, owner: Address, amount: U512);

    /// Decreases the balance of the passive reputation of the given address.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance) if the passed
    /// amount exceeds the balance of the passive reputation of the given address.
    fn burn_passive(&mut self, owner: Address, amount: U512);

    /// Change ownership of the contract. Transfer the ownership to the `owner`. Only current owner
    /// is permitted to call this method.
    ///
    /// See [AccessControl](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);

    /// Add new address to the whitelist.
    ///
    /// See [AccessControl](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);

    /// Remove address from the whitelist.
    ///
    /// See [AccessControl](AccessControl::remove_from_whitelist())
    fn remove_from_whitelist(&mut self, address: Address);

    /// Returns the address of the current owner.
    fn get_owner(&self) -> Option<Address>;

    /// Returns the total token supply.
    fn total_supply(&self) -> U512;

    /// Returns the current token balance of the given address.
    fn balance_of(&self, address: Address) -> U512;

    /// Returns the current passive balance of the given address.
    fn passive_balance_of(&self, address: Address) -> U512;

    /// Checks whether the given address is added to the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;

    /// Stakes the reputation for a given voting and choice.
    fn stake_voting(&mut self, voter: Address, voting_id: VotingId, choice: Choice, amount: U512);

    fn unstake_voting(&mut self, voter: Address, voting_id: VotingId);

    fn stake_bid(&mut self, voter: Address, bid_id: BidId, amount: U512);

    fn unstake_bid(&mut self, voter: Address, bid_id: BidId);

    fn get_stake(&self, address: Address) -> U512;

    fn all_balances(&self) -> (U512, Balances);

    fn partial_balances(&self, addresses: Vec<Address>) -> (U512, Balances);

    // Redistributes the reputation based on the voting summary
    fn bulk_mint_burn(&mut self, mints: BTreeMap<Address, U512>, burns: BTreeMap<Address, U512>);

    fn burn_all(&mut self, owner: Address);

    fn stakes_info(&self, address: Address) -> AccountStakeInfo;
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
#[derive(Instance)]
pub struct ReputationContract {
    total_supply: Variable<U512>,
    balances: Variable<Balances>,
    // (owner, staker, voting) -> (stake, choice)
    stakes: Mapping<Address, AccountStakeInfo>,
    total_stake: Mapping<Address, U512>,
    access_control: AccessControl,
    bid_escrows: Variable<BidEscrows>,
    passive_reputation: PassiveReputation,
}

impl ReputationContractInterface for ReputationContract {
    delegate! {
        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }

        to self.passive_reputation {
            #[call(mint)]
            fn mint_passive(&mut self, recipient: Address, amount: U512);
            #[call(burn)]
            fn burn_passive(&mut self, owner: Address, amount: U512);
            #[call(balance_of)]
            fn passive_balance_of(&self, address: Address) -> U512;
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
        self.balances.set(Balances::default());
    }

    fn mint(&mut self, recipient: Address, amount: U512) {
        self.access_control.ensure_whitelisted();

        let mut balances = self.balances.get_or_revert();
        balances.inc(recipient, amount);
        self.balances.set(balances);

        let (new_supply, is_overflowed) = self.total_supply().overflowing_add(amount);
        if is_overflowed {
            revert(Error::TotalSupplyOverflow);
        }
        self.total_supply.set(new_supply);

        emit(Mint {
            address: recipient,
            amount,
        });
    }

    fn burn(&mut self, owner: Address, amount: U512) {
        self.access_control.ensure_whitelisted();
        let mut balances = self.balances.get_or_revert();
        balances.dec(owner, amount);
        self.balances.set(balances);
        let (new_supply, is_overflowed) = self.total_supply().overflowing_sub(amount);
        if is_overflowed {
            revert(Error::TotalSupplyOverflow);
        }

        self.total_supply.set(new_supply);

        // Emit Burn event.
        emit(Burn {
            address: owner,
            amount,
        });
    }

    fn total_supply(&self) -> U512 {
        self.total_supply.get().unwrap_or_default()
    }

    fn balance_of(&self, address: Address) -> U512 {
        self.balances.get_or_revert().get(&address)
    }

    fn stake_voting(&mut self, voter: Address, voting_id: VotingId, choice: Choice, amount: U512) {
        if amount.is_zero() {
            revert(Error::ZeroStake);
        }
        self.access_control.ensure_whitelisted();
        self.assert_available_balance(voter, amount);
        let mut stake_info = self.stakes_info(voter);
        stake_info.add_stake_from_voting(caller(), voting_id, choice, amount);
        self.stakes.set(&voter, stake_info);

        self.inc_total_stake(voter, amount);

        // // Emit Stake event.
        // emit(Stake {
        //     voter,
        //     voting_id,
        //     choice,
        //     amount,
        // });
    }

    fn unstake_voting(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();

        let mut stake_info = self.stakes_info(voter);
        let amount = stake_info.remove_stake_from_voting(caller(), voting_id);
        self.stakes.set(&voter, stake_info);

        // // Decrement total stake.
        self.dec_total_stake(voter, amount);
    }

    fn stake_bid(&mut self, voter: Address, bid_id: BidId, amount: U512) {
        if amount.is_zero() {
            revert(Error::ZeroStake);
        }
        let bid_escrow_contract = caller();
        self.access_control.ensure_whitelisted();
        self.assert_available_balance(voter, amount);
        let mut stake_info = self.stakes_info(voter);
        stake_info.add_stake_from_bid(bid_escrow_contract, bid_id, amount);
        self.stakes.set(&voter, stake_info);

        self.inc_total_stake(voter, amount);

        // Record BidEscrow address.
        let mut bid_escrows = self.bid_escrows.get().unwrap_or_default();
        bid_escrows.add(bid_escrow_contract);
        self.bid_escrows.set(bid_escrows);

        // // Emit Stake event.
        // emit(Stake {
        //     voter,
        //     bid_id,
        //     choice,
        //     amount,
        // });
    }

    fn unstake_bid(&mut self, voter: Address, bid_id: BidId) {
        self.access_control.ensure_whitelisted();

        let mut stake_info = self.stakes_info(voter);
        let amount = stake_info.remove_stake_from_bid(caller(), bid_id);
        self.stakes.set(&voter, stake_info);

        // // Decrement total stake.
        self.dec_total_stake(voter, amount);
    }

    fn get_stake(&self, address: Address) -> U512 {
        self.total_stake.get(&address).unwrap_or_default()
    }

    fn all_balances(&self) -> (U512, Balances) {
        (self.total_supply(), self.balances.get_or_revert())
    }

    fn partial_balances(&self, addresses: Vec<Address>) -> (U512, Balances) {
        let mut balances = Balances::default();
        let mut partial_supply = U512::zero();
        for address in addresses {
            let balance = self.balance_of(address);
            balances.set(address, balance);
            partial_supply += balance;
        }
        (partial_supply, balances)
    }

    fn bulk_mint_burn(&mut self, mints: BTreeMap<Address, U512>, burns: BTreeMap<Address, U512>) {
        self.access_control.ensure_whitelisted();

        let mut total_supply = self.total_supply();
        let mut balances = self.balances.get_or_revert();
        for (address, amount) in mints {
            balances.inc(address, amount);
            // self.unstake_voting(address, voting_id);
            total_supply += amount;
        }
        for (address, amount) in burns {
            balances.dec(address, amount);
            // self.unstake_voting(address, voting_id);
            total_supply -= amount;
        }

        self.balances.set(balances);
        self.total_supply.set(total_supply);
    }

    fn burn_all(&mut self, owner: Address) {
        let balance = self.balance_of(owner);
        self.burn(owner, balance);
    }

    fn stakes_info(&self, address: Address) -> AccountStakeInfo {
        self.stakes.get(&address).unwrap_or_default()
    }
}

impl ReputationContract {
    fn inc_total_stake(&mut self, account: Address, amount: U512) {
        self.total_stake
            .set(&account, self.get_stake(account) + amount);
    }

    fn dec_total_stake(&mut self, account: Address, amount: U512) {
        self.total_stake
            .set(&account, self.get_stake(account) - amount);
    }

    fn assert_available_balance(&mut self, voter: Address, amount: U512) {
        if amount > self.balance_of(voter).saturating_sub(self.get_stake(voter)) {
            revert(Error::InsufficientBalance);
        }
    }
}

impl ReputationContractCaller {
    /// Indicates whether balance of the `address` is greater than 0.
    pub fn has_reputation(&self, address: &Address) -> bool {
        !self.balance_of(*address).is_zero()
    }
}

pub enum CSPRRedistributionMode {
    OnlyVoters,
    AllVAs,
}

pub struct CSPRRedistribution {
    pub mode: CSPRRedistributionMode,
    pub purse: URef,
}

#[derive(Default, Debug, FromBytes, ToBytes, CLTyped)]
pub struct AccountStakeInfo {
    stakes_from_voting: BTreeMap<(Address, VotingId), (Choice, U512)>,
    stakes_from_bid: BTreeMap<(Address, BidId), U512>,
}

impl AccountStakeInfo {
    fn add_stake_from_voting(
        &mut self,
        operator: Address,
        voting_id: VotingId,
        choice: Choice,
        amount: U512,
    ) {
        let result = self
            .stakes_from_voting
            .insert((operator, voting_id), (choice, amount));
        if result.is_some() {
            revert(Error::CannotStakeTwice)
        }
    }

    fn add_stake_from_bid(&mut self, operator: Address, bid_id: BidId, amount: U512) {
        let result = self.stakes_from_bid.insert((operator, bid_id), amount);
        if result.is_some() {
            revert(Error::CannotStakeTwice)
        }
    }

    fn remove_stake_from_voting(&mut self, operator: Address, voting_id: VotingId) -> U512 {
        let key = (operator, voting_id);
        match self.stakes_from_voting.remove(&key) {
            Some((_, amount)) => amount,
            None => revert(Error::VotingStakeDoesntExists),
        }
    }

    fn remove_stake_from_bid(&mut self, operator: Address, bid_id: BidId) -> U512 {
        let key = (operator, bid_id);
        match self.stakes_from_bid.remove(&key) {
            Some(amount) => amount,
            None => revert(Error::BidStakeDoesntExists),
        }
    }

    pub fn get_voting_stakes_origins(&self) -> Vec<(Address, VotingId)> {
        self.stakes_from_voting.keys().cloned().collect()
    }

    pub fn get_bids_stakes_origins(&self) -> Vec<(Address, BidId)> {
        self.stakes_from_bid.keys().cloned().collect()
    }
}

#[derive(Default, Debug, FromBytes, ToBytes, CLTyped)]
pub struct Balances {
    pub balances: BTreeMap<Address, U512>,
}

impl Balances {
    pub fn get(&self, address: &Address) -> U512 {
        self.balances.get(address).cloned().unwrap_or_default()
    }

    pub fn set(&mut self, address: Address, amount: U512) {
        if amount.is_zero() {
            self.balances.remove(&address);
        } else {
            self.balances.insert(address, amount);
        }
    }

    pub fn inc(&mut self, address: Address, amount: U512) {
        self.set(address, self.get(&address) + amount);
    }

    pub fn dec(&mut self, address: Address, amount: U512) {
        self.set(address, self.get(&address) - amount);
    }

    pub fn rem(&mut self, address: Address) {
        self.balances.remove(&address);
    }
}

#[derive(Default, Debug, FromBytes, ToBytes, CLTyped)]
pub struct BidEscrows {
    addresses: BTreeSet<Address>,
}

impl BidEscrows {
    pub fn add(&mut self, address: Address) {
        self.addresses.insert(address);
    }

    pub fn list(&self) -> &BTreeSet<Address> {
        &self.addresses
    }
}

pub mod events {
    use casper_dao_utils::{casper_dao_macros::Event, Address};
    use casper_types::U512;

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Burn {
        pub address: Address,
        pub amount: U512,
    }

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Mint {
        pub address: Address,
        pub amount: U512,
    }
}
