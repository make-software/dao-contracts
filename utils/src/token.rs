use casper_types::U256;

use crate::{emit, Address, Mapping, Variable};

use self::events::Transfer;

pub struct Token {
    pub total_supply: Variable<U256>,
    pub balances: Mapping<Address, U256>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            total_supply: Variable::new(String::from("total_supply")),
            balances: Mapping::new(String::from("balances")),
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
            size
        }
    }
}
