use std::time::Duration;

use casper_dao_contracts::{
    mocks::test::MockVoterContractTest,
    voting::{
        consts as gv_consts, voting::Voting, Vote, VoteCast, VotingContractCreated, VotingCreated,
        VotingEnded, VotingId,
    },
    ReputationContractTest, VariableRepositoryContractTest,
};

use casper_dao_utils::{consts, Address, Error, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

pub fn into_bytes(val: &str) -> Bytes {
    val.as_bytes().into()
}

#[test]
fn test_contract_deploy() {
    let (_env, mock_voter_contract, variable_repo_contract, reputation_token_contract) = setup();

    assert_eq!(
        mock_voter_contract.get_variable_repo_address(),
        Address::from(variable_repo_contract.get_package_hash())
    );
    assert_eq!(
        mock_voter_contract.get_reputation_token_address(),
        Address::from(reputation_token_contract.get_package_hash())
    );

    mock_voter_contract.assert_event_at(
        0,
        VotingContractCreated {
            variable_repo: Address::from(variable_repo_contract.get_package_hash()),
            reputation_token: Address::from(reputation_token_contract.get_package_hash()),
            voter_contract: Address::from(mock_voter_contract.get_package_hash()),
        },
    )
}

#[test]
fn test_create_voting() {
    // Create voting
    let (env, mut mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_voting();

    // check voting event
    mock_voter_contract.assert_event_at(
        1,
        VotingCreated {
            creator: env.get_account(0),
            voting_id: VotingId::zero(),
            stake: U256::from(500),
        },
    );

    let voting_created_event: VotingCreated = mock_voter_contract.event(1);
    let vote_cast_event: VoteCast = mock_voter_contract.event(2);

    // check if voting was created correctly
    let voting: Voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);
    assert_eq!(voting.voting_id(), vote_cast_event.voting_id);
    assert_eq!(voting.voting_id(), VotingId::zero());
    assert_eq!(voting.formal_voting_time(), 432000000);
    assert_eq!(voting.formal_voting_quorum(), U256::from(3));
    assert_eq!(voting.informal_voting_time(), 86400000);
    assert_eq!(voting.informal_voting_quorum(), U256::from(3));
    assert_eq!(voting_created_event.voting_id, voting.voting_id());
    assert_eq!(voting_created_event.creator, env.get_account(0));
    assert_eq!(voting_created_event.stake, U256::from(500));

    // check if first vote was created correctly
    let vote: Vote = mock_voter_contract.get_vote(voting.voting_id(), env.get_account(0));
    assert_eq!(vote.voting_id, voting.voting_id());
    assert_eq!(vote.voter, Some(env.get_account(0)));
    assert_eq!(vote.choice, true);
    assert_eq!(vote.stake, U256::from(500));

    // check if first vote was created by a caller
    let first_voter = mock_voter_contract.get_voter(voting.voting_id(), 0);
    assert_eq!(first_voter, env.get_account(0));

    // check if the reputation was staked
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        500.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 - 500).into()
    );

    // check the voting counter after creating next voting
    mock_voter_contract
        .create_voting("some_other_value".to_string(), U256::from(321))
        .unwrap();
    let vote_cast_event: VoteCast = mock_voter_contract.event(4);
    let voting: Voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);
    assert_eq!(voting.voting_id(), VotingId::from(1));
}

#[test]
fn test_informal_before_end() {
    // create voting
    let (_env, mut mock_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_voting();

    // We cannot finish voting which time didn't elapse
    let voting_id = VotingId::zero();
    let result = mock_voter_contract.finish_voting(voting_id);

    assert_eq!(result.unwrap_err(), Error::InformalVotingTimeNotReached);
}

#[test]
fn test_informal_vote_without_a_quorum() {
    // create voting
    let (env, mut mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_voting();

    let voting_id = VotingId::zero();
    let voting: Voting = mock_voter_contract.get_voting(VotingId::zero());

    // cast a vote
    mock_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

    // Now the time should be fine, but a single vote should not reach quorum
    mock_voter_contract.finish_voting(voting_id).unwrap();
    mock_voter_contract.assert_last_event(VotingEnded {
        voting_id,
        result: gv_consts::INFORMAL_VOTING_QUORUM_NOT_REACHED.into(),
        votes_count: U256::from(2),
        stake_in_favor: U256::from(500),
        stake_against: U256::from(500),
        informal_voting_id: VotingId::zero(),
        formal_voting_id: None,
    });

    // voting status should be completed
    let voting: Voting = mock_voter_contract.get_voting(U256::zero());
    assert_eq!((voting.completed()), true);

    // cast a vote on a finished voting should return an error
    let result = mock_voter_contract.vote(U256::zero(), false, U256::from(500));
    assert_eq!(result.unwrap_err(), Error::VoteOnCompletedVotingNotAllowed);

    // the same goes for finishing voting
    let result = mock_voter_contract.finish_voting(voting_id);
    assert_eq!(
        result.unwrap_err(),
        Error::FinishingCompletedVotingNotAllowed
    );

    // creator's reputation should be burned and voters' returned
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        0.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 - 500).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(1)),
        10000.into()
    );
}

