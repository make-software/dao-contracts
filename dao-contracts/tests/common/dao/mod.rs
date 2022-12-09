use casper_dao_contracts::{
    BidEscrowContractTest,
    KycNftContractTest,
    KycVoterContractTest,
    ReputationContractTest,
    SlashingVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{TestContract, TestEnv};

mod account;
mod bid_escrow;
mod cspr;
mod events;
mod kyc;
mod ownership;
mod reputation;
mod va;
mod voting;

#[allow(dead_code)]
pub fn setup_dao() -> (
    TestEnv,
    BidEscrowContractTest,
    ReputationContractTest,
    VaNftContractTest,
    KycNftContractTest,
    VariableRepositoryContractTest,
    SlashingVoterContractTest,
    KycVoterContractTest,
) {
    let env = TestEnv::new();
    let variable_repo = VariableRepositoryContractTest::new(&env);
    let mut reputation_token = ReputationContractTest::new(&env);

    let mut va_token = VaNftContractTest::new(
        &env,
        "va_token".to_string(),
        "VAT".to_string(),
        "".to_string(),
    );

    let mut kyc_token = KycNftContractTest::new(
        variable_repo.get_env(),
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );

    let mut bid_escrow = BidEscrowContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        kyc_token.address(),
        va_token.address(),
    );

    let mut slashing_voter = SlashingVoterContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        va_token.address(),
    );

    let kyc_voter = KycVoterContractTest::new(
        &env,
        variable_repo.address(),
        reputation_token.address(),
        va_token.address(),
        kyc_token.address(),
    );

    reputation_token
        .add_to_whitelist(bid_escrow.address())
        .unwrap();

    reputation_token
        .add_to_whitelist(slashing_voter.address())
        .unwrap();

    reputation_token
        .add_to_whitelist(kyc_voter.address())
        .unwrap();

    kyc_token.add_to_whitelist(kyc_voter.address()).unwrap();

    va_token.add_to_whitelist(bid_escrow.address()).unwrap();

    va_token.add_to_whitelist(slashing_voter.address()).unwrap();

    bid_escrow
        .add_to_whitelist(reputation_token.address())
        .unwrap();

    slashing_voter
        .update_bid_escrow_list(vec![bid_escrow.address()])
        .unwrap();

    (
        env,
        bid_escrow,
        reputation_token,
        va_token,
        kyc_token,
        variable_repo,
        slashing_voter,
        kyc_voter,
    )
}
