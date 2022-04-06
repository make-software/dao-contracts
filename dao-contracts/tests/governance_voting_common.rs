use casper_dao_contracts::{ReputationContractTest, VariableRepositoryContractTest};
use casper_dao_utils::{consts, TestEnv};
use casper_types::{bytesrepr::ToBytes, U256};

pub fn get_variable_repo_contract(
    env: &TestEnv,
    informal_voting_quorum: U256,
    formal_voting_quorum: U256,
    informal_voting_time: u64,
    formal_voting_time: u64,
    minimum_reputation: U256,
) -> VariableRepositoryContractTest {
    let mut variable_repo_contract = VariableRepositoryContractTest::new(env);

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
    let mut reputation_token_contract = ReputationContractTest::new(env);

    for i in 0..reputation_token_contract.total_onboarded().as_usize() {
        reputation_token_contract
            .mint(env.get_account(i), tokens.into())
            .unwrap();
    }

    reputation_token_contract
}
