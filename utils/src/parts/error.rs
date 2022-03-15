use casper_types::ApiError;

/// All possible errors that can be raised by the utils crate.
pub enum Error {
    NotAnOwner,
    OwnerIsNotInitialized,
    NotWhitelisted,
    InsufficientBalance,
    TotalSupplyOverflow,
}

impl From<Error> for ApiError {
    fn from(val: Error) -> Self {
        let id = match val {
            Error::NotAnOwner => 1000,
            Error::OwnerIsNotInitialized => 1001,
            Error::NotWhitelisted => 1002,
            Error::InsufficientBalance => 1003,
            Error::TotalSupplyOverflow => 1004,
        };
        ApiError::User(id)
    }
}
