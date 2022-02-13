use casper_contract::contract_api::runtime;
use casper_types::U256;

use crate::{casper_env::emit, consts, token::Token, Address, Error, Mapping};

use self::events::{TokensStaked, TokensUnstaked};

pub struct TokenWithStaking {
    pub stakes: Mapping<Address, U256>,
    pub token: Token,
}

impl Default for TokenWithStaking {
    fn default() -> Self {
        Self {
            stakes: Mapping::from(consts::NAME_STAKES),
            token: Token::default(),
        }
    }
}

impl TokenWithStaking {
    pub fn init(&mut self) {
        self.stakes.init();
        self.token.init();
    }

    pub fn mint(&mut self, recipient: Address, amount: U256) {
        self.token.mint(recipient, amount);
    }

    pub fn burn(&mut self, owner: Address, amount: U256) {
        self.ensure_balance(&owner, amount);
        self.token.burn(owner, amount);
    }

    pub fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256) {
        self.ensure_balance(&sender, amount);
        self.token.raw_transfer(sender, recipient, amount);
    }

    pub fn stake(&mut self, address: Address, amount: U256) {
        self.ensure_balance(&address, amount);
        self.stakes
            .set(&address, self.stakes.get(&address) + amount);
        emit(TokensStaked { address, amount });
    }

    pub fn unstake(&mut self, address: Address, amount: U256) {
        self.ensure_staked_balance(&address, amount);
        self.stakes
            .set(&address, self.stakes.get(&address) - amount);
        emit(TokensUnstaked { address, amount });
    }

    fn ensure_balance(&mut self, address: &Address, amount: U256) {
        let staked_amount = self.stakes.get(address);
        self.token.ensure_balance(address, staked_amount + amount);
    }

    fn ensure_staked_balance(&mut self, address: &Address, amount: U256) {
        if self.stakes.get(address) < amount {
            runtime::revert(Error::InsufficientBalance);
        }
    }
}

pub mod events {
    use casper_dao_macros::Event;
    use casper_types::U256;

    use crate::Address;

    #[derive(Debug, PartialEq, Event)]
    pub struct TokensStaked {
        pub address: Address,
        pub amount: U256,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct TokensUnstaked {
        pub address: Address,
        pub amount: U256,
    }
}
