use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::Instance,
    Address,
    Error,
    Mapping,
};
use casper_types::U512;

/// The PassiveReputation module.
///
/// Stores information about potential balances of the users who are not eligible to have reputation tokens.
/// If an Address owns a passive reputation, it means they have impacted the system (eg. have done a job). 
/// These balances allow for keeping track of the total value of the system.
#[derive(Instance)]
pub struct PassiveReputation {
    balances: Mapping<Address, U512>,
    #[scoped = "contract"]
    pub access_control: AccessControl,
}

impl PassiveReputation {
    pub fn mint(&mut self, recipient: Address, amount: U512) {
        self.access_control.ensure_whitelisted();

        let balance = self.balances.get(&recipient).unwrap_or_default();
        let new_balance = balance
            .checked_add(amount)
            .unwrap_or_revert_with(Error::ArithmeticOverflow);

        self.set_balance(&recipient, new_balance);
    }

    pub fn burn(&mut self, owner: Address, amount: U512) {
        self.access_control.ensure_whitelisted();

        let balance = self.balances.get(&owner).unwrap_or_default();
        let new_balance = balance
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::InsufficientBalance);

        self.set_balance(&owner, new_balance);
    }

    pub fn balance_of(&self, address: Address) -> U512 {
        self.balances.get(&address).unwrap_or_default()
    }

    fn set_balance(&mut self, owner: &Address, new_balance: U512) {
        self.balances.set(owner, new_balance);
    }
}
