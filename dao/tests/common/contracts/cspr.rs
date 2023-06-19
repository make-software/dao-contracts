use std::collections::HashMap;

use odra::types::address::OdraAddress;
use odra::{test_env, types::Address};

use crate::common::helpers::is_cspr_balance_close_enough;
use crate::common::{
    params::{Account, CsprBalance},
    DaoWorld,
};

#[derive(Default, Clone)]
pub struct VirtualBalances {
    current: HashMap<Address, CsprBalance>,
    initial: HashMap<Address, CsprBalance>,
}

impl VirtualBalances {
    pub fn init(&mut self, account: Address, amount: CsprBalance) {
        assert!(
            !self.current.contains_key(&account),
            "Cannot set cspr balance twice"
        );

        self.current.insert(account, amount);

        self.initial
            .insert(account, CsprBalance(test_env::token_balance(account)));
    }

    pub fn get(&self, address: Address) -> CsprBalance {
        let mut balance = self.current.get(&address).unwrap().0 + test_env::token_balance(address);
        if !address.is_contract() {
            balance += test_env::total_gas_used(address);
        }

        let result = balance
            .checked_sub(self.initial.get(&address).unwrap().0)
            .unwrap();
        CsprBalance(result)
    }
}

#[allow(dead_code)]
impl DaoWorld {
    // sets relative amount of motes to the account
    pub fn set_cspr_balance(&mut self, account: &Account, amount: CsprBalance) {
        let account = self.get_address(account);

        self.virtual_balances.init(account, amount);
    }

    // gets relative amount of motes of the account
    pub fn get_cspr_balance(&self, account: &Account) -> CsprBalance {
        let account = self.get_address(account);
        self.virtual_balances.get(account)
    }

    pub fn assert_cspr_balance(&self, account: &Account, expected_balance: CsprBalance) {
        let real_cspr_balance = self.get_cspr_balance(account);

        assert!(
            is_cspr_balance_close_enough(expected_balance, real_cspr_balance),
            "For account {:?} CSPR balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_cspr_balance
        );
    }
}
