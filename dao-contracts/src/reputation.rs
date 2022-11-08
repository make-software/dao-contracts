use std::collections::BTreeMap;

use crate::{
    bid::types::BidId,
    voting::{Choice, VotingId},
};
use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, CLTyped, FromBytes, Instance, ToBytes},
    casper_env::{self, caller, emit, revert},
    math::{add_to_balance, rem_from_balance},
    Address, Error, Mapping, Variable, VecMapping,
};
use casper_types::{URef, U256};
use delegate::delegate;

use self::events::{Burn, Mint};

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
    /// increment the total supply. Only whitelisted addresses are permited to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Mint`](casper_dao_contracts::reputation::events::Mint) event.
    fn mint(&mut self, recipient: Address, amount: U256);

    /// Burn existing tokens. Remove `amount` of existing tokens from the balance of the `owner`
    /// and decrement the total supply. Only whitelisted addresses are permited to call this
    /// method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Burn`](casper_dao_contracts::reputation::events::Burn) event.
    fn burn(&mut self, owner: Address, amount: U256);

    /// Change ownership of the contract. Transfer the ownership to the `owner`. Only current owner
    /// is permited to call this method.
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
    fn total_supply(&self) -> U256;

    /// Returns the current token balance of the given address.
    fn balance_of(&self, address: Address) -> U256;

    /// Checks whether the given address is added to the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;

    /// Stakes the reputation for a given voting and choice.
    fn stake_voting(&mut self, voter: Address, voting_id: VotingId, choice: Choice, amount: U256);

    fn unstake_voting(&mut self, voter: Address, voting_id: VotingId);

    fn stake_bid(&mut self, voter: Address, bid_id: BidId, amount: U256);

    fn unstake_bid(&mut self, voter: Address, bid_id: BidId);

    fn get_stake(&self, address: Address) -> U256;

    fn all_balances(&self) -> (U256, Balances);
    
    // Redistributes the reputation based on the voting summary
    fn redistribute(&mut self, voting_id: VotingId, result: Choice);
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
#[derive(Instance)]
pub struct ReputationContract {
    total_supply: Variable<U256>,
    balances: Variable<Balances>,
    // (owner, staker, voting) -> (stake, choice)
    stakes: Mapping<Address, AccountStakeInfo>,
    total_stake: Mapping<Address, U256>,
    pub access_control: AccessControl,
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
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
        self.balances.set(Balances::default());
    }

    fn total_supply(&self) -> U256 {
        self.total_supply.get().unwrap_or_default()
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.balances.get_or_revert().get(&address)
    }

    fn all_balances(&self) -> (U256, Balances) {
        (self.total_supply(), self.balances.get_or_revert())
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        self.access_control.ensure_whitelisted();

        let mut balances = self.balances.get_or_revert();
        balances.inc(recipient, amount);
        self.balances.set(balances);

        let (new_supply, is_overflowed) = self.total_supply().overflowing_add(amount);
        if is_overflowed {
            casper_env::revert(Error::TotalSupplyOverflow);
        }
        self.total_supply.set(new_supply);

        emit(Mint {
            address: recipient,
            amount,
        });
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        self.access_control.ensure_whitelisted();

        let mut balances = self.balances.get_or_revert();
        balances.dec(owner, amount);
        self.balances.set(balances);
    
        let (new_supply, is_overflowed) = self.total_supply().overflowing_sub(amount);
        if is_overflowed {
            casper_env::revert(Error::TotalSupplyOverflow);
        }
        self.total_supply.set(new_supply);

        // Emit Burn event.
        emit(Burn {
            address: owner,
            amount,
        });
    }

    fn get_stake(&self, address: Address) -> U256 {
        self.total_stake.get(&address).unwrap_or_default()
    }

    fn stake_voting(&mut self, voter: Address, voting_id: VotingId, choice: Choice, amount: U256) {
        self.access_control.ensure_whitelisted();

        let mut stake_info = self.stake_info(&voter);
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

        let mut stake_info = self.stake_info(&voter);
        let amount = stake_info.remove_stake_from_voting(caller(), voting_id);
        self.stakes.set(&voter, stake_info);

        // // Decrement total stake.
        self.dec_total_stake(voter, amount);
    }

    fn stake_bid(&mut self, voter: Address, bid_id: BidId, amount: U256) {
        self.access_control.ensure_whitelisted();

        let mut stake_info = self.stake_info(&voter);
        stake_info.add_stake_from_bid(caller(), bid_id, amount);
        self.stakes.set(&voter, stake_info);

        self.inc_total_stake(voter, amount);

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

        let mut stake_info = self.stake_info(&voter);
        let amount = stake_info.remove_stake_from_bid(caller(), bid_id);
        self.stakes.set(&voter, stake_info);

        // // Decrement total stake.
        self.dec_total_stake(voter, amount);
    }

    fn redistribute(&mut self, voting_id: VotingId, result: Choice) {
        let stakes: BTreeMap<Account, (U256, Choice)> = self.stakes_info.get_or_revert().stakes_for(voting_id);
        let total_stake: U256 = self.stakes_info.get_or_revert().total_stake(voting_id);
        let in_favor_stake: U256 = self.stakes_info.get_or_revert().in_favor_stake(voting_id);
        let against_stake: U256 = self.stakes_info.get_or_revert().against_stake(voting_id);

        for (account, (stake, choice)) in stakes.iter() {
            
        } 
    }
}

