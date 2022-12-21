extern crate alloc;

// Reexport of casper-contract crate.
pub use casper_contract;

pub mod casper_env;
mod events;
pub mod instance;
mod parts;
pub use casper_dao_macros;
pub mod conversions;
pub mod cspr_rate;
pub mod definitions;
pub mod math;
pub mod transfer;

#[cfg(feature = "test-support")]
pub use conversions::BytesConversion;
pub use parts::{
    address::Address,
    collection::{List, OrderedCollection, Set},
    consts,
    contract_call::ContractCall,
    error::Error,
    mapping::{Mapping, VecMapping},
    sequence::SequenceGenerator,
    types::{BlockTime, DocumentHash},
    variable::Variable,
};

#[cfg(feature = "test-support")]
mod test_env;

#[cfg(feature = "test-support")]
pub use test_env::{ExecutionError, TestEnv};

#[cfg(feature = "test-support")]
mod test_contract;

#[cfg(feature = "test-support")]
pub use test_contract::TestContract;
