use casper_dao_contracts::{
    voting::{voting::Voting, Choice, VotingId},
    MockVoterContractTest, ReputationContractTest, VariableRepositoryContractTest,
};

use casper_dao_utils::{consts, Error, TestEnv};
use casper_types::{bytesrepr::ToBytes, U256};

pub fn setup_voting_contract(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (
    MockVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let minimum_reputation = 500.into();
    let reputation_to_mint = 10_000;
    let informal_voting_time: u64 = 3_600;
    let formal_voting_time: u64 = 2 * informal_voting_time;

    let env = TestEnv::new();
    let mut variable_repo_contract = setup_variable_repo_contract(
        &env,
        informal_quorum,
        formal_quorum,
        informal_voting_time,
        formal_voting_time,
        minimum_reputation,
    );
    let mut reputation_token_contract =
        setup_reputation_token_contract(&env, reputation_to_mint, total_onboarded);

    #[allow(unused_mut)]
    let mut mock_voter_contract = MockVoterContractTest::new(
        &env,
        variable_repo_contract.address(),
        reputation_token_contract.address(),
    );

    variable_repo_contract
        .add_to_whitelist(mock_voter_contract.address())
        .unwrap();

    reputation_token_contract
        .add_to_whitelist(mock_voter_contract.address())
        .unwrap();

    (
        mock_voter_contract,
        variable_repo_contract,
        reputation_token_contract,
    )
}

pub fn setup_voting_contract_with_informal_voting(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (MockVoterContractTest, ReputationContractTest, Voting) {
    let (mut mock_voter_contract, _variable_repository_contract, reputation_token_contract) =
        setup_voting_contract(informal_quorum, formal_quorum, total_onboarded);

    mock_voter_contract
        .create_voting("value".to_string(), U256::from(500))
        .unwrap();

    let voting = mock_voter_contract.get_voting(U256::zero()).unwrap();
    (mock_voter_contract, reputation_token_contract, voting)
}

#[allow(dead_code)]
pub fn setup_voting_contract_with_formal_voting(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (MockVoterContractTest, ReputationContractTest, Voting) {
    let (mut mock_voter_contract, reputation_token_contract, voting) =
        setup_voting_contract_with_informal_voting(informal_quorum, formal_quorum, total_onboarded);

    for account in 1..total_onboarded {
        mock_voter_contract
            .as_nth_account(account)
            .vote(
                voting.voting_id(),
                Choice::InFavor,
                voting.minimum_governance_reputation(),
            )
            .unwrap();
    }

    mock_voter_contract
        .advance_block_time_by(voting.informal_voting_time() + 1)
        .finish_voting(voting.voting_id())
        .unwrap();

    let formal_voting = mock_voter_contract.get_voting(VotingId::from(1)).unwrap();

    (
        mock_voter_contract,
        reputation_token_contract,
        formal_voting,
    )
}

pub fn setup_variable_repo_contract(
    env: &TestEnv,
    informal_voting_quorum: U256,
    formal_voting_quorum: U256,
    informal_voting_time: u64,
    formal_voting_time: u64,
    minimum_reputation: U256,
) -> VariableRepositoryContractTest {
    let mut variable_repo_contract = VariableRepositoryContractTest::new(env);

    update(
        &mut variable_repo_contract,
        consts::INFORMAL_VOTING_QUORUM,
        informal_voting_quorum,
    );
    update(
        &mut variable_repo_contract,
        consts::FORMAL_VOTING_QUORUM,
        formal_voting_quorum,
    );
    update(
        &mut variable_repo_contract,
        consts::MINIMUM_GOVERNANCE_REPUTATION,
        minimum_reputation,
    );
    update(
        &mut variable_repo_contract,
        consts::FORMAL_VOTING_TIME,
        formal_voting_time,
    );
    update(
        &mut variable_repo_contract,
        consts::INFORMAL_VOTING_TIME,
        informal_voting_time,
    );

    variable_repo_contract
}

pub fn setup_reputation_token_contract(
    env: &TestEnv,
    tokens: usize,
    total_onboarded: usize,
) -> ReputationContractTest {
    let mut reputation_token_contract = ReputationContractTest::new(env);

    reputation_token_contract
        .set_total_onboarded(U256::from(total_onboarded))
        .unwrap();

    for i in 0..total_onboarded {
        reputation_token_contract
            .mint(env.get_account(i), tokens.into())
            .unwrap();
    }

    reputation_token_contract
}

#[allow(dead_code)]
pub fn mass_vote(
    votes_in_favor: usize,
    votes_against: usize,
    voting_contract: &mut MockVoterContractTest,
    voting: &Voting,
) {
    let mut account = 1;
    for _ in 1..votes_in_favor {
        // we skip one vote in favor - creator's vote
        voting_contract
            .as_nth_account(account)
            .vote(
                voting.voting_id(),
                Choice::InFavor,
                voting.minimum_governance_reputation(),
            )
            .unwrap();
        account += 1;
    }

    for _ in 0..votes_against {
        voting_contract
            .as_nth_account(account)
            .vote(
                voting.voting_id(),
                Choice::Against,
                voting.minimum_governance_reputation(),
            )
            .unwrap();
        account += 1;
    }
}

#[allow(dead_code)]
pub fn assert_reputation(reputation_contract: &ReputationContractTest, reputation: &[usize]) {
    for (account, amount) in reputation.iter().enumerate() {
        let address = reputation_contract.get_env().get_account(account);
        assert_eq!(reputation_contract.balance_of(address), U256::from(*amount));
    }
}

#[allow(dead_code)]
pub fn assert_voting_completed(voter_contract: &mut MockVoterContractTest, voting_id: VotingId) {
    let voting = voter_contract.get_voting(voting_id).unwrap();

    // it is completed
    assert!(voting.completed());

    // it doesn't allow voting
    assert_eq!(
        voter_contract.as_nth_account(1).vote(
            voting.voting_id(),
            casper_dao_contracts::voting::Choice::InFavor,
            voting.minimum_governance_reputation()
        ),
        Err(Error::VoteOnCompletedVotingNotAllowed)
    );

    // it cannot be finished again
    assert_eq!(
        voter_contract.finish_voting(voting.voting_id()),
        Err(Error::FinishingCompletedVotingNotAllowed)
    );
}

fn update<T: ToBytes>(contract: &mut VariableRepositoryContractTest, name: &str, value: T) {
    contract
        .update_at(name.into(), value.to_bytes().unwrap().into(), None)
        .unwrap();
}
