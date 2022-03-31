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
    TokenDoesNotExist,
    ApprovalToCurrentOwner,
    ApproveCallerIsNotOwnerNorApprovedForAll,
    TransferCallerIsNotOwnerNorApproved,
    TransferToNonERC721ReceiverImplementer,
    TransferFromIncorrectOwner,
    TransferToNone,
    ApproveToCaller,
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
            Error::TokenDoesNotExist => 1700,
            Error::ApprovalToCurrentOwner => 1701,
            Error::ApproveCallerIsNotOwnerNorApprovedForAll => 1702,
            Error::TransferCallerIsNotOwnerNorApproved => 1703,
            Error::TransferToNonERC721ReceiverImplementer => 1704,
            Error::TransferFromIncorrectOwner => 1705,
            Error::TransferToNone => 1706,
            Error::ApproveToCaller => 1707,
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
            7000 => Error::TokenDoesNotExist,
            _ => Error::Unknown,
        }
    }
}
