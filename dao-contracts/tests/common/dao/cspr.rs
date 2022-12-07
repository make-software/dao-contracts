use casper_types::U512;

use crate::common::{params::{Account, CsprBalance}, DaoWorld, helpers::is_cspr_close_enough};

#[allow(dead_code)]
impl DaoWorld {
    // sets relative amount of motes to the account
    pub fn set_cspr_balance(&mut self, account: &Account, amount: CsprBalance) {
        let account = self.get_address(account);

        assert!(
            !self.balances.contains_key(&account),
            "Cannot set cspr balance twice"
        );

        self.balances.insert(account, *amount);

        self.starting_balances
            .insert(account, self.env.get_address_cspr_balance(account));
    }

    // gets relative amount of motes of the account
    pub fn get_cspr_balance(&self, account: &Account) -> U512 {
        let account = self.get_address(account);
        let balance =
            self.balances.get(&account).unwrap() + self.env.get_address_cspr_balance(account);
        let result = balance
            .checked_sub(*self.starting_balances.get(&account).unwrap())
            .unwrap();
        result
    }

    pub fn assert_cspr_balance(&self, account: &Account, expected_balance: CsprBalance) {
        let real_cspr_balance = self.get_cspr_balance(account);

        assert!(
            is_cspr_close_enough(*expected_balance, real_cspr_balance),
            "For account {:?} CSPR balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_cspr_balance
        );
    }
}
