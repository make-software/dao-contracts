use casper_dao_contracts::{VariableRepositoryContractTest, ReputationContractTest, RepoVoterContractTest};
use casper_dao_modules::{VotingId, events::{VotingContractCreated, VotingCreated, VoteCast, InformalVotingEnded}, voting::Voting, vote::Vote};
use casper_dao_utils::{Error, TestEnv, Address};
use casper_types::{bytesrepr::{ToBytes, FromBytes}, U256, RuntimeArgs};

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

    repo_voter_contract.assert_event_at(0, VotingContractCreated {
        variable_repo: Address::from(variable_repo_contract.get_package_hash()),
        reputation_token: Address::from(reputation_token_contract.get_package_hash()),
        repo_voter: Address::from(repo_voter_contract.get_package_hash())
    })
}

#[test]
fn test_create_voting() {
    // Create voting
    let into_bytes = |val: &str| val.as_bytes().into();
    let (env, mut repo_voter_contract, variable_repo_contract, _reputation_token_contract) = setup();
    repo_voter_contract.create_voting(Address::from(variable_repo_contract.get_package_hash()), "variable_name".into(), into_bytes("new_value"), Some(123), U256::from(123)).unwrap();
    

    // check event
    repo_voter_contract.assert_event_at(2, VotingCreated {
        creator: env.get_account(0),
        voting_id: U256::from(0),
        stake: U256::from(123),
    });

    let vote_cast_event: VoteCast = repo_voter_contract.event(1);
    let voting_created_event: VotingCreated = repo_voter_contract.event(2);
    
    // check if voting was created correctly
    let voting: Voting = repo_voter_contract.get_voting(vote_cast_event.voting_id);
    assert_eq!(voting.voting_id, vote_cast_event.voting_id);
    assert_eq!(voting.voting_id, U256::from(0));
    assert_eq!(voting.informal_voting_quorum, U256::from(2));
    assert_eq!(voting_created_event.voting_id, voting.voting_id);
    assert_eq!(voting_created_event.creator, env.get_account(0));
    assert_eq!(voting_created_event.stake, U256::from(123));

    // check if first vote was created correctly
    let vote: Vote = repo_voter_contract.get_vote(voting.voting_id, env.get_account(0));
    assert_eq!(vote.voting_id, voting.voting_id);
    assert_eq!(vote.voter, env.get_account(0));
    assert_eq!(vote.choice, true);
    assert_eq!(vote.stake, U256::from(123));
    
    // check if first vote was created by a caller
    let voters = repo_voter_contract.get_voters(voting.voting_id);
    assert_eq!(voters.len(), 1);
    assert_eq!(voters.get(0).unwrap().unwrap(), env.get_account(0));
    
    // check the voting counter
    repo_voter_contract.create_voting(variable_repo_contract.address(), "variable_name".into(), into_bytes("new_value"), Some(123), U256::from(321)).unwrap();
    let vote_cast_event: VoteCast = repo_voter_contract.event(3);
    let voting: Voting = repo_voter_contract.get_voting(vote_cast_event.voting_id);
    assert_eq!(voting.voting_id, U256::from(1));
    assert_eq!(vote_cast_event.stake, U256::from(321));
}

#[test]
fn test_vote() {
    // create voting
    let into_bytes = |val: &str| val.as_bytes().into();
    let (env, mut repo_voter_contract, variable_repo_contract, _reputation_token_contract) = setup();    
    let first_voting_id = U256::from(0);
    repo_voter_contract.create_voting(variable_repo_contract.address(), "variable_name".into(), into_bytes("new_value"), Some(123), U256::from(500)).unwrap();
    
    // Single vote should not reach quorum
    let result = repo_voter_contract.finish_voting(first_voting_id);
    assert_eq!(result.unwrap_err(), Error::InformalQuorumNotReached);
    
    // cast a vote as somebody else, now the quorum should be fine and informal voting should end
    repo_voter_contract.as_account(env.get_account(1)).vote(U256::from(0), false, U256::from(500)).unwrap();
    repo_voter_contract.finish_voting(first_voting_id).unwrap();

    repo_voter_contract.assert_event_at(0, InformalVotingEnded {
        result: "converted_to_formal".into(),
        votes_count: U256::from(2),
        stake_in_favor: U256::from(500),
        stake_against: U256::from(500),
        informal_voting_id: U256::from(0),
        formal_voting_id: Some(U256::from(1)),
    });
}

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
        entry_point: "update_variable".into(),
        runtime_args: RuntimeArgs::new(),
        formal_voting_quorum: U256::from(2),
        formal_voting_time: U256::from(2),
        informal_voting_time: U256::from(2),
        minimum_governance_reputation: U256::from(2),
    };

    let (voting2, _bytes) = Voting::from_bytes(&voting.to_bytes().unwrap()).unwrap();

    assert_eq!(voting.voting_id, voting2.voting_id);
    assert_eq!(voting.informal_voting_id, voting2.informal_voting_id);
    assert_eq!(voting.formal_voting_id, voting2.formal_voting_id);
    assert_eq!(voting.informal_voting_quorum, voting2.informal_voting_quorum);
    assert_eq!(voting.formal_voting_quorum, voting2.formal_voting_quorum);
    assert_eq!(voting.stake_against, voting2.stake_against);
    assert_eq!(voting.stake_in_favor, voting2.stake_in_favor);
    assert_eq!(voting.completed, voting2.completed);
    assert_eq!(voting.entry_point, voting2.entry_point);
    assert_eq!(voting.runtime_args, voting2.runtime_args);
    assert_eq!(voting.formal_voting_time, voting2.formal_voting_time);
    assert_eq!(voting.informal_voting_time, voting2.informal_voting_time);
    assert_eq!(voting.minimum_governance_reputation, voting2.minimum_governance_reputation);
}

#[test]
fn test_vote_serialization() {
    let env = TestEnv::new();
    let vote = Vote {
        voter: env.get_account(0),
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

fn setup() -> (TestEnv, RepoVoterContractTest, VariableRepositoryContractTest, ReputationContractTest) {
    let env = TestEnv::new();
    let variable_repo_contract = VariableRepositoryContractTest::new(&env);
    let reputation_token_contract = ReputationContractTest::new(&env);
    let repo_voter_contract = RepoVoterContractTest::new(&env, Address::from(variable_repo_contract.get_package_hash()), Address::from(reputation_token_contract.get_package_hash()));

    (env, repo_voter_contract, variable_repo_contract, reputation_token_contract)
}