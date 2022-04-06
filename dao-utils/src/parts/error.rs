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
    ArithmeticOverflow,
    Unknown,
    NoSuchMethod(String),
    InvalidContext,
    TokenDoesNotExist,
    TokenAlreadyExists,
    ApprovalToCurrentOwner,
    ApproveCallerIsNotOwnerNorApprovedForAll,
    CallerIsNotOwnerNorApproved,
    TransferToNonERC721ReceiverImplementer,
    TransferFromIncorrectOwner,
    ApproveToCaller,
    InvalidTokenOwner,
    InformalVotingTimeNotReached,
    FormalQuorumNotReached,
    FormalVotingTimeNotReached,
    VoteOnCompletedVotingNotAllowed,
    FinishingCompletedVotingNotAllowed,
    CannotVoteTwice,
    BytesConversionError,
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
            Error::ArithmeticOverflow => 1007,
            Error::BytesConversionError => 1008,
            Error::InvalidContext => 1099,
            Error::Unknown => 1100,
            Error::NoSuchMethod(_) => 1101,
            Error::TokenDoesNotExist => 1700,
            Error::TokenAlreadyExists => 1701,
            Error::ApprovalToCurrentOwner => 1702,
            Error::ApproveCallerIsNotOwnerNorApprovedForAll => 1703,
            Error::CallerIsNotOwnerNorApproved => 1704,
            Error::TransferToNonERC721ReceiverImplementer => 1705,
            Error::TransferFromIncorrectOwner => 1706,
            Error::ApproveToCaller => 1707,
            Error::InvalidTokenOwner => 1708,
            Error::InformalVotingTimeNotReached => 2101, // Voting errors start with 21xx
            Error::FormalQuorumNotReached => 2102,
            Error::FormalVotingTimeNotReached => 2103,
            Error::VoteOnCompletedVotingNotAllowed => 2104,
            Error::FinishingCompletedVotingNotAllowed => 2105,
            Error::CannotVoteTwice => 2106,
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
            1007 => Error::ArithmeticOverflow,
            1008 => Error::BytesConversionError,
            1700 => Error::TokenDoesNotExist,
            1701 => Error::TokenAlreadyExists,
            1702 => Error::ApprovalToCurrentOwner,
            1703 => Error::ApproveCallerIsNotOwnerNorApprovedForAll,
            1704 => Error::CallerIsNotOwnerNorApproved,
            1705 => Error::TransferToNonERC721ReceiverImplementer,
            1706 => Error::TransferFromIncorrectOwner,
            1707 => Error::ApproveToCaller,
            1708 => Error::InvalidTokenOwner,
            2101 => Error::InformalVotingTimeNotReached,
            2102 => Error::FormalQuorumNotReached,
            2103 => Error::FormalVotingTimeNotReached,
            2104 => Error::VoteOnCompletedVotingNotAllowed,
            2105 => Error::FinishingCompletedVotingNotAllowed,
            2106 => Error::CannotVoteTwice,
            _ => Error::Unknown,
        }
    }
}
