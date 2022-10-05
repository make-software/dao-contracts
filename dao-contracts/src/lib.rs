//! A selection of contracts implemented for usage in DAO

#[doc(hidden)]
pub mod action;
mod admin;
pub mod bid;
mod bid_escrow;
mod builder;
mod kyc_nft;
mod kyc_voter;
mod va_nft;

#[doc(hidden)]
pub mod mocks;
pub mod onboarding_voter;
pub mod repo_voter;
pub mod reputation;
pub mod reputation_voter;
pub mod simple_voter;
pub mod variable_repository;
pub mod voting;

pub use admin::{AdminContract, AdminContractCaller, AdminContractInterface};
pub use bid_escrow::{BidEscrowContract, BidEscrowContractCaller, BidEscrowContractInterface};
pub use builder::voting_configuration_builder::VotingConfigurationBuilder;
pub use kyc_nft::{KycNftContract, KycNftContractCaller, KycNftContractInterface};
pub use kyc_voter::{KycVoterContract, KycVoterContractCaller, KycVoterContractInterface};
#[doc(hidden)]
pub use mocks::mock_voter::{
    MockVoterContract, MockVoterContractCaller, MockVoterContractInterface,
};
pub use onboarding_voter::{
    OnboardingVoterContract, OnboardingVoterContractCaller, OnboardingVoterContractInterface,
};
pub use repo_voter::{RepoVoterContract, RepoVoterContractCaller, RepoVoterContractInterface};
pub use reputation::{ReputationContract, ReputationContractCaller, ReputationContractInterface};
pub use reputation_voter::{
    ReputationVoterContract, ReputationVoterContractCaller, ReputationVoterContractInterface,
};
pub use va_nft::{VaNftContract, VaNftContractCaller, VaNftContractInterface};
pub use variable_repository::{
    VariableRepositoryContract, VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};

pub use simple_voter::{
    SimpleVoterContract, SimpleVoterContractCaller, SimpleVoterContractInterface,
};

#[cfg(feature = "test-support")]
pub use reputation::ReputationContractTest;

#[cfg(feature = "test-support")]
pub use reputation_voter::ReputationVoterContractTest;

#[cfg(feature = "test-support")]
pub use variable_repository::VariableRepositoryContractTest;

#[cfg(feature = "test-support")]
pub use repo_voter::RepoVoterContractTest;

#[doc(hidden)]
#[cfg(feature = "test-support")]
pub use admin::AdminContractTest;

#[cfg(feature = "test-support")]
pub use bid_escrow::BidEscrowContractTest;

#[cfg(feature = "test-support")]
pub use mocks::mock_voter::MockVoterContractTest;

#[cfg(feature = "test-support")]
pub use onboarding_voter::OnboardingVoterContractTest;

#[cfg(feature = "test-support")]
pub use va_nft::VaNftContractTest;

#[cfg(feature = "test-support")]
pub use kyc_nft::KycNftContractTest;

#[cfg(feature = "test-support")]
pub use kyc_voter::KycVoterContractTest;

#[cfg(feature = "test-support")]
pub use simple_voter::SimpleVoterContractTest;
