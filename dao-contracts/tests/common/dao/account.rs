use casper_dao_utils::Address;

use crate::common::{params::Account, DaoWorld};

impl DaoWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        let idx = match account {
            Account::Owner => 0,
            Account::Deployer => 0,
            Account::Alice => 1,
            Account::Bob => 2,
            Account::Holder => 3,
            Account::Any => 4,
        };
        self.env.get_account(idx)
    }
}
