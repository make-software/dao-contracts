//! Errors.

use std::process::exit;

use crate::log;

/// Errors enum.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Contract {0} not found in 'deployed_contracts.toml'.")]
    ContractAddressNotFound(String),

    #[error("Not a valid account: {0}.")]
    InvalidAccount(String),
}

impl Error {
    /// Returns error code.
    pub fn code(&self) -> i32 {
        match self {
            Error::ContractAddressNotFound(_) => 100,
            Error::InvalidAccount(_) => 101,
        }
    }

    /// Logs error message and exits with the given error code.
    pub fn print_and_die(&self) -> ! {
        log::error(self.to_string());
        exit(self.code());
    }
}
