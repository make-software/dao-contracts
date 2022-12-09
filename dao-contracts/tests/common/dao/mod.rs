use casper_dao_contracts::{
    BidEscrowContractTest,
    CSPRRateProviderContractTest,
    KycNftContractTest,
    KycVoterContractTest,
    ReputationContractTest,
    SlashingVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{consts, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U512,
};

mod account;
mod bid_escrow;
mod cspr;
mod events;
mod kyc;
mod ownership;
mod reputation;
mod va;
mod voting;

const DEFAULT_CSPR_USD_RATE: u64 = 2_000_000_000;

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
    CSPRRateProviderContractTest,
) {
    let env = TestEnv::new();
    let rate_provider = CSPRRateProviderContractTest::new(&env, DEFAULT_CSPR_USD_RATE.into());
    dbg!(U512::from(DEFAULT_CSPR_USD_RATE));
    let mut variable_repo = VariableRepositoryContractTest::new(&env);
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

    let bid_escrow = BidEscrowContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        kyc_token.address(),
        va_token.address(),
    );

    let slashing_voter = SlashingVoterContractTest::new(
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

    variable_repo
        .update_at(
            consts::FIAT_CONVERSION_RATE_ADDRESS.to_string(),
            Bytes::from(rate_provider.address().to_bytes().unwrap()),
            None,
        )
        .unwrap();

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
    (
        env,
        bid_escrow,
        reputation_token,
        va_token,
        kyc_token,
        variable_repo,
        slashing_voter,
        kyc_voter,
        rate_provider,
    )
}
