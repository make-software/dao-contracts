use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{self, emit},
    Address, Error, Mapping, Variable,
};
use casper_types::U256;

use self::events::{Approval, Transfer};

#[casper_contract_interface]
pub trait ERC20Interface {
    fn init(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256);
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn decimals(&self) -> u8;
    fn total_supply(&self) -> U256;
    fn balance_of(&self, address: Address) -> U256;
    fn transfer(&mut self, recipient: Address, amount: U256);
    fn approve(&mut self, spender: Address, amount: U256);
    fn allowance(&self, owner: Address, spender: Address) -> U256;
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);
}

#[derive(Instance)]
pub struct ERC20 {
    name: Variable<String>,
    symbol: Variable<String>,
    decimals: Variable<u8>,
    total_supply: Variable<U256>,
    balances: Mapping<Address, U256>,
    allowances: Mapping<(Address, Address), U256>,
}

impl ERC20Interface for ERC20 {
    fn init(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let sender = casper_env::caller();
        self.name.set(name);
        self.symbol.set(symbol);
        self.decimals.set(decimals);
        self.total_supply.set(initial_supply);
        self.balances.set(&sender, initial_supply);

        emit(Transfer {
            from: None,
            to: Some(sender),
            value: initial_supply,
        });
    }

    fn name(&self) -> String {
        self.name.get()
    }

    fn symbol(&self) -> String {
        self.symbol.get()
    }

    fn decimals(&self) -> u8 {
        self.decimals.get()
    }

    fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.balances.get(&address)
    }

    fn transfer(&mut self, recipient: Address, amount: U256) {
        let owner = casper_env::caller();
        self.raw_transfer(owner, recipient, amount);
    }

    fn approve(&mut self, spender: Address, amount: U256) {
        let owner = casper_env::caller();
        self.allowances.set(&(owner, spender), amount);

        emit(Approval {
            owner,
            spender,
            value: amount,
        });
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.get(&(owner, spender))
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        let spender = casper_env::caller();
        self.raw_transfer(owner, recipient, amount);
        self.spend_allowance(owner, spender, amount);
    }
}

impl ERC20 {
    pub fn raw_transfer(&mut self, owner: Address, recipient: Address, amount: U256) {
        let owner_balance = self.balances.get(&owner);
        let recipient_balance = self.balances.get(&recipient);
        if owner_balance < amount {
            casper_env::revert(Error::InsufficientBalance)
        }
        self.balances.set(&owner, owner_balance - amount);
        self.balances.set(&recipient, recipient_balance + amount);

        emit(Transfer {
            from: Some(owner),
            to: Some(recipient),
            value: amount,
        });
    }

    pub fn spend_allowance(&mut self, owner: Address, spender: Address, amount: U256) {
        let key = (owner, spender);
        let allowance = self.allowances.get(&key);
        if amount > allowance {
            casper_env::revert(Error::InsufficientAllowance);
        }
        self.allowances.set(&key, allowance - amount);

        emit(Approval {
            owner,
            spender,
            value: allowance - amount,
        });
    }
}

pub mod events {
    use casper_dao_utils::{casper_dao_macros::Event, Address};
    use casper_types::U256;

    #[derive(Debug, PartialEq, Event)]
    pub struct Transfer {
        pub from: Option<Address>,
        pub to: Option<Address>,
        pub value: U256,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct Approval {
        pub owner: Address,
        pub spender: Address,
        pub value: U256,
    }
}
