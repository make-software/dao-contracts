use casper_contract::contract_api::runtime;
use casper_types::U256;

use self::events::Transfer;
use crate::{consts, emit, Address, Error, Mapping, Variable};

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

pub mod entry_points {
    use casper_types::{CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter, U256};

    use crate::{consts, Address};

    pub fn mint() -> EntryPoint {
        EntryPoint::new(
            consts::EP_MINT,
            vec![
                Parameter::new(consts::PARAM_RECIPIENT, Address::cl_type()),
                Parameter::new(consts::PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn burn() -> EntryPoint {
        EntryPoint::new(
            consts::EP_BURN,
            vec![
                Parameter::new(consts::PARAM_OWNER, Address::cl_type()),
                Parameter::new(consts::PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn transfer_from() -> EntryPoint {
        EntryPoint::new(
            consts::EP_TRANSFER_FROM,
            vec![
                Parameter::new(consts::PARAM_OWNER, Address::cl_type()),
                Parameter::new(consts::PARAM_RECIPIENT, Address::cl_type()),
                Parameter::new(consts::PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}

pub mod events {
    use casper_types::U256;
    use macros::Event;

    use crate::Address;

    #[derive(Debug, PartialEq, Event)]
    pub struct Transfer {
        pub from: Address,
        pub to: Address,
        pub value: U256,
    }
}
