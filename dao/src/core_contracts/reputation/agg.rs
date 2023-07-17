use odra::{
    types::{Address, Balance},
    OdraType,
};
use alloc::{collections::BTreeMap, vec::Vec};

use super::balances::BalanceStorage;

/// A module that provides aggregated data about reputation tokens.
#[odra::module]
pub struct BalanceAggregates {
    reputation_storage: BalanceStorage,
}

impl BalanceAggregates {
    /// Gets balances of all the token holders.
    pub fn all_balances(&self) -> AggregatedBalance {
        let mut balances = BTreeMap::<Address, Balance>::new();
        self.reputation_storage.holders().for_each(|address| {
            balances.insert(address, self.reputation_storage.balance_of(address));
        });

        AggregatedBalance::new(balances, self.reputation_storage.total_supply())
    }

    /// Gets balances of the given account addresses.
    pub fn partial_balances(&self, addresses: Vec<Address>) -> AggregatedBalance {
        let mut balances = BTreeMap::<Address, Balance>::new();
        let mut partial_supply = Balance::zero();
        for address in addresses {
            let balance = self.reputation_storage.balance_of(address);
            balances.insert(address, balance);
            partial_supply += balance;
        }
        AggregatedBalance {
            balances,
            total_supply: partial_supply,
        }
    }
}

/// Stores information about balances and the total supply.
#[derive(OdraType)]
pub struct AggregatedBalance {
    balances: BTreeMap<Address, Balance>,
    total_supply: Balance,
}

impl AggregatedBalance {
    pub fn new(balances: BTreeMap<Address, Balance>, total_supply: Balance) -> Self {
        Self {
            balances,
            total_supply,
        }
    }

    pub fn balances(&self) -> &BTreeMap<Address, Balance> {
        &self.balances
    }

    pub fn total_supply(&self) -> Balance {
        self.total_supply
    }
}
