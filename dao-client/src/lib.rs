//! This crate provides a library for interacting with the DAO.

pub mod actions;
pub mod cli;
mod dao_snapshot;
mod deployed_contracts_toml;
pub use dao_snapshot::DaoSnapshot;
pub use deployed_contracts_toml::DeployedContractsToml;
mod error;

// 1CSPR ~= 0.02924$
const DEFAULT_CSPR_USD_RATE: u64 = 34_000_000_000;

const DEPLOYED_CONTRACTS_FILE: &str = "deployed_contracts.toml";

fn cspr(amount: u64) -> u64 {
    amount * 1_000_000_000
}

mod log {
    /// Info message.
    pub fn info<T: AsRef<str>>(message: T) {
        prettycli::info(message.as_ref());
    }

    /// Error message.
    pub fn error<T: AsRef<str>>(message: T) {
        prettycli::error(message.as_ref());
    }
}