#[test]
fn test_informal_voting_rejected() {
    // create voting
    let (env, mut mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_voting();
    let voting_id = VotingId::zero();
    let voting: Voting = mock_voter_contract.get_voting(voting_id);

    // cast votes against
    mock_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, false, U256::from(500))
        .unwrap();
    mock_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // fast-forward
    env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

    // finish voting
    mock_voter_contract.finish_voting(voting_id).unwrap();

    // voting status should be completed
    let voting: Voting = mock_voter_contract.get_voting(voting_id);
    assert_eq!((voting.completed()), true);

    // the status should be rejected
    mock_voter_contract.assert_event_at(
        5,
        VotingEnded {
            voting_id,
            result: gv_consts::INFORMAL_VOTING_REJECTED.into(),
            votes_count: U256::from(3),
            stake_in_favor: U256::from(500),
            stake_against: U256::from(1000),
            informal_voting_id: VotingId::zero(),
            formal_voting_id: None,
        },
    );

    // creator's reputation should be burned and voters' returned
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        0.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 - 500).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(1)),
        10000.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(2)),
        10000.into()
    );
}

#[test]
fn test_informal_voting_converted() {
    // create voting
    let (env, mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::zero();

    // voting status should be completed
    let voting: Voting = mock_voter_contract.get_voting(voting_id);
    assert_eq!((voting.completed()), true);

    // new voting should be created with first creator
    mock_voter_contract.assert_event_at(
        -3,
        VotingCreated {
            creator: env.get_account(0),
            voting_id: VotingId::from(1),
            stake: 500.into(),
        },
    );

    // with initial vote as creator
    mock_voter_contract.assert_event_at(
        -2,
        VoteCast {
            voter: env.get_account(0),
            voting_id: VotingId::from(1),
            choice: true,
            stake: 500.into(),
        },
    );

    // the status should be converted
    mock_voter_contract.assert_last_event(VotingEnded {
        voting_id,
        result: gv_consts::INFORMAL_VOTING_PASSED.into(),
        votes_count: U256::from(3),
        stake_in_favor: U256::from(1000),
        stake_against: U256::from(500),
        informal_voting_id: VotingId::zero(),
        formal_voting_id: Some(VotingId::from(1)),
    });

    // creator's reputation should stay staked and voters' returned
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        500.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 - 500).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(1)),
        10000.into()
    );
}

#[test]
fn test_formal_voting_before_end() {
    let (_env, mut mock_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);

    let result = mock_voter_contract.finish_voting(voting_id);
    assert_eq!(result.unwrap_err(), Error::FormalVotingTimeNotReached);
}

#[test]
fn test_formal_vote_without_a_quorum() {
    let (env, mut mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = mock_voter_contract.get_voting(voting_id);

    // cast a vote
    mock_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));

    // Now the time should be fine, but a single vote should not reach quorum
    mock_voter_contract.finish_voting(voting_id).unwrap();
    mock_voter_contract.assert_last_event(VotingEnded {
        voting_id,
        result: gv_consts::FORMAL_VOTING_QUORUM_NOT_REACHED.into(),
        votes_count: U256::from(2),
        stake_in_favor: U256::from(500),
        stake_against: U256::from(500),
        informal_voting_id: VotingId::zero(),
        formal_voting_id: Some(voting_id),
    });

    // voting status should be completed
    let voting: Voting = mock_voter_contract.get_voting(voting_id);
    assert_eq!((voting.completed()), true);

    // cast a vote on a finished voting should return an error
    let result = mock_voter_contract.vote(voting_id, false, U256::from(500));
    assert_eq!(result.unwrap_err(), Error::VoteOnCompletedVotingNotAllowed);

    // the same goes for finishing voting
    let result = mock_voter_contract.finish_voting(voting_id);
    assert_eq!(
        result.unwrap_err(),
        Error::FinishingCompletedVotingNotAllowed
    );

    // creator's reputation should be burned and voters' returned
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        0.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 - 500).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(1)),
        10000.into()
    );
}

