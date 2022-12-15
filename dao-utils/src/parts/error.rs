macro_rules! dao_errors {
    ( $($name:ident $( ($optional_arg:ident) )? => $value:expr,)* ) => {
        #[doc = "All possible errors that can be raised by the utils crate."]
        #[derive(Debug, PartialEq, Eq)]
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
    InsufficientAllowance => 401,
    CannotDepositZeroAmount => 402,
    PurseBalanceMismatch => 403,
    InsufficientBalance => 404,
    PurseError => 405,
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
    BytesDeserializationError => 1103,
    TokenDoesNotExist => 1700,
    TokenAlreadyExists => 1701,
    ApprovalToCurrentOwner => 1702,
    ApproveCallerIsNotOwnerNorApprovedForAll => 1703,
    CallerIsNotOwnerNorApproved => 1704,
    TransferToNonERC721ReceiverImplementer => 1705,
    TransferFromIncorrectOwner => 1706,
    ApproveToCaller => 1707,
    InvalidTokenOwner => 1708,
    UserAlreadyOwnsToken => 1709,
    InformalVotingTimeNotReached => 2101,
    FormalVotingTimeNotReached => 2102,
    VoteOnCompletedVotingNotAllowed => 2103,
    FinishingCompletedVotingNotAllowed => 2104,
    CannotVoteTwice => 2105,
    NotEnoughReputation => 2106,
    ContractToCallNotSet => 2107,
    VotingDuringTimeBetweenVotingsNotAllowed => 2108,
    VotingNotCompleted => 2109,
    FormalVotingNotCompleted => 2110,
    InformalVotingNotStarted => 2111,
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
    ZeroStake => 3408,
    VotingAlreadyCanceled => 3409,
    OnlyReputationTokenContractCanCancel => 3410,
    SubjectOfSlashing => 3411,
    VotingAlreadyFinished => 3412,
    VotingWithGivenTypeNotInProgress => 3413,
    VotingIdNotFound => 3414,

    // Bid Escrow Errors.
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
    DosFeeTooLow => 4011,
    CannotBidOnOwnJob => 4012,
    PaymentExceedsMaxBudget => 4013,
    JobOfferNotFound => 4014,
    BidNotFound => 4015,
    JobNotFound => 4016,
    OnlyJobPosterCanPickABid => 4017,
    OnlyWorkerCanSubmitProof => 4018,
    InternalAuctionTimeExpired => 4019,
    PublicAuctionTimeExpired => 4020,
    PublicAuctionNotStarted => 4021,
    AuctionNotRunning => 4022,
    OnlyOnboardedWorkerCanBid => 4023,
    OnboardedWorkerCannotBid => 4024,
    CannotCancelBidBeforeAcceptanceTimeout => 4025,
    CannotCancelBidOnCompletedJobOffer => 4026,
    CannotCancelNotOwnedBid => 4027,
    CannotSubmitJobProof => 4028,
    GracePeriodNotStarted => 4029,
    OnlyJobPosterCanCancelJobOffer => 4030,
    JobOfferCannotBeYetCanceled => 4031,
    // Reputation Token Errors.
    CannotStakeTwice => 4500,
    VotingStakeDoesntExists => 4501,
    BidStakeDoesntExists => 4502,

    InvalidAddress => 5000,
    TransferError => 6000,

    ExpectedInformal => 7000,
    ExpectedFormalToBeOn => 7001,
);
