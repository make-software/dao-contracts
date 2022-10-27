use casper_dao_utils::{
    casper_dao_macros::Instance,
    casper_env::{self, emit, get_block_time},
    consts, Error, Mapping, OrderedCollection, Set,
};

use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

use self::events::ValueUpdated;

/// A data struct stored in the repository.
///
/// The first value represents the current value.
///
/// The second value is an optional tuple consisting of the future value and its activation time.
pub type Record = (Bytes, Option<(Bytes, u64)>);

#[derive(Instance)]
pub struct Repository {
    pub storage: Mapping<String, Record>,
    pub keys: OrderedCollection<String>,
}

impl Repository {
    pub fn init(&mut self) {
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
                casper_env::revert(Error::ActivationTimeInPast)
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

    pub fn get(&self, key: String) -> Option<Bytes> {
        let (current, future) = self.storage.get_or_none(&key)?;
        let now = get_block_time();
        if let Some((value, activation_time)) = future {
            if now > activation_time {
                return Some(value);
            }
        }
        Some(current)
    }

    pub fn get_full_value(&self, key: String) -> Option<Record> {
        self.storage.get_or_none(&key)
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
        use casper_dao_utils::casper_contract::unwrap_or_revert::UnwrapOrRevert;
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
        items.push(consts::POST_JOB_DOS_FEE, U256::from(10000));
        items.push(consts::INTERNAL_AUCTION_TIME, U256::from(604800));
        items.push(consts::PUBLIC_AUCTION_TIME, U256::from(864000));
        items.push(consts::DEFAULT_POLICING_RATE, U256::from(300));
        items.push(consts::REPUTATION_CONVERSION_RATE, U256::from(100));
        items.push(consts::FORUM_KYC_REQUIRED, true);
        items.push(consts::FORMAL_VOTING_QUORUM, U256::from(500));
        items.push(consts::INFORMAL_VOTING_QUORUM, U256::from(50));
        items.push(consts::VOTING_QUORUM, U256::from(200));
        items.push(consts::FORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::INFORMAL_VOTING_TIME, 86400000u64);
        items.push(consts::VOTING_TIME, 172800000u64);
        // TODO: Remove
        items.push(consts::MINIMUM_GOVERNANCE_REPUTATION, U256::from(100));
        // TODO: Remove
        items.push(consts::MINIMUM_VOTING_REPUTATION, U256::from(10));
        // TODO: Add TimeBetweenInformalAndFormalVoting
        items.push(consts::VA_BID_ACCEPTANCE_TIMEOUT, U256::from(172800));
        items.push(consts::VA_CAN_BID_ON_PUBLIC_AUCTION, false);
        // TODO: Confirm value
        items.push(consts::JOB_SUBMIT_GRACE_PERIOD, U256::from(86400));
        // TODO: Confirm value
        items.push(consts::DEFAULT_REPUTATION_SLASH, U256::from(100));
        items
    }
}

pub mod events {
    use casper_dao_utils::casper_dao_macros::Event;
    use casper_types::bytesrepr::Bytes;

    #[derive(Debug, PartialEq, Eq, Event)]
    pub struct ValueUpdated {
        pub key: String,
        pub value: Bytes,
        pub activation_time: Option<u64>,
    }
}
