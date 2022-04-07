mod mock_owner;
mod mock_repository;
mod mock_staking;
mod mock_token;
mod mock_voter;
mod mock_whitelist;

#[cfg(feature = "test-support")]
pub mod test {
    pub use super::mock_owner::MockOwnerContractTest;
    pub use super::mock_repository::MockRepositoryTest;
    pub use super::mock_staking::MockStakingContractTest;
    pub use super::mock_token::MockTokenContractTest;
    pub use super::mock_voter::MockVoterContractTest;
    pub use super::mock_whitelist::MockWhitelistContractTest;
}

pub use mock_owner::{MockOwnerContract, MockOwnerContractCaller, MockOwnerContractInterface};
pub use mock_repository::{MockRepository, MockRepositoryCaller, MockRepositoryInterface};
pub use mock_staking::{MockStakingContract, MockStakingContractCaller, MockStakingContractInterface};
pub use mock_token::{MockTokenContract, MockTokenContractCaller, MockTokenContractInterface};
pub use mock_voter::{MockVoterContract, MockVoterContractCaller, MockVoterContractInterface};
pub use mock_whitelist::{
    MockWhitelistContract, MockWhitelistContractCaller, MockWhitelistContractInterface,
};
