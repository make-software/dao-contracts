use casper_dao_utils::TestContract;

use crate::common::{
    helpers::is_balance_close_enough,
    params::{Account, Balance},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn reputation_balance(&self, account: &Account) -> Balance {
        let address = self.get_address(account);
        let balance = self.reputation_token.balance_of(address);
        Balance(balance)
    }

    pub fn staked_reputation(&self, account: &Account) -> Balance {
        let address = self.get_address(account);

        Balance(self.reputation_token.get_stake(address))
    }

    pub fn mint_reputation(&mut self, minter: &Account, recipient: &Account, amount: Balance) {
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);

        self.reputation_token
            .as_account(minter)
            .mint(recipient, amount.0)
            .expect("Mint failed");
    }

    pub fn assert_staked_reputation(&self, account: &Account, expected_balance: Balance) {
        let real_reputation_stake = self.staked_reputation(account);

        assert!(
            is_balance_close_enough(expected_balance, *real_reputation_stake),
            "For account {:?} sREP balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_reputation_stake
        );
    }

    pub fn assert_reputation(&self, account: &Account, expected_balance: Balance) {
        let real_reputation_balance = self.reputation_balance(account);

        assert!(
            is_balance_close_enough(expected_balance, *real_reputation_balance),
            "For account {:?} REP balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_reputation_balance
        );
    }

    pub fn assert_total_supply(&self, expected_balance: Balance) {
        let total_reputation = self.reputation_token.total_supply();

        assert!(
            is_balance_close_enough(total_reputation, *expected_balance),
            "REP total supply should be {:?} but is {:?}",
            expected_balance,
            total_reputation
        );
    }
}
