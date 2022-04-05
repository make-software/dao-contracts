use std::time::Duration;

use casper_dao_contracts::{
    voting::{voting::Voting, VotingContractCreated, VotingId},
    AdminContractTest, ReputationContractTest, VariableRepositoryContractTest, action::Action,
};

use casper_dao_utils::{consts, Address, TestEnv};
use casper_types::{
    bytesrepr::{ToBytes},
    U256,
};

#[test]
fn test_contract_deploy() {
    let (_env, admin_contract, variable_repo_contract, reputation_token_contract) = setup();

    assert_eq!(
        admin_contract.get_variable_repo_address(),
        Address::from(variable_repo_contract.get_package_hash())
    );
    assert_eq!(
        admin_contract.get_reputation_token_address(),
        Address::from(reputation_token_contract.get_package_hash())
    );

    admin_contract.assert_event_at(
        0,
        VotingContractCreated {
            variable_repo: Address::from(variable_repo_contract.get_package_hash()),
            reputation_token: Address::from(reputation_token_contract.get_package_hash()),
            voter_contract: Address::from(admin_contract.get_package_hash()),
        },
    )
}

#[test]
fn test_action_performed() {
    let (env, mut admin_contract, _variable_repo_contract, reputation_token_contract) = setup();
    let voting_id = VotingId::from(1);
    let voting: Voting = admin_contract.get_voting(voting_id);

    // vote to reach quorum
    admin_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, 1000.into())
        .unwrap();
    admin_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, 1000.into())
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));

    // Before finishing, our address shouldn't be whitelisted
    assert!(!reputation_token_contract.is_whitelisted(env.get_account(1)));

    // finish voting
    admin_contract.finish_voting(voting_id).unwrap();

    // the action should be performed
    assert!(reputation_token_contract.is_whitelisted(env.get_account(1)));
}

pub fn setup() -> (
    TestEnv,
    AdminContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let env = TestEnv::new();
    let mut variable_repo_contract = VariableRepositoryContractTest::new(&env);
    let mut reputation_token_contract = ReputationContractTest::new(&env);
    let mut admin_contract = AdminContractTest::new(
        &env,
        Address::from(variable_repo_contract.get_package_hash()),
        Address::from(reputation_token_contract.get_package_hash()),
    );

    variable_repo_contract
        .add_to_whitelist(admin_contract.address())
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
        .change_ownership(admin_contract.address())
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

    admin_contract
        .create_voting(
            Address::from(reputation_token_contract.get_package_hash()),
            Action::AddToWhitelist,
            env.get_account(1),
            U256::from(500),
        )
        .unwrap();

    let voting_id = VotingId::zero();
    let voting: Voting = admin_contract.get_voting(voting_id);

    // cast votes
    admin_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, U256::from(500))
        .unwrap();
    admin_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // fast-forward
    env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

    // finish voting as somebody else
    admin_contract
        .as_account(env.get_account(1))
        .finish_voting(voting_id)
        .unwrap();

    (
        env,
        admin_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}
