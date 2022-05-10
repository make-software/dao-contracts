//! Token with staking powers.

use self::events::{TokensStaked, TokensUnstaked};
use crate::Token;
use casper_dao_utils::{
    casper_dao_macros::Instance,
    casper_env::{self, emit},
    Address, Error, Mapping,
};
use casper_types::U256;

/// The TokenWithStaking module.
#[derive(Instance)]
pub struct TokenWithStaking {
    pub stakes: Mapping<Address, U256>,
    pub token: Token,
}

impl TokenWithStaking {
    /// Mint new tokens. See [`Token::mint`](Token::mint).
    pub fn mint(&mut self, recipient: Address, amount: U256) {
        self.token.mint(recipient, amount);
    }

    /// Burn unstaked tokens. See [`Token::burn`](Token::burn)
    pub fn burn(&mut self, owner: Address, amount: U256) {
        self.ensure_balance(&owner, amount);
        self.token.burn(owner, amount);
    }

    /// Transfer unstaked tokens. See [`Token::raw_transfer`](Token::raw_transfer)
    pub fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256) {
        self.ensure_balance(&sender, amount);
        self.token.raw_transfer(sender, recipient, amount);
    }

    /// Stake `amount` of tokens for the `address`. It decrements `address`'s balance by `amount`.
    pub fn stake(&mut self, address: Address, amount: U256) {
        self.ensure_balance(&address, amount);
        self.stakes.set(
            &address,
            self.stakes.get(&address).unwrap_or_default() + amount,
        );
        emit(TokensStaked { address, amount });
    }

    /// Unstake `amount` of tokens for the `address`. It increments `address`'s balance by `amount`.
    pub fn unstake(&mut self, address: Address, amount: U256) {
        self.ensure_staked_balance(&address, amount);
        self.stakes.set(
            &address,
            self.stakes.get(&address).unwrap_or_default() - amount,
        );
        emit(TokensUnstaked { address, amount });
    }

    pub fn total_supply(&self) -> U256 {
        self.token.total_supply()
    }

    pub fn balance_of(&self, address: &Address) -> U256 {
        self.token.balance_of(address)
    }

    pub fn get_stake_of(&self, address: &Address) -> U256 {
        self.stakes.get(address).unwrap_or_default()
    }

    fn ensure_balance(&mut self, address: &Address, amount: U256) {
        let staked_amount = self.stakes.get(address).unwrap_or_default();
        self.token.ensure_balance(address, staked_amount + amount);
    }

    fn ensure_staked_balance(&mut self, address: &Address, amount: U256) {
        if self.stakes.get(address).unwrap_or_default() < amount {
            casper_env::revert(Error::InsufficientBalance);
        }
    }
}

pub mod events {
    //! Events definitions.
    use casper_dao_utils::{casper_dao_macros::Event, Address};
    use casper_types::U256;

    /// Informs tokens have been staked.
    #[derive(Debug, PartialEq, Event)]
    pub struct TokensStaked {
        pub address: Address,
        pub amount: U256,
    }

    /// Informs tokens have been unstaked.
    #[derive(Debug, PartialEq, Event)]
    pub struct TokensUnstaked {
        pub address: Address,
        pub amount: U256,
    }
}
