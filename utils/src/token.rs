use casper_contract::contract_api::runtime;
use casper_types::U256;

use crate::{Address, Error, Mapping, Variable};

pub struct Token {
    pub total_supply: Variable<U256>,
    pub balances: Mapping<Address, U256>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            total_supply: Variable::new(format!("total_supply")),
            balances: Mapping::new(format!("balances")),
        }
    }
}

impl Token {
    pub fn init(&mut self) {
        self.balances.init();
        self.total_supply.set(U256::zero());
    }

    pub fn mint(&mut self, recipient: Address, amount: U256) {
        self.total_supply.set(self.total_supply.get() + amount);
        self.balances
            .set(&recipient, self.balances.get(&recipient) + amount);
    }

    pub fn burn(&mut self, owner: Address, amount: U256) {
        self.total_supply.set(self.total_supply.get() - amount);
        self.balances
            .set(&owner, self.balances.get(&owner) - amount);
    }

    pub fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256) {
        self.balances
            .set(&sender, self.balances.get(&sender) - amount);
        self.balances
            .set(&recipient, self.balances.get(&recipient) + amount);
    }

    pub fn ensure_balance(&mut self, address: &Address, amount: U256) {
        if self.balances.get(address) < amount {
            runtime::revert(Error::InsufficientBalance);
        }
    }
}

pub mod entry_points {
    use casper_types::{CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter, U256};

    use crate::Address;

    pub fn mint() -> EntryPoint {
        EntryPoint::new(
            "mint",
            vec![
                Parameter::new("recipient", Address::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn burn() -> EntryPoint {
        EntryPoint::new(
            "burn",
            vec![
                Parameter::new("owner", Address::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn transfer_from() -> EntryPoint {
        EntryPoint::new(
            "transfer_from",
            vec![
                Parameter::new("owner", Address::cl_type()),
                Parameter::new("recipient", Address::cl_type()),
                Parameter::new("amount", U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}
