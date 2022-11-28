use casper_dao_utils::{Address, TestContract};

use crate::common::{
    params::{Account, U256},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        let idx = match account {
            Account::Owner => 0,
            Account::Deployer => 0,
            Account::Alice => 1,
            Account::Bob => 2,
            Account::Holder => 3,
            Account::Any => 4,
            Account::VA(n) => 4 + n,
        };
        self.env.get_account(idx)
    }

    pub fn mint_reputation(&mut self, minter: &Account, recipient: &Account, amount: U256) {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);

        self.reputation_token
            .as_account(minter)
            .mint(recipient, amount.0)
            .expect("Mint failed");
    }
}
