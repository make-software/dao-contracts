mod governance_voting_common;
use casper_dao_contracts::voting::{consts, VotingEnded};
use casper_dao_utils::TestContract;
use casper_types::U256;
use test_case::test_case;

#[test_case(0, 0, 4, U256::from(500), consts::INFORMAL_VOTING_QUORUM_NOT_REACHED, &[9500, 0, 0]; "Nobody votes")]
#[test_case(1, 1, 4, U256::from(500), consts::INFORMAL_VOTING_PASSED, &[9500, 10000, 10000, 500, 0]; "Exact number of votes, tie")]
#[test_case(2, 0, 4, U256::from(500), consts::INFORMAL_VOTING_PASSED, &[9500, 10000, 500, 0]; "Exact number of votes in favor")]
#[test_case(0, 2, 4, U256::from(500), consts::INFORMAL_VOTING_REJECTED, &[9500, 10000, 10000, 0, 0]; "Exact number of votes againts")]
#[test_case(2, 2, 10, U256::from(500), consts::INFORMAL_VOTING_QUORUM_NOT_REACHED, &[9500, 10000, 10000, 10000, 0, 0]; "One vote less than quorum")]
#[test_case(2, 3, 10, U256::from(500), consts::INFORMAL_VOTING_REJECTED, &[9500, 10000, 10000, 10000, 10000, 0, 0]; "Exact number of votes - 10 onboarded")]
#[test_case(10, 0, 10, U256::from(500), consts::INFORMAL_VOTING_PASSED, &[9500, 10000, 500, 0]; "Everybody votes in favor - 10 onboarded")]
fn test_informal_voting_result(
    votes_in_favor: usize,
    votes_against: usize,
    total_onboarded: usize,
    quorum: U256,
    result: &str,
    reputation: &[usize],
) {
    let contract_balance = reputation[reputation.len() - 2];
    let dust = reputation[reputation.len() - 1];
    let (mut voting_contract, reputation_token_contract, voting) =
        governance_voting_common::setup_voting_contract_with_informal_voting(
            quorum,
            U256::zero(),
            total_onboarded,
        );
    governance_voting_common::mass_vote(
        votes_in_favor,
        votes_against,
        &mut voting_contract,
        &voting,
    );
    voting_contract
        .advance_block_time_by(voting.informal_voting_time().unwrap() + 1)
        .finish_voting(voting.voting_id())
        .unwrap();

    let event: VotingEnded = voting_contract.event(-1);
    assert_eq!(event.result, result);

    governance_voting_common::assert_reputation(
        &reputation_token_contract,
        &reputation[0..reputation.len() - 2],
    );
    assert_eq!(
        reputation_token_contract.balance_of(voting_contract.address()),
        U256::from(contract_balance)
    );
    assert_eq!(voting_contract.get_dust_amount(), U256::from(dust));
}

#[test_case(0, 0, 4, U256::from(750), consts::FORMAL_VOTING_QUORUM_NOT_REACHED, &[9500, 0, 0]; "Nobody votes")]
#[test_case(2, 1, 4, U256::from(750), consts::FORMAL_VOTING_PASSED, &[10250, 10250, 9500, 0, 0]; "Exact number of votes")]
#[test_case(2, 0, 4, U256::from(750), consts::FORMAL_VOTING_QUORUM_NOT_REACHED, &[9500, 10000, 0, 0]; "One vote less than quorum - 4 onboarded")]
#[test_case(3, 0, 4, U256::from(750), consts::FORMAL_VOTING_PASSED, &[10000, 10000, 10000, 0, 0]; "Exact number of votes in favor")]
#[test_case(0, 3, 4, U256::from(750), consts::FORMAL_VOTING_REJECTED, &[9500, 10166, 10166, 10166, 2, 2]; "Exact number of votes againts")]
#[test_case(6, 0, 10, U256::from(750), consts::FORMAL_VOTING_QUORUM_NOT_REACHED, &[9500, 10000, 10000, 0, 0]; "One vote less than quorum - 10 onboarded")]
#[test_case(2, 5, 10, U256::from(750), consts::FORMAL_VOTING_REJECTED, &[9500, 9500, 10200, 10200, 0, 0]; "Exact number of votes - 10 onboarded")]
#[test_case(10, 0, 10, U256::from(750), consts::FORMAL_VOTING_PASSED, &[10000, 10000, 10000, 0, 0]; "Everybody votes in favor - 10 onboarded")]
fn test_formal_voting_result(
    votes_in_favor: usize,
    votes_against: usize,
    total_onboarded: usize,
    quorum: U256,
    result: &str,
    reputation: &[usize],
) {
    let contract_balance = reputation[reputation.len() - 2];
    let dust = reputation[reputation.len() - 1];
    let (mut voting_contract, reputation_token_contract, voting) =
        governance_voting_common::setup_voting_contract_with_formal_voting(
            U256::zero(),
            quorum,
            total_onboarded,
        );
    governance_voting_common::mass_vote(
        votes_in_favor,
        votes_against,
        &mut voting_contract,
        &voting,
    );
    voting_contract
        .advance_block_time_by(voting.formal_voting_time() + 1)
        .finish_voting(voting.voting_id())
        .unwrap();

    let event: VotingEnded = voting_contract.event(-1);
    assert_eq!(event.result, result);

    governance_voting_common::assert_reputation(
        &reputation_token_contract,
        &reputation[0..reputation.len() - 2],
    );
    assert_eq!(
        reputation_token_contract.balance_of(voting_contract.address()),
        U256::from(contract_balance)
    );
    assert_eq!(voting_contract.get_dust_amount(), U256::from(dust));
}
