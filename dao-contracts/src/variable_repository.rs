use std::collections::BTreeMap;

use casper_dao_modules::{AccessControl, Record, Repository};
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert},
    consts as dao_consts,
    math,
    Address,
    Error,
};
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    U512,
};
use delegate::delegate;

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
    /// See [`AccessControl`](AccessControl::change_ownership())
    fn change_ownership(&mut self, owner: Address);

    /// Adds a new address to the whitelist.
    ///
    /// See [`AccessControl`](AccessControl::add_to_whitelist())
    fn add_to_whitelist(&mut self, address: Address);

    /// Remove address from the whitelist.
    ///
    /// See [`AccessControl`](AccessControl::remove_from_whitelist())
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

    fn all_variables(&self) -> BTreeMap<String, Bytes>;
}

/// Variable Repository Contract implementation. See [`VariableRepositoryContractInterface`].
#[derive(Instance)]
pub struct VariableRepositoryContract {
    pub access_control: AccessControl,
    pub repository: Repository,
}

impl VariableRepositoryContractInterface for VariableRepositoryContract {
    delegate! {
        to self.access_control {
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
        }
    }

    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
        self.repository.init();
    }

    fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        self.access_control.ensure_whitelisted();
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

    fn all_variables(&self) -> BTreeMap<String, Bytes> {
        let mut result: BTreeMap<String, Bytes> = BTreeMap::new();

        for key in 0..self.repository.keys.length.get().unwrap() {
            let repo_key = self.repository.keys.get(key).unwrap();
            let value = self.repository.get(repo_key.clone()).unwrap();
            result.insert(repo_key, value);
        }

        result
    }
}

impl VariableRepositoryContractCaller {
    /// Retrieves the value for the given key and returns a deserialized struct.
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

    /// Retrieves the value stored under the [INFORMAL_VOTING_TIME](dao_consts::INFORMAL_VOTING_TIME) key.
    pub fn informal_voting_time(&self) -> u64 {
        self.get_variable(dao_consts::INFORMAL_VOTING_TIME)
    }

    /// Retrieves the value stored under the [FORMAL_VOTING_TIME](dao_consts::FORMAL_VOTING_TIME) key.
    pub fn formal_voting_time(&self) -> u64 {
        self.get_variable(dao_consts::FORMAL_VOTING_TIME)
    }

    /// Retrieves the value stored under the [REPUTATION_CONVERSION_RATE](dao_consts::REPUTATION_CONVERSION_RATE) key.
    pub fn reputation_conversion_rate(&self) -> U512 {
        self.get_variable(dao_consts::REPUTATION_CONVERSION_RATE)
    }

    /// Retrieves the value stored under the [DEFAULT_POLICING_RATE](dao_consts::DEFAULT_POLICING_RATE) key.
    pub fn default_policing_rate(&self) -> U512 {
        self.get_variable(dao_consts::DEFAULT_POLICING_RATE)
    }

    /// Retrieves a normalized value stored under the [INFORMAL_VOTING_QUORUM](dao_consts::INFORMAL_VOTING_QUORUM) key.
    pub fn informal_voting_quorum(&self, total_onboarded: U512) -> U512 {
        math::promils_of(
            total_onboarded,
            self.get_variable(dao_consts::GOVERNANCE_INFORMAL_QUORUM_RATIO),
        )
        .unwrap_or_revert()
    }

    /// Retrieves a normalized value stored under the [FORMAL_VOTING_QUORUM](dao_consts::FORMAL_VOTING_QUORUM) key.
    pub fn formal_voting_quorum(&self, total_onboarded: U512) -> U512 {
        math::promils_of(
            total_onboarded,
            self.get_variable(dao_consts::GOVERNANCE_FORMAL_QUORUM_RATIO),
        )
        .unwrap_or_revert()
    }

    /// Calculates amount of reputation to be minted
    pub fn reputation_to_mint(&self, cspr_amount: U512) -> U512 {
        math::promils_of(
            cspr_amount,
            self.reputation_conversion_rate(),
        )
        .unwrap_or_revert()
    }

    /// Calculates amount of reputation to be redistributed
    pub fn reputation_to_redistribute(&self, reputation_amount: U512) -> U512 {
        math::promils_of(reputation_amount, self.default_policing_rate()).unwrap_or_revert()
    }

    /// Calculates amount of CSPR to be redistributed
    pub fn cspr_to_redistribute(&self, cspr_amount: U512) -> U512 {
        math::promils_of_u512(
            cspr_amount,
            self.default_policing_rate(),
        )
        .unwrap_or_revert()
    }

    pub fn governance_wallet(&self) -> Address {
        self.get_variable(dao_consts::GOVERNANCE_WALLET_ADDRESS)
    }

    pub fn governance_payment_ratio(&self) -> U512 {
        self.get_variable(dao_consts::GOVERNANCE_PAYMENT_RATIO)
    }

    pub fn payment_for_governance(&self, cspr_amount: U512) -> U512 {
        math::promils_of_u512(cspr_amount, self.governance_payment_ratio()).unwrap_or_revert()
    }
}
