use casper_dao_utils::TestContract;

use crate::common::{
    params::{Account, Balance, U256},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn reputation_balance(&self, account: &Account) -> Balance {
        let address = self.get_address(&account);
        let balance = self.reputation_token.balance_of(address);
        Balance(balance)
    }

    pub fn staked_reputation(&self, account: &Account) -> Balance {
        let address = self.get_address(&account);

        Balance(self.reputation_token.get_stake(address))
    }

    pub fn mint_reputation(&mut self, minter: &Account, recipient: &Account, amount: U256) {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);
        let amount: Balance = amount.into();

        self.reputation_token
            .as_account(minter)
            .mint(recipient, amount.0)
            .expect("Mint failed");
    }
}
