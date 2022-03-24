use casper_types::ApiError;

/// All possible errors that can be raised by the utils crate.
#[derive(Debug, PartialEq)]
pub enum Error {
    NotAnOwner,
    OwnerIsNotInitialized,
    NotWhitelisted,
    InsufficientBalance,
    TotalSupplyOverflow,
    ValueNotAvailable,
    ActivationTimeInPast,
    Unknown,
    InvalidContext,
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
            Error::ActivationTimeInPast => 1006,
            Error::InvalidContext => 1099,
            Error::Unknown => 1100,
        };
        ApiError::User(id)
    }
}

impl From<u16> for Error {
    fn from(val: u16) -> Self {
        match val {
            1000 => Error::NotAnOwner,
            1001 => Error::OwnerIsNotInitialized,
            1002 => Error::NotWhitelisted,
            1003 => Error::InsufficientBalance,
            1004 => Error::TotalSupplyOverflow,
            1005 => Error::ValueNotAvailable,
            1006 => Error::ActivationTimeInPast,
            _ => Error::Unknown,
        }
    }
}
