use core::hash::Hash;

use crate::modules::AccessControl;
use crate::utils::Error;

use alloc::vec::Vec;
use odra::{
    contract_env,
    types::{Address, Balance, OdraType},
    List, Mapping, UnwrapOrRevert,
};

use super::balances::BalanceStorage;

/// A module that stores information about stakes.
#[odra::module]
pub struct StakesStorage {
    stake: Mapping<Address, Balance>,
    access_control: AccessControl,
    reputation_storage: BalanceStorage,
}

impl StakesStorage {
    /// Increases the voter's stake and total stake.
    pub fn stake(&mut self, voter: Address, stake: Balance) {
        self.access_control.ensure_whitelisted();
        self.assert_stake(stake);
        self.assert_balance(voter, stake);
        self.inc_stake(voter, stake);
    }

    pub fn unstake(&mut self, voter: Address, stake: Balance) {
        self.bulk_unstake(alloc::vec![(voter, stake)]);
    }

    pub fn bulk_unstake(&mut self, stakes: Vec<(Address, Balance)>) {
        self.access_control.ensure_whitelisted();

        for (voter, stake) in stakes {
            self.assert_stake(stake);
            self.dec_stake(voter, stake);
        }
    }

    /// Returns the total stake of the given account.
    pub fn get_stake(&self, address: Address) -> Balance {
        self.stake.get(&address).unwrap_or_default()
    }
}

impl StakesStorage {
    fn assert_balance(&self, address: Address, stake: Balance) {
        let user_stake = self.get_stake(address);
        let available_balance = self
            .reputation_storage
            .balance_of(address)
            .saturating_sub(user_stake);

        if available_balance < stake {
            contract_env::revert(Error::InsufficientBalance)
        }
    }

    #[inline(always)]
    fn assert_stake(&self, stake: Balance) {
        if stake.is_zero() {
            contract_env::revert(Error::ZeroStake)
        }
    }

    fn inc_stake(&mut self, account: Address, amount: Balance) {
        let new_value = self.get_stake(account) + amount;
        self.stake.set(&account, new_value);
    }

    fn dec_stake(&mut self, account: Address, amount: Balance) {
        let new_value = self
            .get_stake(account)
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::CannotUnstakeMoreThanStaked);
        self.stake.set(&account, new_value);
    }
}

trait UpdatableVec<K, R> {
    fn push_record(&mut self, key: &K, record: R);
    fn remove_record(&mut self, key: &K, record: R);
}

impl<Key> UpdatableVec<Key, (Address, u32)> for Mapping<Key, List<Option<(Address, u32)>>>
where
    Key: OdraType + Hash,
{
    fn push_record(&mut self, key: &Key, record: (Address, u32)) {
        let mut records = self.get_instance(key);
        records.push(Some(record));
    }

    fn remove_record(&mut self, key: &Key, record: (Address, u32)) {
        let mut records = self.get_instance(key);
        if let Some(position) = records.iter().position(|r| r == Some(record)) {
            records.replace(position as u32, None);
        }
    }
}
