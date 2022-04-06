use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};

use casper_types::U256;
use delegate::delegate;

use crate::TokenWithStaking;

#[casper_contract_interface]
trait MockStakingInterface {
    fn init(&mut self) {}
    fn mint(&mut self, recipient: Address, amount: U256);
    fn burn(&mut self, owner: Address, amount: U256);
    fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256);
    fn stake(&mut self, address: Address, amount: U256);
    fn unstake(&mut self, address: Address, amount: U256);
    fn total_supply(&self) -> U256;
    fn balance_of(&self, address: Address) -> U256;
    fn get_stake_of(&self, address: Address) -> U256;
}

#[derive(Instance)]
pub struct MockStaking {
    token: TokenWithStaking,
}

impl MockStakingInterface for MockStaking {
    delegate! {
        to self.token {
            fn mint(&mut self, recipient: Address, amount: U256);
            fn burn(&mut self, owner: Address, amount: U256);
            fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256);
            fn stake(&mut self, address: Address, amount: U256);
            fn unstake(&mut self, address: Address, amount: U256);
            fn total_supply(&self) -> U256;
        }
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.token.balance_of(&address)
    }

    fn get_stake_of(&self, address: Address) -> U256 {
        self.token.get_stake_of(&address)
    }
}
