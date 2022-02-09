extern crate alloc;

pub mod casper_env;
mod modules;
mod parts;
pub use casper_dao_macros;

pub use parts::address::Address;
pub use parts::consts;
pub use parts::error::Error;
pub use parts::mapping::Mapping;
pub use parts::variable::Variable;

pub use modules::owner;
pub use modules::staking;
pub use modules::token;
pub use modules::whitelist;

#[cfg(feature = "test-support")]
mod test_env;

#[cfg(feature = "test-support")]
pub use test_env::{ExecutionError, TestEnv};
