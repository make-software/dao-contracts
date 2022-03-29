use std::time::Duration;

use casper_dao_contracts::{
    RepoVoterContractTest, ReputationContractTest, VariableRepositoryContractTest,
};
use casper_dao_modules::{
    events::{
        FormalVotingEnded, InformalVotingEnded, VoteCast, VotingContractCreated, VotingCreated,
    },
    vote::Vote,
    voting::Voting,
    VotingId,
};
use casper_dao_utils::{consts, Address, Error, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, FromBytes, ToBytes},
    RuntimeArgs, U256,
};

#[test]
fn test_voting_serialization() {
    let voting = Voting {
        voting_id: VotingId::from(1),
        informal_voting_id: VotingId::from(1),
        formal_voting_id: None,
        informal_voting_quorum: U256::from(2),
        stake_in_favor: U256::from(0),
        stake_against: U256::from(0),
        completed: false,
        formal_voting_quorum: U256::from(2),
        formal_voting_time: U256::from(2),
        informal_voting_time: U256::from(2),
        contract_to_call: None,
        entry_point: "update_variable".into(),
        runtime_args: RuntimeArgs::new(),
        minimum_governance_reputation: U256::from(2),
        finish_time: U256::from(123),
    };

    let (voting2, _bytes) = Voting::from_bytes(&voting.to_bytes().unwrap()).unwrap();

    assert_eq!(voting.voting_id, voting2.voting_id);
    assert_eq!(voting.informal_voting_id, voting2.informal_voting_id);
    assert_eq!(voting.formal_voting_id, voting2.formal_voting_id);
    assert_eq!(
        voting.informal_voting_quorum,
        voting2.informal_voting_quorum
    );
    assert_eq!(voting.formal_voting_quorum, voting2.formal_voting_quorum);
    assert_eq!(voting.stake_against, voting2.stake_against);
    assert_eq!(voting.stake_in_favor, voting2.stake_in_favor);
    assert_eq!(voting.completed, voting2.completed);
    assert_eq!(voting.contract_to_call, voting2.contract_to_call);
    assert_eq!(voting.entry_point, voting2.entry_point);
    assert_eq!(voting.runtime_args, voting2.runtime_args);
    assert_eq!(voting.formal_voting_time, voting2.formal_voting_time);
    assert_eq!(voting.informal_voting_time, voting2.informal_voting_time);
    assert_eq!(
        voting.minimum_governance_reputation,
        voting2.minimum_governance_reputation
    );
    assert_eq!(voting.finish_time, voting2.finish_time);
}

#[test]
fn test_vote_serialization() {
    let env = TestEnv::new();
    let vote = Vote {
        voter: Some(env.get_account(0)),
        voting_id: U256::from(123),
        choice: true,
        stake: U256::from(456),
    };

    let (vote2, _bytes) = Vote::from_bytes(&vote.to_bytes().unwrap()).unwrap();
    assert_eq!(vote.voter, vote2.voter);
    assert_eq!(vote.voting_id, vote2.voting_id);
    assert_eq!(vote.choice, vote2.choice);
    assert_eq!(vote.stake, vote2.stake);
}

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
            repo_voter: Address::from(repo_voter_contract.get_package_hash()),
        },
    )
}

#[test]
fn test_create_voting() {
    // Create voting
    let (env, mut repo_voter_contract, variable_repo_contract, _reputation_token_contract) =
        create_voting();

    // check voting event
    repo_voter_contract.assert_event_at(
        1,
        VotingCreated {
            creator: env.get_account(0),
            voting_id: U256::from(0),
            stake: U256::from(500),
        },
    );

    let voting_created_event: VotingCreated = repo_voter_contract.event(1);
    let vote_cast_event: VoteCast = repo_voter_contract.event(2);

    // check if voting was created correctly
    let voting: Voting = repo_voter_contract.get_voting(vote_cast_event.voting_id);
    assert_eq!(voting.voting_id, vote_cast_event.voting_id);
    assert_eq!(voting.voting_id, U256::from(0));
    assert_eq!(voting.formal_voting_time, U256::from(432000000));
    assert_eq!(voting.informal_voting_time, U256::from(86400000));
    assert_eq!(voting.informal_voting_quorum, U256::from(2));
    assert_eq!(voting_created_event.voting_id, voting.voting_id);
    assert_eq!(voting_created_event.creator, env.get_account(0));
    assert_eq!(voting_created_event.stake, U256::from(500));

    // check if first vote was created correctly
    let vote: Vote = repo_voter_contract.get_vote(voting.voting_id, env.get_account(0));
    assert_eq!(vote.voting_id, voting.voting_id);
    assert_eq!(vote.voter, Some(env.get_account(0)));
    assert_eq!(vote.choice, true);
    assert_eq!(vote.stake, U256::from(500));

    // check if first vote was created by a caller
    let voters = repo_voter_contract.get_voters(voting.voting_id);
    assert_eq!(voters.len(), 1);
    assert_eq!(voters.get(0).unwrap().unwrap(), env.get_account(0));

    // check the voting counter after creating next voting
    repo_voter_contract
        .create_voting(
            variable_repo_contract.address(),
            "variable_name".into(),
            into_bytes("new_value"),
            Some(123),
            U256::from(321),
        )
        .unwrap();
    let vote_cast_event: VoteCast = repo_voter_contract.event(4);
    let voting: Voting = repo_voter_contract.get_voting(vote_cast_event.voting_id);
    assert_eq!(voting.voting_id, U256::from(1));
}

