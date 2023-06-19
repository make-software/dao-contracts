use std::collections::BTreeMap;

use crate::modules::{access_control::AccessControlComposer, AccessControl};
use odra::{
    contract_env,
    types::{Address, Balance},
    Instance,
};

use super::{
    agg::{AggregatedBalance, BalanceAggregates, BalanceAggregatesComposer},
    balances::{BalanceStorage, BalanceStorageComposer},
    stakes::{StakesStorage, StakesStorageComposer},
};

/// Implementation of the Reputation Contract.
#[odra::module(skip_instance)]
pub struct ReputationContract {
    reputation_storage: BalanceStorage,
    passive_reputation_storage: BalanceStorage,
    stakes_storage: StakesStorage,
    aggregates: BalanceAggregates,
    access_control: AccessControl,
}

impl Instance for ReputationContract {
    fn instance(namespace: &str) -> Self {
        let access_control = AccessControlComposer::new(namespace, "access_control").compose();
        let reputation_storage = BalanceStorageComposer::new(namespace, "reputation")
            .with_access_control(&access_control)
            .compose();
        let passive_reputation_storage =
            BalanceStorageComposer::new(namespace, "passive_reputation")
                .with_access_control(&access_control)
                .compose();
        let stakes_storage = StakesStorageComposer::new(namespace, "stakes")
            .with_access_control(&access_control)
            .with_reputation_storage(&reputation_storage)
            .compose();
        let aggregates = BalanceAggregatesComposer::new(namespace, "aggregates")
            .with_reputation_storage(&reputation_storage)
            .compose();

        ReputationContractComposer::new(namespace, "reputation")
            .with_reputation_storage(&reputation_storage)
            .with_passive_reputation_storage(&passive_reputation_storage)
            .with_stakes_storage(&stakes_storage)
            .with_aggregates(&aggregates)
            .with_access_control(&access_control)
            .compose()
    }
}

#[odra::module]
impl ReputationContract {
    delegate! {
        to self.access_control {
            /// Changes ownership of the contract. Transfer the ownership to the `owner`. Only the current owner
            /// is permitted to call this method.
            ///
            /// See [AccessControl](AccessControl::change_ownership())
            pub fn change_ownership(&mut self, owner: Address);
            /// Adds a  new address to the whitelist.
            ///
            /// See [AccessControl](AccessControl::add_to_whitelist())
            pub fn add_to_whitelist(&mut self, address: Address);
            /// Removes address from the whitelist.
            ///
            /// See [AccessControl](AccessControl::remove_from_whitelist())
            pub fn remove_from_whitelist(&mut self, address: Address);
            /// Checks whether the given address is added to the whitelist.
            pub fn is_whitelisted(&self, address: Address) -> bool;
            /// Returns the address of the current owner.
            pub fn get_owner(&self) -> Option<Address>;
        }

        to self.reputation_storage {
            /// Mints new tokens. Adds `amount` of new tokens to the balance of the `recipient` and
            /// increments the total supply. Only whitelisted addresses are permitted to call this method.
            ///
            /// # Errors
            /// * [`NotWhitelisted`](utils::errors::Error::NotWhitelisted) if caller
            /// is not whitelisted.
            ///
            /// # Events
            /// * [`Mint`](events::Mint).
            pub fn mint(&mut self, recipient: Address, amount: Balance);
            /// Burns existing tokens. Removes `amount` of existing tokens from the balance of the `owner`
            /// and decrements the total supply. Only whitelisted addresses are permitted to call this
            /// method.
            ///
            /// # Errors
            /// * [`NotWhitelisted`](utils::errors::Error::NotWhitelisted) if caller
            /// is not whitelisted.
            ///
            /// # Events
            /// * [`Burn`](events::Burn) event.
            pub fn burn(&mut self, owner: Address, amount: Balance);
            /// Returns the total token supply.
            pub fn total_supply(&self) -> Balance;
            /// Returns the current token balance of the given address.
            pub fn balance_of(&self, address: Address) -> Balance;
            /// Redistributes the reputation based on the voting summary
            pub fn bulk_mint_burn(&mut self, mints: BTreeMap<Address, Balance>, burns: BTreeMap<Address, Balance>);
            /// Burns all the tokens of the `owner`.
            pub fn burn_all(&mut self, owner: Address);
        }

        to self.stakes_storage {
            pub fn stake(&mut self, voter: Address, stake: Balance);

            pub fn unstake(&mut self, voter: Address, stake: Balance);

            pub fn bulk_unstake(&mut self, stakes: Vec<(Address, Balance)>);

            pub fn get_stake(&self, address: Address) -> Balance;
        }

        to self.aggregates {
            /// Gets balances of all the token holders.
            pub fn all_balances(&self) -> AggregatedBalance;
            /// Gets balances of the given account addresses.
            pub fn partial_balances(&self, addresses: Vec<Address>) -> AggregatedBalance;
        }
    }

    /// Constructor method.
    ///
    /// It initializes contract elements:
    /// * Events dictionary.
    /// * Named keys of [`AccessControl`].
    /// * Set [`caller`] as the owner of the contract.
    /// * Add [`caller`] to the whitelist.
    ///
    /// # Events
    /// * [`OwnerChanged`](modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](modules::events::AddedToWhitelist).
    #[odra(init)]
    pub fn init(&mut self) {
        let deployer = contract_env::caller();
        self.access_control.init(deployer);
    }

    /// Increases the balance of the passive reputation of the given address.
    ///
    /// # Errors
    /// * [`NotWhitelisted`](utils::errors::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    pub fn mint_passive(&mut self, recipient: Address, amount: Balance) {
        self.passive_reputation_storage.mint(recipient, amount);
    }

    /// Decreases the balance of the passive reputation of the given address.
    ///
    /// # Errors
    /// * [`NotWhitelisted`](utils::errors::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    /// * [`InsufficientBalance`](utils::errors::Error::InsufficientBalance) if the passed
    /// amount exceeds the balance of the passive reputation of the given address.
    pub fn burn_passive(&mut self, owner: Address, amount: Balance) {
        self.passive_reputation_storage.burn(owner, amount);
    }

    /// Returns the current passive balance of the given address.
    pub fn passive_balance_of(&self, address: Address) -> Balance {
        self.passive_reputation_storage.balance_of(address)
    }
}

pub mod events {
    use crate::bid_escrow::types::BidId;
    use odra::{
        types::{Address, Balance},
        Event,
    };

    /// Informs tokens have been burnt.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Burn {
        pub address: Address,
        pub amount: Balance,
    }

    /// Informs tokens have been minted.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Mint {
        pub address: Address,
        pub amount: Balance,
    }

    /// Informs tokens have been staked.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Stake {
        pub worker: Address,
        pub amount: Balance,
        pub bid_id: BidId,
    }

    /// Informs tokens have been unstaked.
    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct Unstake {
        pub worker: Address,
        pub amount: Balance,
        pub bid_id: BidId,
    }
}
