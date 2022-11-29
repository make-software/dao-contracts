use crate::common::{
    params::{Account, U256},
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn reputation_balance(&self, account: &Account) -> U256 {
        let address = self.get_address(&account);

        U256(self.reputation_token.balance_of(address))
    }

    pub fn staked_reputation(&self, account: &Account) -> U256 {
        let address = self.get_address(&account);

        U256(self.reputation_token.get_stake(address))
    }
}