#[test]
fn test_formal_vote_rejected() {
    let (env, mut mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = mock_voter_contract.get_voting(voting_id);

    // vote to reach quorum
    mock_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, false, 1000.into())
        .unwrap();
    mock_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, 1000.into())
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));

    // Now the time should be fine, the result should be rejected
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        2500.into()
    );
    mock_voter_contract.finish_voting(voting_id).unwrap();
    mock_voter_contract.assert_last_event(VotingEnded {
        voting_id,
        result: gv_consts::FORMAL_VOTING_REJECTED.into(),
        votes_count: U256::from(3),
        stake_in_favor: U256::from(500),
        stake_against: U256::from(2000),
        informal_voting_id: VotingId::zero(),
        formal_voting_id: Some(voting_id),
    });

    // voting status should be completed
    let voting: Voting = mock_voter_contract.get_voting(voting_id);
    assert_eq!((voting.completed()), true);

    // creator's reputation should be transferred to voters proportionally
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
        0.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(0)),
        (10000 - 500).into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(1)),
        10250.into()
    );
    assert_eq!(
        reputation_token_contract.balance_of(env.get_account(2)),
        10250.into()
    );
}

#[test]
fn test_formal_vote_completed() {
    let (env, mut mock_voter_contract, _variable_repo_contract, reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = mock_voter_contract.get_voting(voting_id);

    // vote to reach quorum
    mock_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, 1000.into())
        .unwrap();
    mock_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, 1000.into())
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));

    // Now the time should be fine, the result should be completed
    mock_voter_contract.finish_voting(voting_id).unwrap();
    mock_voter_contract.assert_event_at(
        -1,
        VotingEnded {
            voting_id,
            result: gv_consts::FORMAL_VOTING_PASSED.into(),
            votes_count: U256::from(3),
            stake_in_favor: U256::from(1500),
            stake_against: U256::from(1000),
            informal_voting_id: VotingId::zero(),
            formal_voting_id: Some(voting_id),
        },
    );

    // voting status should be completed
    let voting: Voting = mock_voter_contract.get_voting(voting_id);
    assert_eq!((voting.completed()), true);

    // the action should be performed
    let variable = mock_voter_contract.get_variable();
    assert_eq!(variable, "some_value");

    // those who voted against' reputation should be transferred to for voters proportionally
    assert_eq!(
        reputation_token_contract.balance_of(mock_voter_contract.address()),
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
    assert_eq!(mock_voter_contract.get_dust_amount(), 1.into());
}

pub fn setup() -> (
    TestEnv,
    MockVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let env = TestEnv::new();
    let mut variable_repo_contract = VariableRepositoryContractTest::new(&env);
    let mut reputation_token_contract = ReputationContractTest::new(&env);
    let mock_voter_contract = MockVoterContractTest::new(
        &env,
        Address::from(variable_repo_contract.get_package_hash()),
        Address::from(reputation_token_contract.get_package_hash()),
    );

    variable_repo_contract
        .add_to_whitelist(mock_voter_contract.address())
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
        .add_to_whitelist(mock_voter_contract.address())
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
        mock_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}

pub fn create_voting() -> (
    TestEnv,
    MockVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let (env, mut mock_voter_contract, variable_repo_contract, reputation_token_contract) = setup();
    mock_voter_contract
        .create_voting("some_value".to_string(), U256::from(500))
        .unwrap();
    (
        env,
        mock_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}

pub fn create_formal_voting() -> (
    TestEnv,
    MockVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let (env, mut mock_voter_contract, variable_repo_contract, reputation_token_contract) =
        create_voting();
    let voting_id = VotingId::zero();
    let voting: Voting = mock_voter_contract.get_voting(voting_id);

    // cast votes
    mock_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, U256::from(500))
        .unwrap();
    mock_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // fast-forward
    env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

    // finish voting as somebody else
    mock_voter_contract
        .as_account(env.get_account(1))
        .finish_voting(voting_id)
        .unwrap();

    (
        env,
        mock_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}
