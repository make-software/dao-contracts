//! Repository module.
use crate::modules::repository::events::ValueUpdated;
use crate::utils::consts;
use crate::utils::Error::{ActivationTimeInPast, KeyValueStorageError};
use odra::contract_env::{get_block_time, revert};
use odra::types::event::OdraEvent;
use odra::types::{Address, Balance, Bytes, OdraType as OdraTyped};
use odra::{List, Mapping, OdraType, UnwrapOrRevert};

/// A data struct stored in the repository.
///
/// The first value represents the current value.
///
/// The second value is an optional tuple consisting of the future value and its activation time.
#[derive(OdraType)]
pub struct Record {
    pub current_value: Bytes,
    pub next_value: Option<(Bytes, u64)>,
}

/// A module that stores the DAO configuration.
///
/// The modules stores key-value pairs and a set of keys.
/// The repository is initialized with the default values.
#[odra::module(events = [ValueUpdated])]
pub struct Repository {
    pub storage: Mapping<String, Record>,
    pub keys2: List<String>,
}

#[odra::module]
impl Repository {
    #[odra(init)]
    pub fn init(
        &mut self,
        fiat_conversion: Address,
        bid_escrow_wallet: Address,
        voting_ids: Address,
    ) {
        let mut config = RepositoryDefaults::default();
        config.push(consts::FIAT_CONVERSION_RATE_ADDRESS, fiat_conversion);
        config.push(consts::BID_ESCROW_WALLET_ADDRESS, bid_escrow_wallet);
        config.push(consts::VOTING_IDS_ADDRESS, voting_ids);
        for (key, value) in config.items() {
            self.set(key, value);
        }
    }

    pub fn update_at(&mut self, key: String, value: Bytes, activation_time: Option<u64>) {
        let now = get_block_time();
        let value_for_event = value.clone();
        let new_value: Record = match activation_time {
            // If no activation_time provided update the record to the value from argument.
            None => Record {
                current_value: value,
                next_value: None,
            },

            // If activation_time is in the past, raise an error.
            Some(activation_time) if activation_time < now => revert(ActivationTimeInPast),

            // If activation time is in future.
            Some(activation_time) => {
                // Load the record.
                let record = self
                    .storage
                    .get(&key)
                    .unwrap_or_revert_with(KeyValueStorageError);
                let current_value = record.current_value;
                let current_next_value = record.next_value;
                match current_next_value {
                    // If current_next_value is not set, update it to the value from arguments.
                    None => Record {
                        current_value,
                        next_value: Some((value, activation_time)),
                    },

                    // If current_next_value is set, but it is in the past, make it a current
                    // value and set next_value to values from arguments.
                    Some((current_next_value, current_activation_time))
                        if current_activation_time < now =>
                    {
                        Record {
                            current_value: current_next_value,
                            next_value: Some((value, activation_time)),
                        }
                    }

                    // If current_next_value is set in future, update it.
                    Some(_) => Record {
                        current_value,
                        next_value: Some((value, activation_time)),
                    },
                }
            }
        };
        self.storage.set(&key, new_value);
        self.keys2.push(key.clone());
        ValueUpdated {
            key,
            value: value_for_event,
            activation_time,
        }
        .emit();
    }

    pub fn get(&self, key: String) -> Option<Bytes> {
        let record = self.storage.get(&key)?;
        let now = get_block_time();
        if let Some((value, activation_time)) = record.next_value {
            if now > activation_time {
                return Some(value);
            }
        }
        Some(record.current_value)
    }

    pub fn get_full_value(&self, key: String) -> Option<Record> {
        self.storage.get(&key)
    }

    fn set(&mut self, key: String, value: Bytes) {
        self.update_at(key, value, None);
    }
}

struct RepositoryDefaults {
    pub items: Vec<(String, Bytes)>,
}

impl RepositoryDefaults {
    pub fn push<T: OdraTyped>(&mut self, key: &str, value: T) {
        self.items
            .push((key.to_string(), value.serialize().unwrap().into()));
    }

    pub fn items(self) -> Vec<(String, Bytes)> {
        self.items
    }
}

impl Default for RepositoryDefaults {
    fn default() -> Self {
        let mut items = RepositoryDefaults { items: vec![] };
        items.push(consts::POST_JOB_DOS_FEE, Balance::from(10000));
        items.push(consts::INTERNAL_AUCTION_TIME, 604800000u64);
        items.push(consts::PUBLIC_AUCTION_TIME, 864000000u64);
        items.push(consts::DEFAULT_POLICING_RATE, Balance::from(300));
        items.push(consts::REPUTATION_CONVERSION_RATE, Balance::from(100));
        items.push(consts::FORUM_KYC_REQUIRED, true);
        items.push(consts::BID_ESCROW_INFORMAL_QUORUM_RATIO, Balance::from(500));
        items.push(consts::BID_ESCROW_FORMAL_QUORUM_RATIO, Balance::from(500));
        items.push(consts::INFORMAL_QUORUM_RATIO, Balance::from(500));
        items.push(consts::FORMAL_QUORUM_RATIO, Balance::from(500));
        items.push(consts::BID_ESCROW_INFORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::BID_ESCROW_FORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::INFORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::FORMAL_VOTING_TIME, 432000000u64);
        items.push(consts::INFORMAL_STAKE_REPUTATION, true);
        items.push(consts::TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING, 86400000u64);
        items.push(consts::VA_BID_ACCEPTANCE_TIMEOUT, 172800000u64);
        items.push(consts::VA_CAN_BID_ON_PUBLIC_AUCTION, false);
        items.push(consts::DISTRIBUTE_PAYMENT_TO_NON_VOTERS, true);
        items.push(consts::DEFAULT_REPUTATION_SLASH, Balance::from(100));
        items.push(consts::VOTING_CLEARNESS_DELTA, Balance::from(8));
        items.push(
            consts::VOTING_START_AFTER_JOB_WORKER_SUBMISSION,
            259200000u64,
        );
        items.push(consts::BID_ESCROW_PAYMENT_RATIO, Balance::from(100));
        items
    }
}

pub mod events {
    use odra::types::Bytes;
    use odra::Event;

    /// Event emitted when the repository value has been changed.
    #[derive(Event, PartialEq, Eq, Debug)]
    pub struct ValueUpdated {
        pub key: String,
        pub value: Bytes,
        pub activation_time: Option<u64>,
    }
}
