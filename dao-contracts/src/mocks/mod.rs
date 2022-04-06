mod mock_owner;
mod mock_repository;
mod mock_staking;
mod mock_token;
mod mock_voter;
mod mock_whitelist;

#[cfg(feature = "test-support")]
pub mod test {
    pub use super::mock_owner::MockOwnerTest;
    pub use super::mock_repository::MockRepositoryTest;
    pub use super::mock_staking::MockStakingTest;
    pub use super::mock_token::MockTokenTest;
    pub use super::mock_voter::MockVoterContractTest;
    pub use super::mock_whitelist::MockWhitelistTest;
}

pub use mock_owner::{MockOwner, MockOwnerCaller, MockOwnerInterface};
pub use mock_repository::{MockRepository, MockRepositoryCaller, MockRepositoryInterface};
pub use mock_staking::{MockStaking, MockStakingCaller, MockStakingInterface};
pub use mock_token::{MockToken, MockTokenCaller, MockTokenInterface};
pub use mock_voter::{MockVoterContract, MockVoterContractCaller, MockVoterContractInterface};
pub use mock_whitelist::{MockWhitelist, MockWhitelistCaller, MockWhitelistInterface};
