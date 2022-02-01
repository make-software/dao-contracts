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
    use casper_types::{
        bytesrepr::{Error, FromBytes, ToBytes},
        CLType, CLTyped, U256,
    };

    use crate::Address;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Transfer {
        pub from: Address,
        pub to: Address,
        pub value: U256,
    }

    impl CLTyped for Transfer {
        fn cl_type() -> casper_types::CLType {
            CLType::Any
        }
    }

    impl FromBytes for Transfer {
        fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
            let (event_name, bytes): (String, _) = FromBytes::from_bytes(bytes)?;
            if &event_name != "transfer" {
                return Err(Error::Formatting);
            }
            let (to, bytes) = FromBytes::from_bytes(bytes)?;
            let (from, bytes) = FromBytes::from_bytes(bytes)?;
            let (value, bytes) = FromBytes::from_bytes(bytes)?;
            let event = Transfer { to, from, value };
            Ok((event, bytes))
        }
    }

    impl ToBytes for Transfer {
        fn to_bytes(&self) -> Result<Vec<u8>, Error> {
            let mut vec = Vec::with_capacity(self.serialized_length());
            vec.append(&mut String::from("transfer").to_bytes()?);
            vec.append(&mut self.to.to_bytes()?);
            vec.append(&mut self.from.to_bytes()?);
            vec.append(&mut self.value.to_bytes()?);
            Ok(vec)
        }

        fn serialized_length(&self) -> usize {
            let mut size = 0;
            size += String::from("transfer").serialized_length();
            size += self.from.serialized_length();
            size += self.to.serialized_length();
            size += self.value.serialized_length();
            return size;
        }
    }
}
