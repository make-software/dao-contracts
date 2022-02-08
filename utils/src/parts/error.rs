use casper_types::ApiError;

#[cfg(feature = "test-support")]
pub use casper_execution_engine::core::execution::Error as ExecutionError;

pub enum Error {
    NotAnOwner,
    OwnerIsNotInitialized,
    NotWhitelisted,
    InsufficientBalance,
    TotalSupplyOverflow,
    ValueNotAvailable,
}

impl From<Error> for ApiError {
    fn from(val: Error) -> Self {
        let id = match val {
            Error::NotAnOwner => 1000,
            Error::OwnerIsNotInitialized => 1001,
            Error::NotWhitelisted => 1002,
            Error::InsufficientBalance => 1003,
            Error::TotalSupplyOverflow => 1004,
            Error::ValueNotAvailable => 1005,
        };
        ApiError::User(id)
    }
}
