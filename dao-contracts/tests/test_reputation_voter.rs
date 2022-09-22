use casper_dao_contracts::reputation_voter::{Action, ReputationVotingCreated};
use casper_dao_contracts::voting::{Choice, VotingCreated};
use casper_dao_contracts::ReputationVoterContractTest;
use casper_dao_utils::{Address, TestContract};
use casper_types::U256;

mod governance_voting_common;

#[test]
fn test_minting_and_burning() {
    let amount = 500.into();
    let (mut reputation_voter_contract, reputation_token_contract) =
        governance_voting_common::setup_reputation_voter();
    let address = reputation_voter_contract.get_env().get_account(5);

    assert_eq!(reputation_token_contract.balance_of(address), U256::zero());
    assert_eq!(reputation_token_contract.debt(address), U256::zero());

    vote_action(
        &mut reputation_voter_contract,
        Action::Mint,
        address,
        amount,
    );

    assert_eq!(reputation_token_contract.balance_of(address), amount);
    assert_eq!(reputation_token_contract.debt(address), U256::zero());

    vote_action(
        &mut reputation_voter_contract,
        Action::Mint,
        address,
        amount,
    );

    assert_eq!(reputation_token_contract.balance_of(address), amount * 2);
    assert_eq!(reputation_token_contract.debt(address), U256::zero());

    vote_action(
        &mut reputation_voter_contract,
        Action::Burn,
        address,
        amount,
    );

    assert_eq!(reputation_token_contract.balance_of(address), amount);
    assert_eq!(reputation_token_contract.debt(address), U256::zero());

    vote_action(
        &mut reputation_voter_contract,
        Action::Burn,
        address,
        amount,
    );

    assert_eq!(reputation_token_contract.balance_of(address), U256::zero());
    assert_eq!(reputation_token_contract.debt(address), U256::zero());

    vote_action(
        &mut reputation_voter_contract,
        Action::Burn,
        address,
        amount,
    );

    assert_eq!(reputation_token_contract.balance_of(address), U256::zero());
    assert_eq!(reputation_token_contract.debt(address), amount);

    vote_action(
        &mut reputation_voter_contract,
        Action::Burn,
        address,
        amount,
    );

    assert_eq!(reputation_token_contract.balance_of(address), U256::zero());
    assert_eq!(reputation_token_contract.debt(address), amount * 2);

    vote_action(
        &mut reputation_voter_contract,
        Action::Mint,
        address,
        amount * 3,
    );

    assert_eq!(reputation_token_contract.balance_of(address), amount);
    assert_eq!(reputation_token_contract.debt(address), U256::zero());
}

#[test]
fn test_document_hash() {
    let amount = 500.into();
    let (mut reputation_voter_contract, _reputation_token_contract) =
        governance_voting_common::setup_reputation_voter();
    let address = reputation_voter_contract.get_env().get_account(5);
    let document_hash = 123.into();
    reputation_voter_contract
        .create_voting(address, Action::Mint, amount, document_hash, 500.into())
        .unwrap();

    let reputation_voting_created: ReputationVotingCreated = reputation_voter_contract.event(-1);
    assert_eq!(
        reputation_voting_created.reputation_voting.document_hash,
        document_hash
    );
}

fn vote_action(
    reputation_voter_contract: &mut ReputationVoterContractTest,
    action: Action,
    address: Address,
    amount: U256,
) {
    reputation_voter_contract
        .create_voting(address, action, amount, 123.into(), 500.into())
        .unwrap();

    let voting_created_event: VotingCreated = reputation_voter_contract.event(-3);
    let voting = reputation_voter_contract
        .get_voting(voting_created_event.voting_id)
        .unwrap();

    reputation_voter_contract
        .as_nth_account(1)
        .vote(voting.voting_id(), Choice::InFavor, 500.into())
        .unwrap();
    reputation_voter_contract.advance_block_time_by(voting.informal_voting_time());
    reputation_voter_contract
        .finish_voting(voting.voting_id())
        .unwrap();

    let voting_created_event: VotingCreated = reputation_voter_contract.event(-3);
    let voting = reputation_voter_contract
        .get_voting(voting_created_event.voting_id)
        .unwrap();

    reputation_voter_contract
        .as_nth_account(1)
        .vote(
            voting.formal_voting_id().unwrap(),
            Choice::InFavor,
            500.into(),
        )
        .unwrap();
    reputation_voter_contract.advance_block_time_by(voting.formal_voting_time());
    reputation_voter_contract
        .finish_voting(voting.formal_voting_id().unwrap())
        .unwrap();
}
