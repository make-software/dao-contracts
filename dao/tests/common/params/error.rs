use std::ops::Deref;
use std::str::FromStr;

pub struct Error(dao::utils::Error);

impl Deref for Error {
    type Target = dao::utils::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = match s {
            "InsufficientAllowance" => dao::utils::Error::InsufficientAllowance,
            "CannotDepositZeroAmount" => dao::utils::Error::CannotDepositZeroAmount,
            "PurseBalanceMismatch" => dao::utils::Error::PurseBalanceMismatch,
            "InsufficientBalance" => dao::utils::Error::InsufficientBalance,
            "PurseError" => dao::utils::Error::PurseError,
            "NotAnOwner" => dao::utils::Error::NotAnOwner,
            "OwnerIsNotInitialized" => dao::utils::Error::OwnerIsNotInitialized,
            "NotWhitelisted" => dao::utils::Error::NotWhitelisted,
            "TotalSupplyOverflow" => dao::utils::Error::TotalSupplyOverflow,
            "ValueNotAvailable" => dao::utils::Error::ValueNotAvailable,
            "ActivationTimeInPast" => dao::utils::Error::ActivationTimeInPast,
            "ArithmeticOverflow" => dao::utils::Error::ArithmeticOverflow,
            "BytesConversionError" => dao::utils::Error::BytesConversionError,
            "MappingItemNotAvailable" => dao::utils::Error::MappingItemNotAvailable,
            "TypeMismatch" => dao::utils::Error::TypeMismatch,
            "InvalidContext" => dao::utils::Error::InvalidContext,
            "Unknown" => dao::utils::Error::Unknown,
            "NoSuchMethod" => dao::utils::Error::NoSuchMethod,
            "VariableValueNotSet" => dao::utils::Error::VariableValueNotSet,
            "BytesDeserializationError" => dao::utils::Error::BytesDeserializationError,
            "TokenDoesNotExist" => dao::utils::Error::TokenDoesNotExist,
            "TokenAlreadyExists" => dao::utils::Error::TokenAlreadyExists,
            "ApprovalToCurrentOwner" => dao::utils::Error::ApprovalToCurrentOwner,
            "ApproveCallerIsNotOwnerNorApprovedForAll" => {
                dao::utils::Error::ApproveCallerIsNotOwnerNorApprovedForAll
            }
            "CallerIsNotOwnerNorApproved" => dao::utils::Error::CallerIsNotOwnerNorApproved,
            "TransferToNonERC721ReceiverImplementer" => {
                dao::utils::Error::TransferToNonERC721ReceiverImplementer
            }
            "TransferFromIncorrectOwner" => dao::utils::Error::TransferFromIncorrectOwner,
            "ApproveToCaller" => dao::utils::Error::ApproveToCaller,
            "InvalidTokenOwner" => dao::utils::Error::InvalidTokenOwner,
            "UserAlreadyOwnsToken" => dao::utils::Error::UserAlreadyOwnsToken,
            "InformalVotingTimeNotReached" => dao::utils::Error::InformalVotingTimeNotReached,
            "FormalVotingTimeNotReached" => dao::utils::Error::FormalVotingTimeNotReached,
            "VoteOnCompletedVotingNotAllowed" => dao::utils::Error::VoteOnCompletedVotingNotAllowed,
            "FinishingCompletedVotingNotAllowed" => {
                dao::utils::Error::FinishingCompletedVotingNotAllowed
            }
            "CannotVoteTwice" => dao::utils::Error::CannotVoteTwice,
            "NotEnoughReputation" => dao::utils::Error::NotEnoughReputation,
            "ContractToCallNotSet" => dao::utils::Error::ContractToCallNotSet,
            "VotingDuringTimeBetweenVotingsNotAllowed" => {
                dao::utils::Error::VotingDuringTimeBetweenVotingsNotAllowed
            }
            "VotingNotCompleted" => dao::utils::Error::VotingNotCompleted,
            "FormalVotingNotCompleted" => dao::utils::Error::FormalVotingNotCompleted,
            "InformalVotingNotStarted" => dao::utils::Error::InformalVotingNotStarted,
            "VaOnboardedAlready" => dao::utils::Error::VaOnboardedAlready,
            "OnboardingAlreadyInProgress" => dao::utils::Error::OnboardingAlreadyInProgress,
            "NotOnboarded" => dao::utils::Error::NotOnboarded,
            "NotKyced" => dao::utils::Error::NotKyced,
            "UnexpectedOnboardingError" => dao::utils::Error::UnexpectedOnboardingError,
            "KycAlreadyInProgress" => dao::utils::Error::KycAlreadyInProgress,
            "UserKycedAlready" => dao::utils::Error::UserKycedAlready,
            "UnexpectedKycError" => dao::utils::Error::UnexpectedKycError,
            "MappingIndexDoesNotExist" => dao::utils::Error::MappingIndexDoesNotExist,
            "BallotDoesNotExist" => dao::utils::Error::BallotDoesNotExist,
            "VoterDoesNotExist" => dao::utils::Error::VoterDoesNotExist,
            "VotingDoesNotExist" => dao::utils::Error::VotingDoesNotExist,
            "ZeroStake" => dao::utils::Error::ZeroStake,
            "VotingAlreadyCanceled" => dao::utils::Error::VotingAlreadyCanceled,
            "OnlyReputationTokenContractCanCancel" => {
                dao::utils::Error::OnlyReputationTokenContractCanCancel
            }
            "SubjectOfSlashing" => dao::utils::Error::SubjectOfSlashing,
            "VotingAlreadyFinished" => dao::utils::Error::VotingAlreadyFinished,
            "VotingWithGivenTypeNotInProgress" => {
                dao::utils::Error::VotingWithGivenTypeNotInProgress
            }
            "VotingIdNotFound" => dao::utils::Error::VotingIdNotFound,
            "VotingAddressNotFound" => dao::utils::Error::VotingAddressNotFound,
            "CannotPostJobForSelf" => dao::utils::Error::CannotPostJobForSelf,
            "JobPosterNotKycd" => dao::utils::Error::JobPosterNotKycd,
            "WorkerNotKycd" => dao::utils::Error::WorkerNotKycd,
            "CannotCancelJob" => dao::utils::Error::CannotCancelJob,
            "NotAuthorizedToSubmitResult" => dao::utils::Error::NotAuthorizedToSubmitResult,
            "CannotAcceptJob" => dao::utils::Error::CannotAcceptJob,
            "CannotSubmitJob" => dao::utils::Error::CannotSubmitJob,
            "CannotVoteOnOwnJob" => dao::utils::Error::CannotVoteOnOwnJob,
            "VotingNotStarted" => dao::utils::Error::VotingNotStarted,
            "JobAlreadySubmitted" => dao::utils::Error::JobAlreadySubmitted,
            "NotOnboardedWorkerCannotStakeReputation" => {
                dao::utils::Error::NotOnboardedWorkerCannotStakeReputation
            }
            "DosFeeTooLow" => dao::utils::Error::DosFeeTooLow,
            "CannotBidOnOwnJob" => dao::utils::Error::CannotBidOnOwnJob,
            "PaymentExceedsMaxBudget" => dao::utils::Error::PaymentExceedsMaxBudget,
            "JobOfferNotFound" => dao::utils::Error::JobOfferNotFound,
            "BidNotFound" => dao::utils::Error::BidNotFound,
            "JobNotFound" => dao::utils::Error::JobNotFound,
            "OnlyJobPosterCanPickABid" => dao::utils::Error::OnlyJobPosterCanPickABid,
            "OnlyWorkerCanSubmitProof" => dao::utils::Error::OnlyWorkerCanSubmitProof,
            "InternalAuctionTimeExpired" => dao::utils::Error::InternalAuctionTimeExpired,
            "PublicAuctionTimeExpired" => dao::utils::Error::PublicAuctionTimeExpired,
            "PublicAuctionNotStarted" => dao::utils::Error::PublicAuctionNotStarted,
            "AuctionNotRunning" => dao::utils::Error::AuctionNotRunning,
            "OnlyOnboardedWorkerCanBid" => dao::utils::Error::OnlyOnboardedWorkerCanBid,
            "OnboardedWorkerCannotBid" => dao::utils::Error::OnboardedWorkerCannotBid,
            "CannotCancelBidBeforeAcceptanceTimeout" => {
                dao::utils::Error::CannotCancelBidBeforeAcceptanceTimeout
            }
            "CannotCancelBidOnCompletedJobOffer" => {
                dao::utils::Error::CannotCancelBidOnCompletedJobOffer
            }
            "CannotCancelNotOwnedBid" => dao::utils::Error::CannotCancelNotOwnedBid,
            "CannotSubmitJobProof" => dao::utils::Error::CannotSubmitJobProof,
            "GracePeriodNotStarted" => dao::utils::Error::GracePeriodNotStarted,
            "CannotCancelNotOwnedJobOffer" => dao::utils::Error::CannotCancelNotOwnedJobOffer,
            "JobOfferCannotBeYetCanceled" => dao::utils::Error::JobOfferCannotBeYetCanceled,
            "JobCannotBeYetCanceled" => dao::utils::Error::JobCannotBeYetCanceled,
            "FiatRateNotSet" => dao::utils::Error::FiatRateNotSet,
            "OnlyJobPosterCanModifyJobOffer" => dao::utils::Error::OnlyJobPosterCanModifyJobOffer,
            "CannotStakeTwice" => dao::utils::Error::CannotStakeTwice,
            "VotingStakeDoesntExists" => dao::utils::Error::VotingStakeDoesntExists,
            "BidStakeDoesntExists" => dao::utils::Error::BidStakeDoesntExists,
            "InvalidAddress" => dao::utils::Error::InvalidAddress,
            "RepositoryError" => dao::utils::Error::RepositoryError,
            "KeyValueStorageError" => dao::utils::Error::KeyValueStorageError,
            "DictionaryStorageError" => dao::utils::Error::DictionaryStorageError,
            "StorageError" => dao::utils::Error::StorageError,
            "VMInternalError" => dao::utils::Error::VMInternalError,
            "CLValueError" => dao::utils::Error::CLValueError,
            "TransferError" => dao::utils::Error::TransferError,
            "ExpectedInformal" => dao::utils::Error::ExpectedInformal,
            "ExpectedFormalToBeOn" => dao::utils::Error::ExpectedFormalToBeOn,
            _ => return Err(String::from("Parsing error")),
        };
        Ok(Error(error))
    }
}
