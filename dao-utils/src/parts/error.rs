macro_rules! dao_errors {
    ( $($name:ident $( ($optional_arg:ident) )? => $value:expr,)* ) => {
        #[doc = "All possible errors that can be raised by the utils crate."]
        #[derive(Debug, PartialEq)]
        pub enum Error {
            $($name$(($optional_arg))*),*
        }

        impl From<Error> for casper_types::ApiError {
            fn from(val: Error) -> Self {
                let id = match val {
                    $(Error::$name$( ( wildcard!($optional_arg) ) )* => $value),*
                };
                casper_types::ApiError::User(id)
            }
        }

        impl From<u16> for Error {
            fn from(val: u16) -> Self {
                match val {
                    $($value => Error::$name $( ($optional_arg::default()) )*),*,
                    _ => Error::Unknown,
                }
            }
        }
    };
}

macro_rules! wildcard {
    ($t:tt) => {
        _
    };
}

dao_errors!(
    InsufficientBalance => 404,
    InsufficientAllowance => 401,
    NotAnOwner => 1000,
    OwnerIsNotInitialized => 1001,
    NotWhitelisted => 1002,
    TotalSupplyOverflow => 1004,
    ValueNotAvailable => 1005,
    ActivationTimeInPast => 1006,
    ArithmeticOverflow => 1007,
    BytesConversionError => 1008,
    InvalidContext => 1099,
    Unknown => 1100,
    NoSuchMethod(String) => 1101,
    VariableValueNotSet => 1102,
    TokenDoesNotExist => 1700,
    TokenAlreadyExists => 1701,
    ApprovalToCurrentOwner => 1702,
    ApproveCallerIsNotOwnerNorApprovedForAll => 1703,
    CallerIsNotOwnerNorApproved => 1704,
    TransferToNonERC721ReceiverImplementer => 1705,
    TransferFromIncorrectOwner => 1706,
    ApproveToCaller => 1707,
    InvalidTokenOwner => 1708,
    InformalVotingTimeNotReached => 2101,
    FormalVotingTimeNotReached => 2102,
    VoteOnCompletedVotingNotAllowed => 2103,
    FinishingCompletedVotingNotAllowed => 2104,
    CannotVoteTwice => 2105,
    NotEnoughReputation => 2106,
    ContractToCallNotSet => 2107,
    VaOnboardedAlready => 2201,
    OnboardingAlreadyInProgress => 2202,
    VaNotOnboarded => 2203,
    VaNotKyced => 2204,
    UnexpectedOnboardingError => 2205,
    KycAlreadyInProgress => 2206,
    UserKycedAlready => 2207,
    UnexpectedKycError => 2208,
    MappingIndexDoesNotExist => 3404,
    BallotDoesNotExist => 3405,
    VoterDoesNotExist => 3406,
    VotingDoesNotExist => 3407,
    CannotPostJobForSelf => 4000,
    JobPosterNotKycd => 4001,
    WorkerNotKycd => 4002,
    CannotCancelJob => 4003,
    NotAuthorizedToSubmitResult => 4004,
    CannotAcceptJob => 4005,
    CannotSubmitJob => 4006,
    CannotVoteOnOwnJob => 4007,
    VotingNotStarted => 4008,
    JobAlreadySubmitted => 4009,
    NotOnboardedWorkerCannotStakeReputation => 4010,
    InvalidAddress => 5000,
    TransferError => 6000,
);
