use std::collections::HashMap;

use casper_contract::contract_api::runtime;

use crate::{casper_env::emit, consts, Error, Mapping, OrderedCollection, Set};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

use self::events::ValueSet;

const BASE: u32 = 1000;

pub struct Repository {
    pub storage: Mapping<String, Option<Bytes>>,
    pub keys: OrderedCollection<String>,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            storage: Mapping::from(consts::NAME_STORAGE),
            keys: OrderedCollection::new(consts::NAME_KEYS),
        }
    }
}

impl Repository {
    pub fn init(&mut self) {
        self.storage.init();
        self.keys.init();

        RepositoryDefaults::new()
            .values
            .into_iter()
            .for_each(|(key, value)| {
                self.set_or_update(key, Bytes::from(value.to_bytes().unwrap()))
            });
    }

    pub fn set_or_update(&mut self, key: String, value: Bytes) {
        self.storage.set(&key, Some(value.to_owned()));
        self.keys.add(key.to_owned());
        let event = ValueSet { key, value };
        emit(event);
    }

    pub fn get(&self, key: String) -> Bytes {
        match self.storage.get(&key) {
            Some(value) => value,
            None => runtime::revert(Error::ValueNotAvailable),
        }
    }
}

struct RepositoryDefaults {
    pub values: HashMap<String, Box<dyn ToBytes>>,
}

impl RepositoryDefaults {
    pub fn new() -> RepositoryDefaults {
        let mut values: HashMap<String, Box<dyn ToBytes>> = HashMap::new();
        values.insert(
            consts::DEFAULT_POLICING_RATE.to_string(),
            Box::new(U256::from(300)),
        );
        values.insert(
            consts::REPUTATION_CONVERSION_RATE.to_string(),
            Box::new(U256::from(10)),
        );
        values.insert(consts::FORUM_KYC_REQUIRED.to_string(), Box::new(true));
        values.insert(
            consts::FORMAL_VOTING_QUORUM.to_string(),
            Box::new(U256::from(500)),
        );
        values.insert(
            consts::INFORMAL_VOTING_QUORUM.to_string(),
            Box::new(U256::from(50)),
        );
        values.insert(consts::VOTING_QUORUM.to_string(), Box::new(U256::from(200)));
        values.insert(
            consts::FORMAL_VOTING_TIME.to_string(),
            Box::new(U256::from(432000)),
        );
        values.insert(
            consts::INFORMAL_VOTING_TIME.to_string(),
            Box::new(U256::from(86400000)),
        );
        values.insert(
            consts::VOTING_TIME.to_string(),
            Box::new(U256::from(172800000)),
        );
        values.insert(
            consts::MINIMUM_GOVERNANCE_REPUTATION.to_string(),
            Box::new(U256::from(100)),
        );
        values.insert(
            consts::MINIMUM_VOTING_REPUTATION.to_string(),
            Box::new(U256::from(10)),
        );
        Self { values }
    }
}

pub mod entry_points {
    use casper_types::{
        bytesrepr::Bytes, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter,
    };

    use crate::consts;

    pub fn set_or_update() -> EntryPoint {
        EntryPoint::new(
            consts::EP_SET_OR_UPDATE,
            vec![
                Parameter::new(consts::PARAM_KEY, String::cl_type()),
                Parameter::new(consts::PARAM_VALUE, Bytes::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn get() -> EntryPoint {
        EntryPoint::new(
            consts::EP_GET,
            vec![Parameter::new(consts::PARAM_KEY, String::cl_type())],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}

pub mod events {
    use casper_dao_macros::Event;
    use casper_types::bytesrepr::Bytes;

    #[derive(Debug, PartialEq, Event)]
    pub struct ValueSet {
        pub key: String,
        pub value: Bytes,
    }
}
