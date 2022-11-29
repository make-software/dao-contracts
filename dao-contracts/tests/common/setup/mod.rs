use casper_dao_contracts::{
    // action::Action,
    simple_voter::SimpleVoterContractTest,
    voting::{types::VotingId, voting::Voting, Choice},
    // AdminContractTest,
    BidEscrowContractTest,
    KycNftContractTest,
    MockVoterContractTest,
    // RepoVoterContractTest,
    ReputationContractTest,
    ReputationVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_erc721::TokenId;
use casper_dao_utils::{consts, Error, TestContract, TestEnv};
use casper_types::{bytesrepr::ToBytes, U256};

#[allow(dead_code)]
pub fn setup_bid_escrow_gherkin() -> (
    BidEscrowContractTest,
    ReputationContractTest,
    VaNftContractTest,
    KycNftContractTest,
) {
    let informal_quorum = 500.into();
    let formal_quorum = 500.into();
    let total_onboarded = 6;

    let (variable_repo_contract, mut reputation_token_contract, _va_owned_nft_contract) =
        setup_repository_and_reputation_contracts_gherkin(
            informal_quorum,
            formal_quorum,
            total_onboarded,
        );

    let va_token = VaNftContractTest::new(
        variable_repo_contract.get_env(),
        "user token".to_string(),
        "usert".to_string(),
        "".to_string(),
    );

    let kyc_token = KycNftContractTest::new(
        variable_repo_contract.get_env(),
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );

    let bid_escrow_contract = BidEscrowContractTest::new(
        variable_repo_contract.get_env(),
        variable_repo_contract.address(),
        reputation_token_contract.address(),
        kyc_token.address(),
        va_token.address(),
    );

    reputation_token_contract
        .add_to_whitelist(bid_escrow_contract.address())
        .unwrap();

    (
        bid_escrow_contract,
        reputation_token_contract,
        va_token,
        kyc_token,
    )
}

fn setup_repository_and_reputation_contracts_gherkin(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (
    VariableRepositoryContractTest,
    ReputationContractTest,
    VaNftContractTest,
) {
    let reputation_to_mint = 0;
    let env = TestEnv::new();
    let variable_repo_contract = setup_variable_repo_contract(
        &env,
    );
    let reputation_token_contract =
        setup_reputation_token_contract(&env, reputation_to_mint, total_onboarded);
    let va_token = setup_va_token(&env, total_onboarded);
    (variable_repo_contract, reputation_token_contract, va_token)
}

#[allow(dead_code)]
pub fn setup_bid_escrow() -> (
    BidEscrowContractTest,
    ReputationContractTest,
    VaNftContractTest,
    KycNftContractTest,
) {
    let informal_quorum = 500.into();
    let formal_quorum = 500.into();
    let total_onboarded = 6;

    let (variable_repo_contract, mut reputation_token_contract, _va_owned_nft_contract) =
        setup_repository_and_reputation_contracts(informal_quorum, formal_quorum, total_onboarded);

    let va_token = VaNftContractTest::new(
        variable_repo_contract.get_env(),
        "user token".to_string(),
        "usert".to_string(),
        "".to_string(),
    );

    let kyc_token = KycNftContractTest::new(
        variable_repo_contract.get_env(),
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );

    let bid_escrow_contract = BidEscrowContractTest::new(
        variable_repo_contract.get_env(),
        variable_repo_contract.address(),
        reputation_token_contract.address(),
        kyc_token.address(),
        va_token.address(),
    );

    reputation_token_contract
        .add_to_whitelist(bid_escrow_contract.address())
        .unwrap();

    (
        bid_escrow_contract,
        reputation_token_contract,
        va_token,
        kyc_token,
    )
}

#[allow(dead_code)]
pub fn setup_reputation_voter() -> (ReputationVoterContractTest, ReputationContractTest) {
    let informal_quorum = 500.into();
    let formal_quorum = 500.into();
    let total_onboarded = 3;

    #[allow(unused_mut)]
    let (mut variable_repo_contract, mut reputation_token_contract, va_token) =
        setup_repository_and_reputation_contracts(informal_quorum, formal_quorum, total_onboarded);

    #[allow(unused_mut)]
    let mut reputation_voter_contract = ReputationVoterContractTest::new(
        variable_repo_contract.get_env(),
        variable_repo_contract.address(),
        reputation_token_contract.address(),
        va_token.address(),
    );

    reputation_token_contract
        .add_to_whitelist(reputation_voter_contract.address())
        .unwrap();

    reputation_token_contract
        .mint(
            reputation_token_contract.get_env().get_account(0),
            1000.into(),
        )
        .unwrap();
    reputation_token_contract
        .mint(
            reputation_token_contract.get_env().get_account(1),
            1000.into(),
        )
        .unwrap();

    (reputation_voter_contract, reputation_token_contract)
}

#[allow(dead_code)]
pub fn setup_simple_voter() -> SimpleVoterContractTest {
    let informal_quorum = 500.into();
    let formal_quorum = 500.into();
    let total_onboarded = 3;

    let (mut variable_repo_contract, mut reputation_token_contract, va_token) =
        setup_repository_and_reputation_contracts(informal_quorum, formal_quorum, total_onboarded);

    #[allow(unused_mut)]
    let mut simple_voter_contract = SimpleVoterContractTest::new(
        variable_repo_contract.get_env(),
        variable_repo_contract.address(),
        reputation_token_contract.address(),
        va_token.address(),
    );

    variable_repo_contract
        .add_to_whitelist(simple_voter_contract.address())
        .unwrap();

    reputation_token_contract
        .add_to_whitelist(simple_voter_contract.address())
        .unwrap();

    simple_voter_contract
}

// #[allow(dead_code)]
// pub fn setup_admin() -> (AdminContractTest, ReputationContractTest) {
//     let minimum_reputation = 500.into();
//     let informal_quorum = 500.into();
//     let formal_quorum = 500.into();
//     let total_onboarded = 3;

//     let (variable_repo_contract, mut reputation_token_contract, va_token) =
//         setup_repository_and_reputation_contracts(informal_quorum, formal_quorum, total_onboarded);

//     #[allow(unused_mut)]
//     let mut admin_contract = AdminContractTest::new(
//         variable_repo_contract.get_env(),
//         variable_repo_contract.address(),
//         reputation_token_contract.address(),
//         va_token.address(),
//     );

//     reputation_token_contract
//         .change_ownership(admin_contract.address())
//         .unwrap();

//     admin_contract
//         .create_voting(
//             reputation_token_contract.address(),
//             Action::AddToWhitelist,
//             admin_contract.get_env().get_account(1),
//             minimum_reputation,
//         )
//         .unwrap();

//     let voting_id = 0;
//     let voting: Voting = admin_contract.get_voting(voting_id).unwrap();

//     admin_contract
//         .as_nth_account(1)
//         .vote(voting_id, Choice::InFavor, minimum_reputation)
//         .unwrap();
//     admin_contract.advance_block_time_by(voting.informal_voting_time() + 1);
//     admin_contract.finish_voting(voting_id).unwrap();

//     (admin_contract, reputation_token_contract)
// }

// #[allow(dead_code)]
// pub fn setup_repo_voter(
//     key: String,
//     value: Bytes,
// ) -> (RepoVoterContractTest, VariableRepositoryContractTest) {
//     let minimum_reputation = 500.into();
//     let informal_quorum = 500.into();
//     let formal_quorum = 500.into();
//     let total_onboarded = 3;

//     let (mut variable_repo_contract, mut reputation_token_contract, va_token) =
//         setup_repository_and_reputation_contracts(informal_quorum, formal_quorum, total_onboarded);

//     #[allow(unused_mut)]
//     let mut repo_voter_contract = RepoVoterContractTest::new(
//         variable_repo_contract.get_env(),
//         variable_repo_contract.address(),
//         reputation_token_contract.address(),
//         va_token.address(),
//     );

//     variable_repo_contract
//         .add_to_whitelist(repo_voter_contract.address())
//         .unwrap();

//     reputation_token_contract
//         .add_to_whitelist(repo_voter_contract.address())
//         .unwrap();

//     repo_voter_contract
//         .create_voting(
//             repo_voter_contract.get_variable_repo_address(),
//             key,
//             value,
//             None,
//             minimum_reputation,
//         )
//         .unwrap();

//     let voting_id = 0;
//     let voting: Voting = repo_voter_contract.get_voting(voting_id).unwrap();

//     repo_voter_contract
//         .as_nth_account(1)
//         .vote(voting_id, Choice::InFavor, minimum_reputation)
//         .unwrap();
//     repo_voter_contract.advance_block_time_by(voting.informal_voting_time() + 1);
//     repo_voter_contract.finish_voting(voting_id).unwrap();

//     (repo_voter_contract, variable_repo_contract)
// }

pub fn setup_voting_contract(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (
    MockVoterContractTest,
    VariableRepositoryContractTest,
    ReputationContractTest,
) {
    let (mut variable_repo_contract, mut reputation_token_contract, va_token) =
        setup_repository_and_reputation_contracts(informal_quorum, formal_quorum, total_onboarded);

    #[allow(unused_mut)]
    let mut mock_voter_contract = MockVoterContractTest::new(
        variable_repo_contract.get_env(),
        variable_repo_contract.address(),
        reputation_token_contract.address(),
        va_token.address(),
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

fn setup_repository_and_reputation_contracts(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (
    VariableRepositoryContractTest,
    ReputationContractTest,
    VaNftContractTest,
) {
    let reputation_to_mint = 10_000;
    let informal_voting_time: u64 = 3_600;
    let env = TestEnv::new();
    let variable_repo_contract = setup_variable_repo_contract(
        &env,
    );
    let reputation_token_contract =
        setup_reputation_token_contract(&env, reputation_to_mint, total_onboarded);
    let va_token = setup_va_token(&env, total_onboarded);
    (variable_repo_contract, reputation_token_contract, va_token)
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

    let voting_id = 0;
    let voting = mock_voter_contract.get_voting(voting_id).unwrap();
    (mock_voter_contract, reputation_token_contract, voting)
}

#[allow(dead_code)]
pub fn setup_voting_contract_with_formal_voting(
    informal_quorum: U256,
    formal_quorum: U256,
    total_onboarded: usize,
) -> (MockVoterContractTest, ReputationContractTest, Voting) {
    let minimum_reputation = 500.into();
    let (mut mock_voter_contract, reputation_token_contract, voting) =
        setup_voting_contract_with_informal_voting(informal_quorum, formal_quorum, total_onboarded);

    for account in 1..total_onboarded {
        mock_voter_contract
            .as_nth_account(account)
            .vote(voting.voting_id(), Choice::InFavor, minimum_reputation)
            .unwrap();
    }

    mock_voter_contract
        .advance_block_time_by(voting.informal_voting_time() + 1)
        .finish_voting(voting.voting_id())
        .unwrap();

    let voting_id = 1;
    let formal_voting = mock_voter_contract.get_voting(voting_id).unwrap();

    (
        mock_voter_contract,
        reputation_token_contract,
        formal_voting,
    )
}

pub fn setup_variable_repo_contract(
    env: &TestEnv,
) -> VariableRepositoryContractTest {
    let mut variable_repo_contract = VariableRepositoryContractTest::new(env);
    variable_repo_contract
}

pub fn setup_reputation_token_contract(
    env: &TestEnv,
    tokens: usize,
    total_onboarded: usize,
) -> ReputationContractTest {
    let mut reputation_token_contract = ReputationContractTest::new(env);

    for i in 0..total_onboarded {
        reputation_token_contract
            .mint(env.get_account(i), tokens.into())
            .unwrap();
    }

    reputation_token_contract
}

pub fn setup_va_token(env: &TestEnv, total_onboarded: usize) -> VaNftContractTest {
    let mut va_token = VaNftContractTest::new(
        env,
        "va_token".to_string(),
        "VAT".to_string(),
        "".to_string(),
    );
    for i in 0..total_onboarded {
        va_token.mint(env.get_account(i), TokenId::from(i)).unwrap();
    }
    va_token
}

#[allow(dead_code)]
pub fn mass_vote(
    votes_in_favor: usize,
    votes_against: usize,
    voting_contract: &mut MockVoterContractTest,
    voting: &Voting,
) {
    let minimum_reputation = 500.into();
    let mut account = 1;
    for _ in 1..votes_in_favor {
        // we skip one vote in favor - creator's vote
        voting_contract
            .as_nth_account(account)
            .vote(voting.voting_id(), Choice::InFavor, minimum_reputation)
            .unwrap();
        account += 1;
    }

    for _ in 0..votes_against {
        voting_contract
            .as_nth_account(account)
            .vote(voting.voting_id(), Choice::Against, minimum_reputation)
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
    let minimum_reputation = 500.into();

    // it is completed
    assert!(voting.completed());

    // it doesn't allow voting
    assert_eq!(
        voter_contract.as_nth_account(1).vote(
            voting.voting_id(),
            Choice::InFavor,
            minimum_reputation,
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
