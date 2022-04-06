use casper_dao_modules::Token;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    Address,
};

use casper_types::U256;
use delegate::delegate;

#[casper_contract_interface]
trait MockTokenInterface {
    fn init(&mut self) {}
    fn mint(&mut self, recipient: Address, amount: U256);
    fn burn(&mut self, owner: Address, amount: U256);
    fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256);
    fn ensure_balance(&mut self, address: Address, amount: U256);
    fn total_supply(&self) -> U256;
    fn balance_of(&self, address: Address) -> U256;
}

#[derive(Instance)]
pub struct MockToken {
    token: Token,
}

impl MockTokenInterface for MockToken {
    delegate! {
        to self.token {
            fn mint(&mut self, recipient: Address, amount: U256);
            fn burn(&mut self, owner: Address, amount: U256);
            fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256);
            fn total_supply(&self) -> U256;
        }
    }

    fn ensure_balance(&mut self, address: Address, amount: U256) {
        self.token.ensure_balance(&address, amount);
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.token.balance_of(&address)
    }
}
