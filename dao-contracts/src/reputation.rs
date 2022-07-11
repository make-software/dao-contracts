use casper_dao_erc20::{ERC20Interface, ERC20};
use casper_dao_modules::AccessControl;
use casper_dao_utils::casper_dao_macros::Event;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address, Mapping,
};
use casper_types::U256;
use delegate::delegate;

/// Event thrown when debt is lowered
#[derive(Debug, PartialEq, Event)]
pub struct DebtPaid {
    pub owner: Address,
    pub amount: U256,
    pub debt: U256,
}

/// Event thrown when debt is made
#[derive(Debug, PartialEq, Event)]
pub struct DebtIncreased {
    pub owner: Address,
    pub amount: U256,
    pub debt: U256,
}

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
    /// It emits [`Mint`](casper_dao_modules::events::Mint) event.
    fn mint(&mut self, recipient: Address, amount: U256);

    /// Burn existing tokens. Remove `amount` of existing tokens from the balance of the `owner`
    /// and decrement the total supply. Only whitelisted addresses are permited to call this
    /// method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Burn`](casper_dao_modules::events::Burn) event.
    fn burn(&mut self, owner: Address, amount: U256);

    /// Transfer `amount` of tokens from `owner` to `recipient`. Only whitelisted addresses are
    /// permited to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance)
    /// if `recipient`'s balance is less then `amount`.
    ///
    /// It emits [`Transfer`](casper_dao_modules::events::Transfer) event.
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

    /// Returns the amount of the debt the owner has
    fn debt(&self, owner: Address) -> U256;
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
#[derive(Instance)]
pub struct ReputationContract {
    pub token: ERC20,
    pub access_control: AccessControl,
    pub debt: Mapping<Address, U256>,
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

    delegate! {
        to self.token {
            fn balance_of(&self, address: Address) -> U256;
            fn total_supply(&self) -> U256;
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        self.access_control.ensure_whitelisted();
        let mut debt = self.debt(recipient);
        let debt_paid;
        if debt > U256::zero() {
            match debt.checked_sub(amount) {
                // minting will cover whole debt
                None => {
                    self.debt.set(&recipient, U256::zero());
                    debt_paid = debt;
                    self.token.mint(recipient, amount - debt);
                    debt = U256::zero();
                }
                // minting will not cover the debt
                Some(left) => {
                    self.debt.set(&recipient, left);
                    debt_paid = left;
                    debt = left;
                }
            }

            DebtPaid {
                owner: recipient,
                amount: debt_paid,
                debt,
            }
            .emit();
        } else {
            self.token.mint(recipient, amount);
        }
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        self.access_control.ensure_whitelisted();
        let balance = self.token.balance_of(owner);
        match balance.checked_sub(amount) {
            // we need to burn more than the owner has
            None => {
                self.token.burn(owner, balance);
                let debt = self.debt(owner);
                let to_add = amount.saturating_sub(balance);
                let new_debt = debt + to_add;
                self.debt.set(&owner, new_debt);
                DebtIncreased {
                    owner,
                    amount: to_add,
                    debt: new_debt,
                }
                .emit();
            }
            // we simply burn
            Some(_) => {
                self.token.burn(owner, amount);
            }
        }
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        self.access_control.ensure_whitelisted();
        self.token.raw_transfer(owner, recipient, amount);
    }

    fn debt(&self, owner: Address) -> U256 {
        match self.debt.get(&owner) {
            None => U256::zero(),
            Some(debt) => debt,
        }
    }
}

impl ReputationContractCaller {
    /// Indicates whether balance of the `address` is greater than 0.
    pub fn has_reputation(&self, address: &Address) -> bool {
        !self.balance_of(*address).is_zero()
    }
}
