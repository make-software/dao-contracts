pub mod mock_voter;

#[cfg(feature = "test-support")]
pub use casper_dao_modules::mocks::{
    MockOwnerTest, MockRepositoryTest, MockStakingTest, MockTokenTest, MockWhitelistTest,
};
#[cfg(feature = "test-support")]
pub use mock_voter::MockVoterContractTest;
