use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

use crate::{casper_env::emit, consts, Error, Mapping, OrderedCollection, Set};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

use self::events::ValueSet;

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

        for (key, value) in RepositoryDefaults::default().items() {
            self.set_or_update(key, value);
        }
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
pub struct RepositoryDefaults {
    pub items: Vec<(String, Bytes)>,
}

impl RepositoryDefaults {}

impl RepositoryDefaults {
    pub fn push<T: ToBytes>(&mut self, key: &str, value: T) {
        let value: Bytes = Bytes::from(value.to_bytes().unwrap_or_revert());
        self.items.push((String::from(key), value));
    }

    pub fn items(self) -> Vec<(String, Bytes)> {
        self.items
    }

    #[cfg(feature = "test-support")]
    pub fn len() -> u32 {
        RepositoryDefaults::default().items.len() as u32
    }
}

impl Default for RepositoryDefaults {
    fn default() -> Self {
        let mut items = RepositoryDefaults { items: vec![] };
        items.push(consts::DEFAULT_POLICING_RATE, U256::from(300));
        items.push(consts::REPUTATION_CONVERSION_RATE, U256::from(10));
        items.push(consts::FORUM_KYC_REQUIRED, true);
        items.push(consts::FORMAL_VOTING_QUORUM, U256::from(500));
        items.push(consts::INFORMAL_VOTING_QUORUM, U256::from(50));
        items.push(consts::VOTING_QUORUM, U256::from(200));
        items.push(consts::FORMAL_VOTING_TIME, U256::from(432000000));
        items.push(consts::INFORMAL_VOTING_TIME, U256::from(86400000));
        items.push(consts::VOTING_TIME, U256::from(172800000));
        items.push(consts::MINIMUM_GOVERNANCE_REPUTATION, U256::from(100));
        items.push(consts::MINIMUM_VOTING_REPUTATION, U256::from(10));
        items
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
