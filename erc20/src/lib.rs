#![no_main]
pub mod error;

use std::fmt::Result;

use casper_dao_utils::{
    casper_dao_macros::casper_contract_interface,
    casper_env::{caller, init_events, revert},
    owner::Owner,
    staking::TokenWithStaking,
    whitelist::Whitelist,
    Address, Mapping, Variable,
};
use casper_types::U256;
use error::Error;

// TODO: Put it lower.
//
// Interface of the Reputation Contract.
//
// It should be implemented by [`ReputationContract`], [`ReputationContractCaller`]
// and [`ReputationContractTest`].

#[casper_contract_interface]
pub trait ERC20Interface {
    fn init(&mut self, initial_supply: U256);

    fn balance_of(&self, address: Address) -> U256;

    fn transfer(&mut self, recipient: Address, amount: U256);
}

pub struct ERC20 {
    balances: Mapping<Address, U256>,
    total_supply: Variable<U256>,
}

impl ERC20Interface for ERC20 {
    fn init(&mut self,initial_supply:U256) {
        self.total_supply.set(initial_supply);
        self.balances.set(&caller(), initial_supply);
    }

    fn balance_of(&self, address: Address) -> U256 {
        self.balances.get(&address)
    }


    fn transfer(&mut self, recipient: Address, amount:U256) {
        self.raw_transfer(&caller(), &recipient, amount)
    }
}

impl ERC20 {
    pub fn raw_transfer(&mut self, from: &Address, to: &Address, amount: U256) {
        let from_balance = self.balances.get(from);
        let to_balance = self.balances.get(to);
        if from_balance < amount { 
            revert(Error::InsufficientBalance)
        }
        self.balances.set(from, from_balance - amount);
        self.balances.set(to, to_balance + amount);
    }
}