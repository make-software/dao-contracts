//! Contains Reputation Token Contract definition and related abstractions.
//!
//! New reputation is minted as a result of engagement in the DAO - voting or doing jobs.
mod agg;
mod balances;
mod stakes;
mod token;

#[cfg(feature = "test-support")]
pub use token::ReputationContractTest;
pub use token::{
    events::*,
    ReputationContract,
    ReputationContractCaller,
    ReputationContractInterface,
};

/// Building blocks of Reputation Token.
pub mod submodules {
    pub use super::{agg::*, balances::BalanceStorage, stakes::StakesStorage};
}
