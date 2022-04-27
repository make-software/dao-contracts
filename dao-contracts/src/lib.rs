pub mod action;
mod admin;
mod dao_nft;
mod kyc_voter;
#[doc(hidden)]
pub mod mocks;
mod onboarding_voter;
mod repo_voter;
mod reputation;
mod variable_repository;
/// Utilities to manage the voting process
pub mod voting;

pub use admin::{AdminContract, AdminContractCaller, AdminContractInterface};
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
pub use variable_repository::{
    VariableRepositoryContract, VariableRepositoryContractCaller,
    VariableRepositoryContractInterface,
};

#[cfg(feature = "test-support")]
pub use reputation::ReputationContractTest;

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