#[test]
fn test_informal_before_end() {
    // create voting
    let (_env, mut repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_voting();

    // We cannot finish voting which time didn't elapse
    let voting_id = VotingId::from(0);
    let result = repo_voter_contract.finish_voting(voting_id);

    assert_eq!(result.unwrap_err(), Error::InformalVotingTimeNotReached);
}

#[test]
fn test_informal_vote_without_a_quorum() {
    // create voting
    let (env, mut repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_voting();

    let voting_id = U256::from(0);
    let voting: Voting = repo_voter_contract.get_voting(U256::from(0));

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(
        voting.informal_voting_time.as_u64() - env.get_current_block_time() + 100,
    ));

    // Now the time should be fine, but a single vote should not reach quorum
    repo_voter_contract.finish_voting(voting_id).unwrap();
    repo_voter_contract.assert_event_at(
        3,
        InformalVotingEnded {
            result: "quorum_not_reached".into(),
            votes_count: U256::from(1),
            stake_in_favor: U256::from(500),
            stake_against: U256::from(0),
            informal_voting_id: VotingId::from(0),
            formal_voting_id: None,
        },
    );

    // voting status should be completed
    let voting: Voting = repo_voter_contract.get_voting(U256::from(0));
    assert_eq!(voting.completed, true);

    // cast a vote on a finished voting should return an error
    let result = repo_voter_contract.vote(U256::from(0), false, U256::from(500));
    assert_eq!(result.unwrap_err(), Error::VoteOnCompletedVotingNotAllowed);

    // the same goes for finishing voting
    let result = repo_voter_contract.finish_voting(voting_id);
    assert_eq!(
        result.unwrap_err(),
        Error::FinishingCompletedVotingNotAllowed
    );
}

#[test]
fn test_informal_voting_rejected() {
    // create voting
    let (env, mut repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_voting();
    let voting_id = VotingId::from(0);
    let voting: Voting = repo_voter_contract.get_voting(voting_id);

    // cast votes against
    repo_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, false, U256::from(500))
        .unwrap();
    repo_voter_contract
        .as_account(env.get_account(2))
        .vote(voting_id, false, U256::from(500))
        .unwrap();

    // fast-forward
    env.advance_block_time_by(Duration::from_secs(
        voting.informal_voting_time.as_u64() - env.get_current_block_time() + 100,
    ));

    // finish voting
    repo_voter_contract.finish_voting(voting_id).unwrap();

    // voting status should be completed
    let voting: Voting = repo_voter_contract.get_voting(voting_id);
    assert_eq!(voting.completed, true);

    // the status should be rejected
    repo_voter_contract.assert_event_at(
        5,
        InformalVotingEnded {
            result: "rejected".into(),
            votes_count: U256::from(3),
            stake_in_favor: U256::from(500),
            stake_against: U256::from(1000),
            informal_voting_id: VotingId::from(0),
            formal_voting_id: None,
        },
    );
}

#[test]
fn test_informal_voting_converted() {
    // create voting
    let (env, repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(0);

    // voting status should be completed
    let voting: Voting = repo_voter_contract.get_voting(voting_id);
    assert_eq!(voting.completed, true);

    // the status should be converted
    repo_voter_contract.assert_event_at(
        5,
        InformalVotingEnded {
            result: "converted_to_formal".into(),
            votes_count: U256::from(3),
            stake_in_favor: U256::from(1000),
            stake_against: U256::from(500),
            informal_voting_id: VotingId::from(0),
            formal_voting_id: Some(VotingId::from(1)),
        },
    );

    // new voting should be created
    repo_voter_contract.assert_event_at(
        6,
        VotingCreated {
            creator: env.get_account(0),
            voting_id: VotingId::from(1),
            stake: 500.into(),
        },
    );

    // with initial vote
    repo_voter_contract.assert_event_at(
        7,
        VoteCast {
            voter: env.get_account(0),
            voting_id: 1.into(),
            choice: true,
            stake: 500.into(),
        },
    );
}

#[test]
fn test_formal_voting_before_end() {
    let (_env, mut repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);

    let result = repo_voter_contract.finish_voting(voting_id);
    assert_eq!(result.unwrap_err(), Error::FormalVotingTimeNotReached);
}

#[test]
fn test_formal_vote_without_a_quorum() {
    let (env, mut repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = repo_voter_contract.get_voting(voting_id);

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(
        voting.formal_voting_time.as_u64() + 100,
    ));

    // Now the time should be fine, but a single vote should not reach quorum
    repo_voter_contract.finish_voting(voting_id).unwrap();
    repo_voter_contract.assert_event_at(
        8,
        FormalVotingEnded {
            result: "quorum_not_reached".into(),
            votes_count: U256::from(1),
            stake_in_favor: U256::from(500),
            stake_against: U256::from(0),
            informal_voting_id: VotingId::from(0),
            formal_voting_id: Some(voting_id),
        },
    );

    // voting status should be completed
    let voting: Voting = repo_voter_contract.get_voting(voting_id);
    assert_eq!(voting.completed, true);

    // cast a vote on a finished voting should return an error
    let result = repo_voter_contract.vote(voting_id, false, U256::from(500));
    assert_eq!(result.unwrap_err(), Error::VoteOnCompletedVotingNotAllowed);

    // the same goes for finishing voting
    let result = repo_voter_contract.finish_voting(voting_id);
    assert_eq!(
        result.unwrap_err(),
        Error::FinishingCompletedVotingNotAllowed
    );
}

#[test]
fn test_formal_vote_rejected() {
    let (env, mut repo_voter_contract, _variable_repo_contract, _reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = repo_voter_contract.get_voting(voting_id);

    // vote to reach quorum
    repo_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, false, 1000.into())
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(
        voting.formal_voting_time.as_u64() + 100,
    ));

    // Now the time should be fine, the result should be rejected
    repo_voter_contract.finish_voting(voting_id).unwrap();
    repo_voter_contract.assert_event_at(
        9,
        FormalVotingEnded {
            result: "rejected".into(),
            votes_count: U256::from(2),
            stake_in_favor: U256::from(500),
            stake_against: U256::from(1000),
            informal_voting_id: VotingId::from(0),
            formal_voting_id: Some(voting_id),
        },
    );

    // voting status should be completed
    let voting: Voting = repo_voter_contract.get_voting(voting_id);
    assert_eq!(voting.completed, true);
}

#[test]
fn test_formal_vote_completed() {
    let (env, mut repo_voter_contract, variable_repo_contract, _reputation_token_contract) =
        create_formal_voting();
    let voting_id = VotingId::from(1);
    let voting: Voting = repo_voter_contract.get_voting(voting_id);

    // vote to reach quorum
    repo_voter_contract
        .as_account(env.get_account(1))
        .vote(voting_id, true, 1000.into())
        .unwrap();

    // advance time, so voting can be finished
    env.advance_block_time_by(Duration::from_secs(
        voting.formal_voting_time.as_u64() + 100,
    ));

    // Now the time should be fine, the result should be completed
    repo_voter_contract.finish_voting(voting_id).unwrap();
    repo_voter_contract.assert_last_event(FormalVotingEnded {
        result: "passed".into(),
        votes_count: U256::from(2),
        stake_in_favor: U256::from(1500),
        stake_against: U256::from(0),
        informal_voting_id: VotingId::from(0),
        formal_voting_id: Some(voting_id),
    });

    // voting status should be completed
    let voting: Voting = repo_voter_contract.get_voting(voting_id);
    assert_eq!(voting.completed, true);

    // the action should be performed
    let bytes = variable_repo_contract.get("variable_name".into()).unwrap();
    dbg!(bytes.clone());
    let (variable, bytes) = String::from_bytes(&bytes).unwrap();
    assert_eq!(bytes.len(), 0);
    assert_eq!(variable, "new_value");
}

fn into_bytes(val: &str) -> Bytes {
    val.as_bytes().into()
}

fn setup() -> (
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

    variable_repo_contract
        .update_at(
            consts::INFORMAL_VOTING_QUORUM.into(),
            U256::from(2).to_bytes().unwrap().into(),
            None,
        )
        .unwrap();
    variable_repo_contract
        .update_at(
            consts::FORMAL_VOTING_QUORUM.into(),
            U256::from(2).to_bytes().unwrap().into(),
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

fn create_voting() -> (
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

fn create_formal_voting() -> (
    TestEnv,
    RepoVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let (env, mut repo_voter_contract, variable_repo_contract, reputation_token_contract) =
        create_voting();
    let voting_id = VotingId::from(0);
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
    env.advance_block_time_by(Duration::from_secs(
        voting.informal_voting_time.as_u64() - env.get_current_block_time() + 100,
    ));

    // finish voting
    repo_voter_contract.finish_voting(voting_id).unwrap();

    (
        env,
        repo_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}
