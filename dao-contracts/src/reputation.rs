use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, caller, emit},
    math::{add_to_balance, rem_from_balance},
    Address, Error, Mapping, Variable,
};
use casper_types::U256;
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

    /// Transfer `amount` of tokens from `owner` to `recipient`. Only whitelisted addresses are
    /// permited to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance)
    /// if `recipient`'s balance is less then `amount`.
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);

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

    /// Returns the amount of the debt the owner has.
    fn debt(&self, owner: Address) -> U256;
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
#[derive(Instance)]
pub struct ReputationContract {
    total_supply: Variable<U256>,
    balances: Mapping<Address, (bool, U256)>,
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
    }

    fn total_supply(&self) -> U256 {
        self.total_supply.get().unwrap_or_default()
    }

    fn balance_of(&self, address: Address) -> U256 {
        match self.get_signed_balance(address) {
            (true, value) => value,
            (false, _) => U256::zero(),
        }
    }

    fn debt(&self, address: Address) -> U256 {
        match self.get_signed_balance(address) {
            (true, _) => U256::zero(),
            (false, value) => value,
        }
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        self.access_control.ensure_whitelisted();

        // Load a balance of the account.
        let signed_balance = self.get_signed_balance(recipient);
        let (is_positive, balance) = signed_balance;

        // Increase total_supply by the amount above the debt.
        // This prevents total_supply from overflowing.
        let real_increase_amount = if is_positive {
            amount
        } else if amount > balance {
            amount - balance
        } else {
            U256::zero()
        };

        let (new_supply, is_overflowed) = self.total_supply().overflowing_add(real_increase_amount);
        if is_overflowed {
            casper_env::revert(Error::TotalSupplyOverflow);
        }
        self.total_supply.set(new_supply);

        // Increase the balance of the account.
        let new_balance = add_to_balance(signed_balance, amount);
        self.balances.set(&recipient, new_balance);

        emit(Mint {
            address: recipient,
            amount,
        });
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        self.access_control.ensure_whitelisted();

        // Load a balance of the account.
        let signed_balance = self.get_signed_balance(owner);
        let (is_positive, balance) = signed_balance;

        // Reduce the balance of the account.
        let new_balance = rem_from_balance(signed_balance, amount);
        self.balances.set(&owner, new_balance);

        // Decrease total_supply by only decreased positive balance of owner.
        // This prevents total_supply of getting negative.
        if is_positive {
            let total_supply = self.total_supply();
            if amount > balance {
                self.total_supply.set(total_supply - balance);
            } else {
                self.total_supply.set(total_supply - amount);
            }
        }

        // Emit Burn event.
        emit(Burn {
            address: owner,
            amount,
        });
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        self.access_control.ensure_whitelisted();

        // Load the balance of the owner.
        let owner_signed_balance = self.get_signed_balance(owner);
        let (is_positive_owner_balance, owner_balance) = owner_signed_balance;

        // Check if the owner has sufficient balance.
        if !is_positive_owner_balance || owner_balance < amount {
            casper_env::revert(Error::InsufficientBalance)
        }

        // Load the balance of the recipient.
        let recipient_signed_balance = self.get_signed_balance(recipient);

        // Settle the transfer.
        self.balances
            .set(&owner, rem_from_balance(owner_signed_balance, amount));
        self.balances
            .set(&recipient, add_to_balance(recipient_signed_balance, amount));
    }
}

impl ReputationContract {
    fn get_signed_balance(&self, address: Address) -> (bool, U256) {
        self.balances.get(&address).unwrap_or((true, U256::zero()))
    }
}

impl ReputationContractCaller {
    /// Indicates whether balance of the `address` is greater than 0.
    pub fn has_reputation(&self, address: &Address) -> bool {
        !self.balance_of(*address).is_zero()
    }
}

pub mod events {
    use casper_dao_utils::{casper_dao_macros::Event, Address};
    use casper_types::U256;

    #[derive(Debug, PartialEq, Event)]
    pub struct Burn {
        pub address: Address,
        pub amount: U256,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct Mint {
        pub address: Address,
        pub amount: U256,
    }
}
