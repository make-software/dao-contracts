use casper_dao_contracts::{
    KycNftContractTest, RepoVoterContractTest, ReputationContractTest, ReputationVoterContractTest,
    VaNftContractTest, VariableRepositoryContractTest, OnboardingVoterContractTest, KycVoterContractTest,
    SimpleVoterContractTest, AdminContractTest
};
use casper_dao_erc721::{TokenId, TokenUri};
use casper_dao_utils::{consts, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

#[test]
fn test_crdao_deployment() {
    // VA: 20 (configurable, const)
    let va_count = 20;

    let env = TestEnv::new();
    let contracts_deployer = env.get_account(0);

    // 1. Deploy Reputation Token Contract.
    let mut reputation_token_contract = ReputationContractTest::new(&env);

    // 1.1 Mint reputation token for initial list of users.
    for n in 1..=va_count {
        let address = env.get_account(n);
        reputation_token_contract
            .mint(address, 1_000.into())
            .unwrap();
    }

    // 2. Deploy Variable Repository Contract.
    let mut variable_repository_contract = VariableRepositoryContractTest::new(&env);
    // 2.1 Change both quorum to 30%

    let new_quorum = U256::from(300);
    let one_day_millis = U256::from(86400000);
    let new_quorum_bytes: Bytes = Bytes::from(new_quorum.to_bytes().unwrap());
    let one_day_bytes: Bytes = Bytes::from(one_day_millis.to_bytes().unwrap());

    variable_repository_contract
        .update_at(
            String::from(consts::INFORMAL_VOTING_QUORUM),
            new_quorum_bytes.clone(),
            None,
        )
        .unwrap();

    variable_repository_contract
        .update_at(
            String::from(consts::FORMAL_VOTING_QUORUM),
            new_quorum_bytes,
            None,
        )
        .unwrap();

    // 2.2 Change voting durations to 1d.
    variable_repository_contract
        .update_at(String::from(consts::VOTING_TIME), one_day_bytes, None)
        .unwrap();

    // 3. Deploy VA Token Contract.
    let mut va_token_contract = VaNftContractTest::new(
        &env,
        String::from("VA Token"),
        String::from("VAT"),
        TokenUri::default(),
    );
    // 3.1 Mint VA tokens for every VA.
    for n in 1..=va_count {
        let address = env.get_account(n);
        va_token_contract.mint(address, TokenId::from(n)).unwrap();
    }

    // 4. Deploy KYC Token Contract.
    let mut kyc_token_contract = KycNftContractTest::new(
        &env,
        String::from("KYC Token"),
        String::from("KYCT"),
        TokenUri::default(),
    );

    // 5. Deploy Reputation Voter.
    // fn init(&mut self, variable_repo: Address, reputation_token: Address, va_token: Address);
    let mut reputation_voter_contract = ReputationVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );

    // 5.1 Whitelist in Reputation Token Contract.
    reputation_token_contract
        .add_to_whitelist(reputation_voter_contract.address())
        .unwrap();

    // 6. Deploy Repo Voter Contract.
    let mut repo_voter_contract = RepoVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 6.1 Whitelist in Reputation Token Contract.
    reputation_token_contract
        .add_to_whitelist(repo_voter_contract.address())
        .unwrap();
        
    // 6.2 Whitelist in Variable Repository Contract.
    variable_repository_contract
        .add_to_whitelist(repo_voter_contract.address())
        .unwrap();

    // 7. Deploy Onboarding Voter Contract.
    let mut onboarding_voter_contract = OnboardingVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        kyc_token_contract.address(),
        va_token_contract.address(),
    );
    // 7.1 Whitelist in Reputation Token Contract.
    reputation_token_contract
        .add_to_whitelist(onboarding_voter_contract.address())
        .unwrap();
    // 7.2 Whitelist in VA Token Contract
    va_token_contract
        .add_to_whitelist(onboarding_voter_contract.address())
        .unwrap();
    assert_eq!(true, false);

    // 8. Deploy KYC Voter Contract.
    let mut kyc_voter_contract = KycVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
        kyc_token_contract.address(),
    );
    // 8.1 Whitelist in Reputation Token Contract.
    reputation_token_contract
        .add_to_whitelist(kyc_voter_contract.address())
        .unwrap();
    // 8.2 Whitelist in KYC Token Contract
    kyc_token_contract
        .add_to_whitelist(kyc_voter_contract.address())
        .unwrap();

    // 9. Deploy Simple Voter Contract
    let mut simple_voter_contract = SimpleVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 9.1 Whitelist in Reputation Token Contract.
    reputation_token_contract
        .add_to_whitelist(simple_voter_contract.address())
        .unwrap();

    // 10. Deploy Admin Contract.
    let mut admin_contract = AdminContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 10.1.1 Remove itself from the whitelist in Reputation Token Contract.

    // 10.1.2 Change Reputation Token Contract owner to Admin Contract.
    
    // 10.2.1 Remove itself from the whitelist in Variable Repository Contract.
    
    // 10.2.2 Change Variable Repository Contract owner to Admin Contract.
    
    // 10.3.1 Remove itself from the whitelist in VA Token Contract.
    
    // 10.3.2 Change VA Token Contract owner to Admin Contract.
    
    // 10.4.1 Remove itself from the whitelist in KYC Token Contract.
    
    // 10.4.2 Change KYC Token Contract owner to Admin Contract.

    // 11. Onboard new VA.
    // 11.1. Call create_voting on KYC Voter Contract as a VA to mint new KYC Token to a new user.
    // 11.2. All votes in favor.
    // 11.3. Call create_voting on Onboarding Voter Contract as a VA to mint new VA Token to the same user.
    // 11.4. All votes in favor.
    // 11.5. Call create_voting on Reputation Voter Contract as a VA to mint reputation tokens to the same user.
    // 11.6. All votes in favor.

    // 12. The New VA
    // 12.1 Call create_voting on Admin Contract to whitelist itself in Reputation Token.
    // 12.2 All votes against.
}
