//! A selection of contracts implemented for usage in DAO

#[doc(hidden)]
pub mod action;
pub mod admin;
pub mod builder;
pub mod dao_nft;
pub mod kyc_voter;

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
pub use builder::voting_configuration_builder::VotingConfigurationBuilder;
pub use dao_nft::{DaoOwnedNftContract, DaoOwnedNftContractCaller, DaoOwnedNftContractInterface};
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
pub use mocks::mock_voter::MockVoterContractTest;

#[cfg(feature = "test-support")]
pub use onboarding_voter::OnboardingVoterContractTest;

#[cfg(feature = "test-support")]
pub use dao_nft::DaoOwnedNftContractTest;

#[cfg(feature = "test-support")]
pub use kyc_voter::KycVoterContractTest;
