use crate::{
    casper_env::{emit, get_block_time},
    consts, Error, Mapping, OrderedCollection, Set,
};
use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

use self::events::ValueUpdated;

type Record = (Bytes, Option<(Bytes, u64)>);
pub struct Repository {
    pub storage: Mapping<String, Record>,
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
            self.set(key, value);
        }
    }

    pub fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        let now = get_block_time();
        let value_for_event = value.clone();
        let new_value: Record = match activation_time {
            // If no activation_time provided update the record to the value from argument.
            None => (value, None),

            // If activation_time is in the past, raise an error.
            Some(activation_time) if activation_time < now => {
                runtime::revert(Error::ActivationTimeInPast)
            }

            // If activation time is in future.
            Some(activation_time) => {
                // Load the record.
                let (current_value, current_next_value) = self.storage.get_or_revert(&key);
                match current_next_value {
                    // If current_next_value is not set, update it to the value from arguments.
                    None => (current_value, Some((value, activation_time))),

                    // If current_next_value is set, but it is in the past, make it a current
                    // value and set next_value to values from arguments.
                    Some((current_next_value, current_activation_time))
                        if current_activation_time < now =>
                    {
                        (current_next_value, Some((value, activation_time)))
                    }

                    // If current_next_value is set in future, update it.
                    Some(_) => (current_value, Some((value, activation_time))),
                }
            }
        };
        self.storage.set(&key, new_value);
        self.keys.add(key.clone());
        emit(ValueUpdated {
            key,
            value: value_for_event,
            activation_time,
        });
    }

    pub fn get(&self, key: String) -> Bytes {
        let (current, future) = self.storage.get_or_revert(&key);
        let now = get_block_time();
        if let Some((value, activation_time)) = future {
            if now > activation_time {
                return value;
            }
        }
        current
    }

    fn set(&mut self, key: String, value: Bytes) {
        self.update_at(key, value, None);
    }
}

pub struct RepositoryDefaults {
    pub items: Vec<(String, Bytes)>,
}

impl RepositoryDefaults {
    #[cfg(not(feature = "test-support"))]
    pub fn push<T: ToBytes>(&mut self, key: &str, value: T) {
        use casper_contract::unwrap_or_revert::UnwrapOrRevert;
        let value: Bytes = Bytes::from(value.to_bytes().unwrap_or_revert());
        self.items.push((String::from(key), value));
    }

    #[cfg(feature = "test-support")]
    pub fn push<T: ToBytes>(&mut self, key: &str, value: T) {
        let value: Bytes = Bytes::from(value.to_bytes().unwrap());
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
    pub struct ValueUpdated {
        pub key: String,
        pub value: Bytes,
        pub activation_time: Option<u64>,
    }
}
