use std::{
    collections::BTreeMap,
    ops::{AddAssign, SubAssign},
};

use crate::modules::AccessControl;
use crate::utils::Error;
use odra::{
    contract_env,
    types::{event::OdraEvent, Address, Balance},
    Iter, List, Mapping, UnwrapOrRevert, Variable,
};

use super::token::events::{Burn, Mint};

/// A module that stores information about the users' token balances and the total token supply.
///
/// In the system occurs two types of balances:
/// 1. "Real balance" - the actual tokens that a user posses.
/// 2. "Passive balance" - a potential balance that applies to a user who is not eligible to have "real" tokens.
/// If an Address owns a "passive token", it means he's impacted the system (eg. have done a job).
///
/// Having both types of balances allows for keeping track of the total value of the system.
#[odra::module(events = [Mint, Burn])]
pub struct BalanceStorage {
    balances: Mapping<Address, Balance>,
    holders: List<Address>,
    total_supply: TotalSupply,
    access_control: AccessControl,
}

impl BalanceStorage {
    /// Increases the user's balance and the total supply.
    /// If the call succeeds, emits a [Mint] event.
    ///
    /// # Arguments
    ///
    /// * `recipient` - the token recipient address.
    /// * `amount` - the number of tokens to be minted.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn mint(&mut self, recipient: Address, amount: Balance) {
        self.access_control.ensure_whitelisted();
        self.inc_balance(&recipient, amount);
        self.total_supply += amount;

        self.holders.push(recipient);

        Mint {
            address: recipient,
            amount,
        }
        .emit();
    }

    /// Decreases the user's balance and the total supply.
    /// If the call succeeds, emits a [Burn] event.
    ///
    /// # Arguments
    ///
    /// * `owner` - the account address of which token are burned.
    /// * `amount` - the number of tokens to be burned.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn burn(&mut self, owner: Address, amount: Balance) {
        self.access_control.ensure_whitelisted();
        self.dec_balance(&owner, amount);
        self.total_supply -= amount;

        Burn {
            address: owner,
            amount,
        }
        .emit();
    }

    /// Performs mint and/or burn for multiple accounts at once.
    /// If the call succeeds, emits a [Burn] event.
    ///
    /// # Arguments
    ///
    /// * `mints` - a map of addresses and amounts to mint.
    /// * `burns` - a map of addresses and amounts to burn.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn bulk_mint_burn(
        &mut self,
        mints: BTreeMap<Address, Balance>,
        burns: BTreeMap<Address, Balance>,
    ) {
        self.access_control.ensure_whitelisted();

        let mut total_supply = self.total_supply();
        for (address, amount) in mints {
            self.inc_balance(&address, amount);
            total_supply += amount;
        }
        for (address, amount) in burns {
            self.dec_balance(&address, amount);
            total_supply -= amount;
        }

        self.total_supply.set(total_supply);
    }

    /// Burns all tokens of the given account.
    /// See [`Self::burn()`].
    ///
    /// # Arguments
    ///
    /// * `owner` - the address of the tokens owner.
    ///
    /// # Errors
    ///
    /// [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) if called by a not whitelisted account.
    pub fn burn_all(&mut self, owner: Address) {
        self.access_control.ensure_whitelisted();

        let balance = self.balance_of(owner);
        self.burn(owner, balance);
    }

    /// Returns an iterator of token holders.
    pub fn holders(&self) -> Iter<Address> {
        self.holders.iter()
    }

    /// Returns the current balance of the given account address.
    pub fn balance_of(&self, address: Address) -> Balance {
        self.balances.get(&address).unwrap_or_default()
    }

    /// Returns the total token supply.
    pub fn total_supply(&self) -> Balance {
        self.total_supply.value()
    }
}

impl BalanceStorage {
    fn set_balance(&mut self, owner: &Address, new_balance: Balance) {
        self.balances.set(owner, new_balance);
    }

    fn inc_balance(&mut self, owner: &Address, amount: Balance) {
        let balance = self.balances.get(owner).unwrap_or_default();
        let new_balance = balance
            .checked_add(amount)
            .unwrap_or_revert_with(Error::ArithmeticOverflow);

        self.set_balance(owner, new_balance);
    }

    fn dec_balance(&mut self, owner: &Address, amount: Balance) {
        let balance = self.balances.get(owner).unwrap_or_default();
        let new_balance = balance
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::InsufficientBalance);

        self.set_balance(owner, new_balance);
    }
}

/// Wraps `total_supply` and some operations for convenience.
#[odra::module]
pub struct TotalSupply {
    total_supply: Variable<Balance>,
}

impl TotalSupply {
    pub fn value(&self) -> Balance {
        self.total_supply.get().unwrap_or_default()
    }

    pub fn set(&mut self, total_supply: Balance) {
        self.total_supply.set(total_supply);
    }
}

impl AddAssign<Balance> for TotalSupply {
    fn add_assign(&mut self, rhs: Balance) {
        let (new_value, is_overflowed) = self.value().overflowing_add(rhs);
        if is_overflowed {
            contract_env::revert(Error::TotalSupplyOverflow)
        }
        self.set(new_value);
    }
}

impl SubAssign<Balance> for TotalSupply {
    fn sub_assign(&mut self, rhs: Balance) {
        let (new_value, is_overflowed) = self.value().overflowing_sub(rhs);
        if is_overflowed {
            contract_env::revert(Error::TotalSupplyOverflow)
        }
        self.total_supply.set(new_value);
    }
}
