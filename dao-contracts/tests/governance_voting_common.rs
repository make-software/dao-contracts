use casper_dao_contracts::{VariableRepositoryContractTest, ReputationContractTest, MockVoterContractTest};
use casper_dao_utils::{TestEnv, consts};
use casper_types::{U256, bytesrepr::ToBytes};

pub fn get_variable_repo_contract(env: &TestEnv, informal_voting_quorum: U256, formal_voting_quorum: U256, informal_voting_time: u64, formal_voting_time: u64, minimum_reputation: U256) -> VariableRepositoryContractTest {
    let mut variable_repo_contract = VariableRepositoryContractTest::new(&env);
    
    variable_repo_contract
        .update_at(
            consts::INFORMAL_VOTING_QUORUM.into(),
            informal_voting_quorum.to_bytes().unwrap().into(),
            None,
        )
        .unwrap();
    variable_repo_contract
        .update_at(
            consts::FORMAL_VOTING_QUORUM.into(),
            formal_voting_quorum.to_bytes().unwrap().into(),
            None,
        )
        .unwrap();
    variable_repo_contract
        .update_at(
            consts::MINIMUM_GOVERNANCE_REPUTATION.into(),
            minimum_reputation.to_bytes().unwrap().into(),
            None,
        )
        .unwrap();
    variable_repo_contract
        .update_at(
            consts::FORMAL_VOTING_TIME.into(),
            formal_voting_time.to_bytes().unwrap().into(),
            None,
        )
        .unwrap();
    variable_repo_contract
        .update_at(
            consts::INFORMAL_VOTING_TIME.into(),
            informal_voting_time.to_bytes().unwrap().into(),
            None,
        )
        .unwrap();

    variable_repo_contract
}

pub fn get_reputation_token_contract(env: &TestEnv, tokens: usize) -> ReputationContractTest {
    let mut reputation_token_contract = ReputationContractTest::new(&env);
    
    for i in 0..reputation_token_contract.total_onboarded().as_usize() {
        reputation_token_contract
        .mint(env.get_account(i), tokens.into())
        .unwrap();
    }

    reputation_token_contract
}

pub fn cast_votes(env: &TestEnv, contract: &mut MockVoterContractTest, voting_id: U256, votes: Vec<(bool, U256)>, initial_stake: U256) -> (U256, U256, U256) {
    let votes_count = votes.len() + 1;
    let stake_in_favor = initial_stake;
    let stake_against = U256::zero();
    for (i, (choice, stake)) in votes.iter().enumerate() {
        contract.as_account(env.get_account(i+1)).vote(voting_id, *choice, *stake).unwrap();
        if *choice {
            stake_in_favor.saturating_add(*stake);
        } else {
            stake_against.saturating_add(*stake);
        }
    }

    (votes_count.into(), stake_in_favor, stake_against)
}