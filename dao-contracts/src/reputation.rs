use casper_dao_erc20::{ERC20Interface, ERC20};
use casper_dao_modules::{Owner, Whitelist};
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    Address,
};
use casper_types::U256;

// TODO: Put it lower.
//
// Interface of the Reputation Contract.
//
// It should be implemented by [`ReputationContract`], [`ReputationContractCaller`]
// and [`ReputationContractTest`].

#[casper_contract_interface]
pub trait ReputationContractInterface {
    /// Constructor method.
    ///
    /// It initializes contract elements:
    /// * Set [`caller`] as the owner of the contract.
    /// * Add [`caller`] to the whitelist.
    ///
    /// It emits [`OwnerChanged`](casper_dao_utils::owner::events::OwnerChanged),
    /// [`AddedToWhitelist`](casper_dao_utils::whitelist::events::AddedToWhitelist) events.
    fn init(&mut self);

    /// Mint new tokens. Add `amount` of new tokens to the balance of the `recipient` and
    /// increment the total supply. Only whitelisted addresses are permited to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Transfer`](casper_dao_erc20::events::Transfer) event.
    fn mint(&mut self, recipient: Address, amount: U256);

    /// Burn existing tokens. Remove `amount` of existing tokens from the balance of the `owner`
    /// and decrement the total supply. Only whitelisted addresses are permited to call this
    /// method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Transfer`](casper_dao_erc20::events::Transfer) event.
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
    /// It emits [`Transfer`](casper_dao_erc20::events::Transfer) event.
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);

    /// Change ownership of the contract. Transfer the ownership to the `owner`. Only current owner
    /// is permited to call this method.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It emits [`OwnerChanged`](casper_dao_utils::owner::events::OwnerChanged),
    /// [`AddedToWhitelist`](casper_dao_utils::whitelist::events::AddedToWhitelist) and
    /// [`RemovedFromWhitelist`](casper_dao_utils::whitelist::events::RemovedFromWhitelist)
    /// events.
    fn change_ownership(&mut self, owner: Address);

    /// Add new address to the whitelist.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It emits [`AddedToWhitelist`](casper_dao_utils::whitelist::events::AddedToWhitelist) event.
    fn add_to_whitelist(&mut self, address: Address);

    /// Remove address from the whitelist.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It emits [`RemovedFromWhitelist`](casper_dao_utils::whitelist::events::RemovedFromWhitelist)
    /// event.
    fn remove_from_whitelist(&mut self, address: Address);

    /// Get the current owner of the contract. None if not owner is not set.
    fn get_owner(&self) -> Option<Address>;

    /// Get the count of all tokens.
    fn total_supply(&self) -> U256;

    /// Get the current balance of the `address`.
    fn balance_of(&self, address: Address) -> U256;

    /// Check if given `address` is on the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
#[derive(Instance)]
pub struct ReputationContract {
    pub token: ERC20,
    pub owner: Owner,
    pub whitelist: Whitelist,
}

impl ReputationContractInterface for ReputationContract {
    fn init(&mut self) {
        let deployer = caller();
        self.owner.init(deployer);
        self.whitelist.add_to_whitelist(deployer);
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.mint(&recipient, amount);
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.burn(&owner, amount);
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.raw_transfer(owner, recipient, amount);
    }

    fn change_ownership(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.change_ownership(owner);
        self.whitelist.add_to_whitelist(owner);
        self.whitelist.remove_from_whitelist(caller());
    }

    fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
    }

    fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }

    fn total_supply(&self) -> U256 {
        self.token.total_supply()
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.token.balance_of(address)
    }

    fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }
}
