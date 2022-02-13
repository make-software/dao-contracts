use casper_contract::contract_api::runtime;
use casper_types::U256;

use self::events::{Burn, Mint, Transfer};
use crate::{casper_env::emit, consts, Address, Error, Mapping, Variable};

pub struct Token {
    pub total_supply: Variable<U256>,
    pub balances: Mapping<Address, U256>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            total_supply: Variable::from(consts::NAME_TOTAL_SUPPLY),
            balances: Mapping::from(consts::NAME_BALANCES),
        }
    }
}

impl Token {
    pub fn init(&mut self) {
        self.balances.init();
        self.total_supply.set(U256::zero());
    }

    pub fn mint(&mut self, recipient: Address, amount: U256) {
        let (new_supply, is_overflowed) = self.total_supply.get().overflowing_add(amount);
        if is_overflowed {
            runtime::revert(Error::TotalSupplyOverflow);
        }

        self.total_supply.set(new_supply);
        self.balances
            .set(&recipient, self.balances.get(&recipient) + amount);

        emit(Mint {
            recipient,
            value: amount,
        });
    }

    pub fn burn(&mut self, owner: Address, amount: U256) {
        self.total_supply.set(self.total_supply.get() - amount);
        self.balances
            .set(&owner, self.balances.get(&owner) - amount);
        emit(Burn {
            owner,
            value: amount,
        });
    }

    pub fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256) {
        self.balances
            .set(&sender, self.balances.get(&sender) - amount);
        self.balances
            .set(&recipient, self.balances.get(&recipient) + amount);

        emit(Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
    }

    pub fn ensure_balance(&mut self, address: &Address, amount: U256) {
        if self.balances.get(address) < amount {
            runtime::revert(Error::InsufficientBalance);
        }
    }
}

pub mod events {
    use casper_dao_macros::Event;
    use casper_types::U256;

    use crate::Address;

    #[derive(Debug, PartialEq, Event)]
    pub struct Transfer {
        pub from: Address,
        pub to: Address,
        pub value: U256,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct Mint {
        pub recipient: Address,
        pub value: U256,
    }

    #[derive(Debug, PartialEq, Event)]
    pub struct Burn {
        pub owner: Address,
        pub value: U256,
    }
}
