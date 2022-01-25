use casper_contract::contract_api::runtime;
use casper_types::{U256, ApiError};

use crate::{Variable, Mapping, Address};

pub struct ERC20Token {
    pub total_supply: Variable<U256>,
    pub balances: Mapping<Address, U256>
}

impl Default for ERC20Token {
    fn default() -> Self {
        Self {
            total_supply: Variable::new(format!("total_supply")),
            balances: Mapping::new(format!("balances"))
        }
    }
}

impl ERC20Token {
    pub fn init(&mut self) {
        self.balances.init();
    }

    pub fn mint(&mut self, recipient: Address, amount: U256) {
        self.total_supply.set(self.total_supply.get() + amount);
        self.balances.set(&recipient, self.balances.get(&recipient) + amount);
    }

    pub fn transfer(&mut self, sender: Address, recipient: Address, amount: U256) {
        self.balances.set(&sender, self.balances.get(&sender) - amount);
        self.balances.set(&recipient, self.balances.get(&recipient) + amount);
    }
}