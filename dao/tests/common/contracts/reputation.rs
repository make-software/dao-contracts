use crate::common::helpers::{is_cspr_balance_close_enough, is_reputation_close_enough};
use crate::common::params::ReputationBalance;
use crate::common::{params::Account, DaoWorld};

#[allow(dead_code)]
impl DaoWorld {
    pub fn reputation_balance(&self, account: &Account) -> ReputationBalance {
        let address = self.get_address(account);
        let balance = self.reputation_token.balance_of(address);
        ReputationBalance(balance)
    }

    pub fn passive_reputation_balance(&self, account: &Account) -> ReputationBalance {
        let address = self.get_address(account);
        let balance = self.reputation_token.passive_balance_of(address);
        ReputationBalance(balance)
    }

    pub fn staked_reputation(&self, account: &Account) -> ReputationBalance {
        let address = self.get_address(account);

        ReputationBalance(self.reputation_token.get_stake(address))
    }

    pub fn mint_reputation(
        &mut self,
        minter: &Account,
        recipient: &Account,
        amount: ReputationBalance,
    ) {
        let recipient = self.get_address(recipient);

        self.set_caller(minter);
        self.reputation_token.mint(recipient, amount.0);
    }

    pub fn assert_staked_reputation(&self, account: &Account, expected_balance: ReputationBalance) {
        let real_reputation_stake = self.staked_reputation(account);

        assert!(
            is_reputation_close_enough(*expected_balance, *real_reputation_stake),
            "For account {:?} sREP balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_reputation_stake
        );
    }

    pub fn assert_reputation(&self, account: &Account, expected_balance: ReputationBalance) {
        let real_reputation_balance = self.reputation_balance(account);

        assert!(
            is_reputation_close_enough(expected_balance, *real_reputation_balance),
            "For account {:?} REP balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_reputation_balance
        );
    }

    pub fn assert_total_supply(&self, expected_balance: ReputationBalance) {
        let total_reputation = self.reputation_token.total_supply();

        assert!(
            is_cspr_balance_close_enough(total_reputation, *expected_balance),
            "REP total supply should be {:?} but is {:?}",
            expected_balance,
            total_reputation
        );
    }

    pub fn assert_passive_reputation(
        &self,
        account: &Account,
        expected_balance: ReputationBalance,
    ) {
        let real_balance = self.passive_reputation_balance(account);

        assert!(
            is_reputation_close_enough(expected_balance, *real_balance),
            "For account {:?} passive REP balance should be {:?} but is {:?}",
            account,
            expected_balance,
            real_balance
        );
    }

    pub fn burn_all_reputation(&mut self, burner: &Account, holder: &Account) {
        let holder = self.get_address(holder);

        let holder_balance = self.reputation_token.balance_of(holder);
        self.set_caller(burner);
        self.reputation_token.burn(holder, holder_balance);
    }
}
