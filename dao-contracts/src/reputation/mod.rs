//! Utilities related to [ReputationContract](crate::ReputationContract).
mod agg;
mod balances;
mod stakes;
mod token;

#[cfg(feature = "test-support")]
pub use token::ReputationContractTest;
pub use token::{ReputationContract, ReputationContractInterface, ReputationContractCaller};

pub mod modules {
    pub use super::agg::*;
    pub use super::balances::BalanceStorage;
    pub use super::stakes::StakesStorage;
}