impl ReputationContract {
    fn stake_info(&self, address: &Address) -> StakeInfo {
        self.stakes.get(address).unwrap_or_default()
    }

    fn inc_total_stake(&mut self, account: Address, amount: U256) {
        self.total_stake
            .set(&account, self.get_stake(account) + amount);
    }

    fn dec_total_stake(&mut self, account: Address, amount: U256) {
        self.total_stake
            .set(&account, self.get_stake(account) - amount);
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

#[derive(Default, FromBytes, ToBytes, CLTyped)]
struct AccountStakeInfo {
    stakes_from_voting: BTreeMap<(Address, VotingId), (Choice, U256)>,
    stakes_from_bid: BTreeMap<(Address, BidId), U256>,
}

impl AccountStakeInfo {
    fn add_stake_from_voting(
        &mut self,
        operator: Address,
        voting_id: VotingId,
        choice: Choice,
        amount: U256,
    ) {
        let result = self
            .stakes_from_voting
            .insert((operator, voting_id), (choice, amount));
        if result.is_some() {
            revert(Error::CannotStakeTwice)
        }
    }

    fn add_stake_from_bid(&mut self, operator: Address, bid_id: BidId, amount: U256) {
        let result = self.stakes_from_bid.insert((operator, bid_id), amount);
        if result.is_some() {
            revert(Error::CannotStakeTwice)
        }
    }

    fn remove_stake_from_voting(&mut self, operator: Address, voting_id: VotingId) -> U256 {
        let key = (operator, voting_id);
        match self.stakes_from_voting.remove(&key) {
            Some((_, amount)) => amount,
            None => revert(Error::StakeDoesntExists),
        }
    }

    fn remove_stake_from_bid(&mut self, operator: Address, bid_id: BidId) -> U256 {
        let key = (operator, bid_id);
        self.stakes_from_bid
            .remove(&key)
            .unwrap_or_revert_with(Error::StakeDoesntExists)
    }
}

struct StakesInfo {}

#[derive(Default, FromBytes, ToBytes, CLTyped)]
pub struct Balances {
    pub balances: BTreeMap<Address, U256>
}

impl Balances {
    pub fn get(&self, address: &Address) -> U256 {
        self.balances.get(address).cloned().unwrap_or_default()
    }

    pub fn set(&mut self, address: Address, amount: U256) {
        if amount.is_zero() {
            self.balances.remove(&address);
        } else {
            self.balances.insert(address, amount);
        }
    }

    pub fn inc(&mut self, address: Address, amount: U256) {
        self.set(address, self.get(&address) + amount);
    }

    pub fn dec(&mut self, address: Address, amount: U256) {
        self.set(address, self.get(&address) - amount);
    }
}

pub mod events {
    use casper_dao_utils::{casper_dao_macros::Event, Address};
    use casper_types::U256;

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Burn {
        pub address: Address,
        pub amount: U256,
    }

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Mint {
        pub address: Address,
        pub amount: U256,
    }
}
