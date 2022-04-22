use casper_dao_modules::{Owner, Record, Repository, Whitelist};
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert},
    Address, Error,
};
use casper_types::bytesrepr::{Bytes, FromBytes};

// Interface of the Variable Repository Contract.
//
// It should be implemented by [`VariableRepositoryContract`], [`VariableRepositoryContractCaller`]
// and [`VariableRepositoryContractTest`].
#[casper_contract_interface]
pub trait VariableRepositoryContractInterface {
    /// Constructor method.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Events dictionary.
    /// * Sets the default configuration of the [`Repository`](casper_dao_modules::Repository)
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    /// * multiple [`ValueUpdated`](casper_dao_modules::events::ValueUpdated) events,
    /// one per value of the default repository configuration.
    fn init(&mut self);

    /// Changes the ownership of the contract. Transfers the ownership to the `owner`.
    /// Only the current owner is permited to call this method.
    ///
    /// # Events
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist).
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if the caller
    /// is not the current owner.
    fn change_ownership(&mut self, owner: Address);

    /// Adds a new address to the whitelist.
    ///
    /// # Events
    /// Emits [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist) event.
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    fn add_to_whitelist(&mut self, address: Address);

    /// Remove address from the whitelist.
    ///
    /// # Events
    /// Emits [`RemovedFromWhitelist`](casper_dao_modules::events::RemovedFromWhitelist)
    /// event.
    ///
    /// # Errors
    /// Throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    fn remove_from_whitelist(&mut self, address: Address);

    /// Inserts or updates the value under the given key.
    ///
    /// # Note
    /// * The activation time is represented as a unix timestamp.
    /// * If the activitation time is `None` the value is updated immediately.
    /// * If some future time in the future is passed as an argument, the [`Self::get`] function
    /// returns the previously set value.
    ///
    /// # Events
    /// * Emits [`ValueUpdated`](casper_dao_modules::events::ValueUpdated) event.
    ///
    /// # Errors
    /// * Throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if the caller
    /// is not a whitelisted user.
    /// * Throws [`ActivationTimeInPast`](casper_dao_utils::Error::ActivationTimeInPast) if
    /// the activation time has passed already.
    /// * Throws [`ValueNotAvailable`](casper_dao_utils::Error::ValueNotAvailable) on
    /// the future value update if the current value has not been set.
    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>);

    /// Returns the value stored under the given key.
    ///
    /// If the key does not exist, the `None` value is returned.
    fn get(&self, key: String) -> Option<Bytes>;

    /// Returns the full (current and future) value stored under the given key.
    /// See [`Record`](casper_dao_modules::Record).
    ///
    /// If the key does not exist, the `None` value is returned.
    fn get_full_value(&self, key: String) -> Option<Record>;

    /// Returns the value stored under the given index.
    ///
    /// Every freshly added key has the previous key index increased by 1.
    /// The index range is 0 to #keys-1.
    ///
    /// If the given index exceeds #keys-1 the `None` value is returned.
    fn get_key_at(&self, index: u32) -> Option<String>;

    /// Returns the number of existing keys in the [`Repository`](casper_dao_modules::Repository).
    fn keys_count(&self) -> u32;

    /// Returns the address of the current owner.
    fn get_owner(&self) -> Option<Address>;

    /// Checks whether the given address is added to the whitelist.
    fn is_whitelisted(&self, address: Address) -> bool;
}

/// Variable Repository Contract implementation. See [`VariableRepositoryContractInterface`].
#[derive(Instance)]
pub struct VariableRepositoryContract {
    pub owner: Owner,
    pub whitelist: Whitelist,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    fn init(&mut self) {
        let deployer = caller();
        self.owner.init(deployer);
        self.whitelist.add_to_whitelist(deployer);
        self.repository.init();
    }

    fn change_ownership(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.change_ownership(owner);
        self.whitelist.add_to_whitelist(owner);
    }

    fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
    }

    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        self.whitelist.ensure_whitelisted();
        self.repository.update_at(key, value, activation_time);
    }

    fn get(&self, key: String) -> Option<Bytes> {
        self.repository.get(key)
    }

    fn get_full_value(&self, key: String) -> Option<Record> {
        self.repository.get_full_value(key)
    }

    fn get_key_at(&self, index: u32) -> Option<String> {
        self.repository.keys.get(index)
    }

    fn keys_count(&self) -> u32 {
        self.repository.keys.size()
    }

    fn get_owner(&self) -> Option<Address> {
        self.owner.get_owner()
    }

    fn is_whitelisted(&self, address: Address) -> bool {
        self.whitelist.is_whitelisted(&address)
    }
}

impl VariableRepositoryContractCaller {
    /// Retrieves a value for the given key and returns a deserialized struct.
    ///
    /// # Errors
    /// Throws [`ValueNotAvailable`](casper_dao_utils::Error::NotAnOwner) if a value
    /// for the given key does not exist.
    pub fn get_variable<T: FromBytes>(&self, key: &str) -> T {
        let variable = self.get(key.into()).unwrap_or_revert();
        let (variable, bytes) = <T>::from_bytes(&variable).unwrap_or_revert();
        if !bytes.is_empty() {
            revert(Error::ValueNotAvailable)
        }
        variable
    }
}
