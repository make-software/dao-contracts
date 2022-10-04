use std::time::Duration;

use casper_dao_contracts::{
    reputation_voter::Action,
    voting::{onboarding_info::OnboardingAction, Choice, VotingId},
    AdminContractTest, KycNftContractTest, KycVoterContractTest, OnboardingVoterContractTest,
    RepoVoterContractTest, ReputationContractTest, ReputationVoterContractTest,
    SimpleVoterContractTest, VaNftContractTest, VariableRepositoryContractTest,
};
use casper_dao_erc721::{TokenId, TokenUri};
use casper_dao_utils::{consts, Address, DocumentHash, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

const ONE_DAY_IN_SECS: u64 = 86400000u64;

#[test]
fn test_crdao_deployment() {
    let env = TestEnv::new();
    let config = TestConfig::new(&env);

    // 1. Deploy Reputation Token Contract.
    let mut reputation_token_contract = ReputationContractTest::new(&env);

    // 1.1 Mint reputation token for initial list of users.
    for n in 1..=config.va_count {
        let address = env.get_account(n);
        reputation_token_contract
            .mint(address, config.default_balance)
            .unwrap();
    }

    // 2. Deploy Variable Repository Contract.
    let mut variable_repository_contract = VariableRepositoryContractTest::new(&env);

    // 2.1 Change both quorum to 30%
    let new_quorum = U256::from(300);
    let new_quorum_bytes: Bytes = Bytes::from(new_quorum.to_bytes().unwrap());

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
    let one_day_bytes: Bytes = Bytes::from(ONE_DAY_IN_SECS.to_bytes().unwrap());

    variable_repository_contract
        .update_at(
            String::from(consts::INFORMAL_VOTING_TIME),
            one_day_bytes.clone(),
            None,
        )
        .unwrap();

    variable_repository_contract
        .update_at(
            String::from(consts::FORMAL_VOTING_TIME),
            one_day_bytes,
            None,
        )
        .unwrap();

    // 3. Deploy VA Token Contract.
    let mut va_token_contract = VaNftContractTest::new(
        &env,
        String::from("VA Token"),
        String::from("VAT"),
        config.default_token_uri(),
    );
    // 3.1 Mint VA tokens for every VA.
    for n in 1..=config.va_count {
        let address = env.get_account(n);
        va_token_contract.mint(address, TokenId::MAX - n).unwrap();
    }

    // 4. Deploy KYC Token Contract.
    let mut kyc_token_contract = KycNftContractTest::new(
        &env,
        String::from("KYC Token"),
        String::from("KYCT"),
        config.default_token_uri(),
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
    let repo_voter_contract = RepoVoterContractTest::new(
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
    let simple_voter_contract = SimpleVoterContractTest::new(
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
    let admin_contract = AdminContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 10.1.1 Remove itself from the whitelist in Reputation Token Contract.
    reputation_token_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.1.2 Change Reputation Token Contract owner to Admin Contract.
    reputation_token_contract
        .change_ownership(admin_contract.address())
        .unwrap();
    // 10.2.1 Remove itself from the whitelist in Variable Repository Contract.
    variable_repository_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.2.2 Change Variable Repository Contract owner to Admin Contract.
    variable_repository_contract
        .change_ownership(admin_contract.address())
        .unwrap();
    // 10.3.1 Remove itself from the whitelist in VA Token Contract.
    va_token_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.3.2 Change VA Token Contract owner to Admin Contract.
    va_token_contract
        .change_ownership(admin_contract.address())
        .unwrap();
    // 10.4.1 Remove itself from the whitelist in KYC Token Contract.
    kyc_token_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.4.2 Change KYC Token Contract owner to Admin Contract.
    kyc_token_contract
        .change_ownership(admin_contract.address())
        .unwrap();

    // 11. Onboard new VA.
    perform_kyc_voting(&mut kyc_voter_contract, &config);
    assert_eq!(
        kyc_token_contract.balance_of(config.va_candidate()),
        U256::one()
    );

    perform_mint_voting(&mut reputation_voter_contract, &config);
    assert_eq!(
        reputation_token_contract.balance_of(config.va_candidate()),
        config.default_balance
    );

    perform_onboarding_voting(&mut onboarding_voter_contract, &config);
    assert_eq!(
        va_token_contract.balance_of(config.va_candidate()),
        U256::one()
    );

    // 12. The New VA
    // 12.1 Call create_voting on Admin Contract to whitelist itself in Reputation Token.
    // 12.2 All votes against.
}

fn perform_kyc_voting(
    kyc_voter_contract: &mut KycVoterContractTest,
    config: &TestConfig,
) {
    // 11.1. Call create_voting on KYC Voter Contract as a VA to mint new KYC Token to a new user.
    let voting_id: VotingId = 0;
    kyc_voter_contract
        .as_account(config.first_va())
        .create_voting(
            config.va_candidate(),
            config.default_document_hash(),
            config.default_stake,
        )
        .unwrap();

    // 11.2. All votes in favor.
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        kyc_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Finish informal voting and start formal voting.
    kyc_voter_contract.finish_voting(voting_id).unwrap();

    let voting_id: VotingId = voting_id + 1;
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        kyc_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    kyc_voter_contract.finish_voting(voting_id).unwrap();
}

fn perform_mint_voting(
    reputation_voter_contract: &mut ReputationVoterContractTest,
    config: &TestConfig,
) {
    // 11.3. Call create_voting on Reputation Voter Contract as a VA to mint reputation tokens to the same user.
    let voting_id: VotingId = 0;
    let amount_to_mint = config.default_balance;

    reputation_voter_contract
        .as_account(config.first_va())
        .create_voting(
            config.va_candidate(),
            Action::Mint,
            amount_to_mint,
            config.default_document_hash(),
            config.default_stake,
        )
        .unwrap();

    // 11.4. All votes in favor.
    for n in 1..config.va_count {
        // The first voter has voted already while the voting creation.
        let va = config.get_va(n);
        reputation_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Finish informal voting and start formal voting.
    reputation_voter_contract.finish_voting(voting_id).unwrap();

    let voting_id: VotingId = voting_id + 1;
    for n in 1..config.va_count {
        // The first voter has voted already while the voting creation.
        let va = config.get_va(n);
        reputation_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Finish informal voting and start formal voting.
    reputation_voter_contract.finish_voting(voting_id).unwrap();
}

fn perform_onboarding_voting(
    onboarding_voter_contract: &mut OnboardingVoterContractTest,
    config: &TestConfig,
) {
    // 11.5. Call create_voting on Onboarding Voter Contract as a VA to mint new VA Token to the same user.
    let voting_id: VotingId = 0;
    onboarding_voter_contract
        .as_account(config.first_va())
        .create_voting(
            OnboardingAction::Add,
            config.va_candidate(),
            config.default_stake,
        )
        .unwrap();
    // 11.6. All votes in favor.
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        onboarding_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Finish informal voting and start formal voting.
    onboarding_voter_contract.finish_voting(voting_id).unwrap();

    let voting_id: VotingId = voting_id + 1;
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        onboarding_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    onboarding_voter_contract.finish_voting(voting_id).unwrap();
}

struct TestConfig<'a> {
    test_env: &'a TestEnv,
    pub default_balance: U256,
    pub default_stake: U256,
    pub va_count: usize,
}

impl<'a> TestConfig<'a> {
    fn new(env: &'a TestEnv) -> Self {
        Self {
            test_env: env,
            default_balance: 1_000.into(),
            default_stake: 100.into(),
            va_count: 20,
        }
    }
}

impl TestConfig<'_> {
    fn deployer(&self) -> Address {
        self.test_env.get_account(0)
    }

    fn first_va(&self) -> Address {
        self.test_env.get_account(1)
    }

    fn va_candidate(&self) -> Address {
        self.test_env.get_account(21)
    }

    fn default_token_uri(&self) -> TokenUri {
        TokenUri::default()
    }

    fn default_document_hash(&self) -> DocumentHash {
        DocumentHash::default()
    }

    fn get_va(&self, n: usize) -> Address {
        self.test_env.get_account(n + 1)
    }

    fn wait_until_voting_expires(&self) {
        self.test_env
            .advance_block_time_by(Duration::from_secs(ONE_DAY_IN_SECS + 1));
    }
}
