//! Contains Reputation Token Contract definition and related abstractions.
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

pub mod submodules {
    pub use super::{agg::*, balances::BalanceStorage, stakes::StakesStorage};
}
