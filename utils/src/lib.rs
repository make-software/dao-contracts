extern crate alloc;

mod parts;
mod modules;
pub mod casper_env;
pub use macros;

pub use parts::address::Address;
pub use parts::mapping::Mapping;
pub use parts::variable::Variable;
pub use parts::consts;
pub use parts::error::Error;

pub use modules::token;
pub use modules::owner;
pub use modules::staking;
pub use modules::whitelist;

#[cfg(feature = "test-support")]
mod test_env;

#[cfg(feature = "test-support")]
pub use test_env::{TestEnv, ExecutionError};
