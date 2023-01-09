use std::collections::BTreeMap;

use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    casper_env::{emit, revert},
    Address,
    Error,
    Iter,
    Mapping,
    OrderedCollection,
    Set,
    Variable,
};
use casper_types::U512;

use super::token::events::{Burn, Mint};

/// The PassiveReputation module.
///
/// Stores information about potential balances of the users who are not eligible to have reputation tokens.
/// If an Address owns a passive reputation, it means they have impacted the system (eg. have done a job).
/// These balances allow for keeping track of the total value of the system.
#[derive(Instance)]
pub struct BalanceStorage {
    balances: Mapping<Address, U512>,
    holders: OrderedCollection<Address>,
    total_supply: Variable<U512>,
    #[scoped = "contract"]
    access_control: AccessControl,
}

impl BalanceStorage {
    pub fn mint(&mut self, recipient: Address, amount: U512) {
        self.access_control.ensure_whitelisted();
        self.inc_balance(&recipient, amount);
        self.inc_total_supply(amount);

        self.holders.add(recipient);

        emit(Mint {
            address: recipient,
            amount,
        });
    }

    pub fn burn(&mut self, owner: Address, amount: U512) {
        self.access_control.ensure_whitelisted();
        self.dec_balance(&owner, amount);
        self.dec_total_supply(amount);

        // Emit Burn event.
        emit(Burn {
            address: owner,
            amount,
        });
    }

    pub fn balance_of(&self, address: Address) -> U512 {
        self.balances.get(&address).unwrap_or_default()
    }

    pub fn total_supply(&self) -> U512 {
        self.total_supply.get().unwrap_or_default()
    }

    pub fn bulk_mint_burn(
        &mut self,
        mints: BTreeMap<Address, U512>,
        burns: BTreeMap<Address, U512>,
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

    pub fn burn_all(&mut self, owner: Address) {
        let balance = self.balance_of(owner);
        self.burn(owner, balance);
    }

    pub fn holders(&self) -> Iter<Address> {
        self.holders.iter()
    }

    fn set_balance(&mut self, owner: &Address, new_balance: U512) {
        self.balances.set(owner, new_balance);
    }

    fn inc_balance(&mut self, owner: &Address, amount: U512) {
        let balance = self.balances.get(owner).unwrap_or_default();
        let new_balance = balance
            .checked_add(amount)
            .unwrap_or_revert_with(Error::ArithmeticOverflow);

        self.set_balance(owner, new_balance);
    }

    fn dec_balance(&mut self, owner: &Address, amount: U512) {
        let balance = self.balances.get(owner).unwrap_or_default();
        let new_balance = balance
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::InsufficientBalance);

        self.set_balance(owner, new_balance);
    }

    fn inc_total_supply(&mut self, amount: U512) {
        let (new_supply, is_overflowed) = self.total_supply().overflowing_add(amount);
        if is_overflowed {
            revert(Error::TotalSupplyOverflow);
        }
        self.total_supply.set(new_supply);
    }

    fn dec_total_supply(&mut self, amount: U512) {
        let (new_supply, is_overflowed) = self.total_supply().overflowing_sub(amount);
        if is_overflowed {
            revert(Error::TotalSupplyOverflow);
        }
        self.total_supply.set(new_supply);
    }
}
