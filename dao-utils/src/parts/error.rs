use casper_types::ApiError;

/// All possible errors that can be raised by the utils crate.
#[derive(Debug, PartialEq)]
pub enum Error {
    NotAnOwner,
    OwnerIsNotInitialized,
    NotWhitelisted,
    InsufficientBalance,
    InsufficientAllowance,
    TotalSupplyOverflow,
    ValueNotAvailable,
    ActivationTimeInPast,
    Unknown,
    InvalidContext,
    InformalQuorumNotReached,
    InformalVotingTimeNotReached,
    FormalQuorumNotReached,
    FormalVotingTimeNotReached,
}

impl From<Error> for ApiError {
    fn from(val: Error) -> Self {
        let id = match val {
            Error::InsufficientBalance => 404,
            Error::InsufficientAllowance => 401,
            Error::NotAnOwner => 1000,
            Error::OwnerIsNotInitialized => 1001,
            Error::NotWhitelisted => 1002,
            Error::TotalSupplyOverflow => 1004,
            Error::ValueNotAvailable => 1005,
            Error::ActivationTimeInPast => 1006,
            Error::InvalidContext => 1099,
            Error::Unknown => 1100,
            Error::InformalQuorumNotReached => 2100, // Voting errors start with 21xx
            Error::InformalVotingTimeNotReached => 2101,
            Error::FormalQuorumNotReached => 2102,
            Error::FormalVotingTimeNotReached => 2103,
        };
        ApiError::User(id)
    }
}

impl From<u16> for Error {
    fn from(val: u16) -> Self {
        match val {
            401 => Error::InsufficientAllowance,
            404 => Error::InsufficientBalance,
            1000 => Error::NotAnOwner,
            1001 => Error::OwnerIsNotInitialized,
            1002 => Error::NotWhitelisted,
            1004 => Error::TotalSupplyOverflow,
            1005 => Error::ValueNotAvailable,
            1006 => Error::ActivationTimeInPast,
            2100 => Error::InformalQuorumNotReached,
            2101 => Error::InformalVotingTimeNotReached,
            2102 => Error::FormalQuorumNotReached,
            2103 => Error::FormalVotingTimeNotReached,
            _ => Error::Unknown,
        }
    }
}
