use std::time::Duration;

use casper_dao_contracts::{
    voting::{voting::Voting, VotingContractCreated, VotingId},
    RepoVoterContractTest, ReputationContractTest, VariableRepositoryContractTest,
};

use casper_dao_utils::{consts, Address, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, FromBytes, ToBytes},
    U256,
};

#[test]
fn test_contract_deploy() {
    let (_env, repo_voter_contract, variable_repo_contract, reputation_token_contract) = setup();

    assert_eq!(
        repo_voter_contract.get_variable_repo_address(),
        Address::from(variable_repo_contract.get_package_hash())
    );
    assert_eq!(
        repo_voter_contract.get_reputation_token_address(),
        Address::from(reputation_token_contract.get_package_hash())
    );

    repo_voter_contract.assert_event_at(
        0,
        VotingContractCreated {
            variable_repo: Address::from(variable_repo_contract.get_package_hash()),
            reputation_token: Address::from(reputation_token_contract.get_package_hash()),
            voter_contract: Address::from(repo_voter_contract.get_package_hash()),
        },
    )
}

#[test]
fn test_action_performed() {
    let (env, mut repo_voter_contract, variable_repo_contract, reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = repo_voter_contract.get_voting(voting_id);

    // vote to reach quorum
    repo_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, 1000.into())
        .unwrap();
    repo_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, 1000.into())
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));

    // finish voting
    repo_voter_contract.finish_voting(voting_id).unwrap();

    // the action should be performed
    let bytes = variable_repo_contract.get("variable_name".into()).unwrap();
    let (variable, bytes) = String::from_bytes(&bytes).unwrap();
    assert_eq!(bytes.len(), 0);
    assert_eq!(variable, "new_value");

    // those who voted against' reputation should be transferred to for voters proportionally
    assert_eq!(
        reputation_token_contract.balance_of(repo_voter_contract.address()),
        1.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 + 333).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(1)),
        (10000 + 666).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(2)),
        (10000 - 1000).into()
    );

    // as the reputation was not divisible entirely, we check the dust amount
    assert_eq!(repo_voter_contract.get_dust_amount(), 1.into());
}

pub fn setup() -> (
    TestEnv,
    RepoVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let env = TestEnv::new();
    let mut variable_repo_contract = VariableRepositoryContractTest::new(&env);
    let mut reputation_token_contract = ReputationContractTest::new(&env);
    let repo_voter_contract = RepoVoterContractTest::new(
        &env,
        Address::from(variable_repo_contract.get_package_hash()),
        Address::from(reputation_token_contract.get_package_hash()),
    );

    variable_repo_contract
        .add_to_whitelist(repo_voter_contract.address())
        .unwrap();

    // TODO: Currently there are 4 hardcoded onboarded members, quorum at 750 should make the quorum when there are 3 votes
    variable_repo_contract
        .update_at(
            consts::INFORMAL_VOTING_QUORUM.into(),
            U256::from(750).to_bytes().unwrap().into(),
            None,
        )
        .unwrap();
    variable_repo_contract
        .update_at(
            consts::FORMAL_VOTING_QUORUM.into(),
            U256::from(750).to_bytes().unwrap().into(),
            None,
        )
        .unwrap();

    reputation_token_contract
        .add_to_whitelist(repo_voter_contract.address())
        .unwrap();

    reputation_token_contract
        .mint(env.get_account(0), 10000.into())
        .unwrap();
    reputation_token_contract
        .mint(env.get_account(1), 10000.into())
        .unwrap();
    reputation_token_contract
        .mint(env.get_account(2), 10000.into())
        .unwrap();

    (
        env,
        repo_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}

pub fn create_voting() -> (
    TestEnv,
    RepoVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let (env, mut repo_voter_contract, variable_repo_contract, reputation_token_contract) = setup();
    repo_voter_contract
        .create_voting(
            Address::from(variable_repo_contract.get_package_hash()),
            "variable_name".into(),
            Bytes::from("new_value".to_string().to_bytes().unwrap()),
            None,
            U256::from(500),
        )
        .unwrap();
    (
        env,
        repo_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}

pub fn create_formal_voting() -> (
    TestEnv,
    RepoVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let (env, mut repo_voter_contract, variable_repo_contract, reputation_token_contract) =
        create_voting();
    let voting_id = VotingId::zero();
    let voting: Voting = repo_voter_contract.get_voting(voting_id);

    // cast votes
    repo_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, U256::from(500))
        .unwrap();
    repo_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // fast-forward
    env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

    // finish voting as somebody else
    repo_voter_contract
        .as_account(env.get_account(1))
        .finish_voting(voting_id)
        .unwrap();

    (
        env,
        repo_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}
