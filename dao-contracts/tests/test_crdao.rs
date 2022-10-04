use std::time::Duration;

use casper_dao_contracts::{
    reputation_voter,
    voting::{onboarding_info::OnboardingAction, Choice, VotingId},
    AdminContractTest, KycNftContractTest, KycVoterContractTest, OnboardingVoterContractTest,
    RepoVoterContractTest, ReputationContractTest, ReputationVoterContractTest,
    SimpleVoterContractTest, VaNftContractTest, VariableRepositoryContractTest, action::Action,
};
use casper_dao_erc721::{TokenId, TokenUri};
use casper_dao_utils::{consts, Address, DocumentHash, TestContract, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    U256,
};

const ONE_DAY_IN_SECS: u64 = 86400000u64;
const DEFAULT_REPUTATION_TOKEN_BALANCE: u32 = 1_000;
const DEFAULT_STAKE: u32 = 100;
const VA_COUNT: usize = 20;

/// CRDAO deployment scenario:
/// 
/// ### INITIAL SETUP ###
/// 0. There are 20 VAs. Every contract is deployed by the same non-VA account.
/// 1. Deploy Reputation Token contract.
///     1.1 Mint Reputation Token for each VA.
/// 
/// ### VARIABLE REPO SETUP ###
/// 2. Deploy Variable Repository Contract.
///     2.1 Change (in)formal voting quorum to 30%.
///     2.2 Change (in)formal voting duration to 1 day.
///
/// ### NFT CONTRACTS SETUP ###
/// 3. Deploy VA Token contract.
///     3.1 Mint a VA token for each VA.
/// 4. Deploy KYC Token contract.
/// 
/// ### VOTERS CONTRACTS SETUP ###
/// 5. Deploy Reputation Voter contract.
///     5.1 Whitelist the contract in Reputation Token contract.
/// 6. Deploy Repo Voter contract.
///     6.1 Whitelist the contract in Reputation Token contract.
///     6.2 Whitelist the contract in Variable Repository contract.
/// 7. Deploy Onboarding Voter contract.
///     7.1 Whitelist the contract in Reputation Token contract.
///     7.2 Whitelist the contract in VA Token contract.
/// 8. Deploy KYC Voter contract.
///     8.1 Whitelist the contract in Reputation Token contract.
///     8.2 Whitelist the contract in KYC Token contract.
/// 9. Deploy Simple Voter contract.
///     9.1 Whitelist the contract in Reputation Token contract.
/// 
/// ### OWNERSHIP MANAGEMENT ###
/// 10. Deploy Admin contract.
///     10.1.1 Remove the deployer from the whitelist in Reputation Token contract.
///     10.1.2 Change Reputation Token contract owner to Admin contract.
///     10.2.1 Remove the deployer from the whitelist in Variable Repository contract.
///     10.2.2 Change Variable Repository contract owner to Admin contract.
///     10.3.1 Remove the deployer from the whitelist in VA Token contract.
///     10.3.2 Change VA Token contract owner to Admin contract.
///     10.4.1 Remove the deployer from the whitelist in KYC Token contract.
///     10.4.2 Change KYC Token contract owner to Admin contract.
/// 
/// ### ONBOARDING ###
/// 11. Onboard a new VA.
///     11.1. VA creates voting in KYC Voter contract to mint a KYC Token to a VA candidate user.
///     11.2. Every VA votes in favor.
///     11.3. VA creates voting in Reputation Voter contract to mint reputation tokens to the VA candidate user.
///     11.4. Every VA votes in favor.
///     11.5. VA creates voting in Onboarding Voter contract to mint a new VA Token to the VA candidate user.
///     11.6. Every VA votes in favor.
/// 
/// ### ONBOARDING VERIFICATION ###
/// 12. The New VA creates voting.
///     12.1 The newly accepted VA creates voting in Admin contract to whitelist the contract in Reputation Token contract.
///     12.2 Every VA votes against.
#[test]
fn test_crdao_deployment() {
    let env = TestEnv::new();
    let config = TestConfig::new(&env);

    // 1. Deploy Reputation Token contract.
    let mut reputation_token_contract = ReputationContractTest::new(&env);

    // 1.1 Mint Reputation Tokens for each VA.
    for n in 1..=config.va_count {
        let address = env.get_account(n);
        reputation_token_contract
            .mint(address, config.default_balance)
            .unwrap();
    }

    // 2. Deploy Variable Repository contract.
    let mut variable_repository_contract = VariableRepositoryContractTest::new(&env);

    // 2.1 Change (in)formal voting quorum to 30%.
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

    // 2.2 Change (in)formal voting duration to 1 day.
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

    // 3. Deploy VA Token contract.
    let mut va_token_contract = VaNftContractTest::new(
        &env,
        String::from("VA Token"),
        String::from("VAT"),
        config.default_token_uri(),
    );
    // 3.1 Mint a VA token for each VA.
    for n in 0..config.va_count {
        let address = config.get_va(n);
        va_token_contract.mint(address, TokenId::MAX - n).unwrap();
    }

    // 4. Deploy KYC Token contract.
    let mut kyc_token_contract = KycNftContractTest::new(
        &env,
        String::from("KYC Token"),
        String::from("KYCT"),
        config.default_token_uri(),
    );

    // 5. Deploy Reputation Voter contract.
    let mut reputation_voter_contract = ReputationVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );

    // 5.1 Whitelist the contract in Reputation Token contract.
    reputation_token_contract
        .add_to_whitelist(reputation_voter_contract.address())
        .unwrap();

    // 6. Deploy Repo Voter contract.
    let repo_voter_contract = RepoVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 6.1 Whitelist the contract in Reputation Token contract.
    reputation_token_contract
        .add_to_whitelist(repo_voter_contract.address())
        .unwrap();

    // 6.2 Whitelist the contract in Variable Repository contract.
    variable_repository_contract
        .add_to_whitelist(repo_voter_contract.address())
        .unwrap();

    // 7. Deploy Onboarding Voter contract.
    let mut onboarding_voter_contract = OnboardingVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        kyc_token_contract.address(),
        va_token_contract.address(),
    );
    // 7.1 Whitelist the contract in Reputation Token contract.
    reputation_token_contract
        .add_to_whitelist(onboarding_voter_contract.address())
        .unwrap();
    // 7.2 Whitelist the contract in VA Token contract
    va_token_contract
        .add_to_whitelist(onboarding_voter_contract.address())
        .unwrap();

    // 8. Deploy KYC Voter contract.
    let mut kyc_voter_contract = KycVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
        kyc_token_contract.address(),
    );
    // 8.1 Whitelist the contract in Reputation Token contract.
    reputation_token_contract
        .add_to_whitelist(kyc_voter_contract.address())
        .unwrap();
    // 8.2 Whitelist the contract in KYC Token contract.
    kyc_token_contract
        .add_to_whitelist(kyc_voter_contract.address())
        .unwrap();

    // 9. Deploy Simple Voter contract.
    let simple_voter_contract = SimpleVoterContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 9.1 Whitelist the contract in Reputation Token contract.
    reputation_token_contract
        .add_to_whitelist(simple_voter_contract.address())
        .unwrap();

    // 10. Deploy Admin contract.
    let mut admin_contract = AdminContractTest::new(
        &env,
        variable_repository_contract.address(),
        reputation_token_contract.address(),
        va_token_contract.address(),
    );
    // 10.1.1 Remove the deployer from the whitelist in Reputation Token contract.
    reputation_token_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.1.2 Change Reputation Token contract owner to Admin contract.
    reputation_token_contract
        .change_ownership(admin_contract.address())
        .unwrap();
    // 10.2.1 Remove the deployer from the whitelist in Variable Repository contract.
    variable_repository_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.2.2 Change Variable Repository contract owner to Admin contract.
    variable_repository_contract
        .change_ownership(admin_contract.address())
        .unwrap();
    // 10.3.1 Remove the deployer from the whitelist in VA Token contract.
    va_token_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.3.2 Change VA Token contract owner to Admin contract.
    va_token_contract
        .change_ownership(admin_contract.address())
        .unwrap();
    // 10.4.1 Remove the deployer from the whitelist in KYC Token contract.
    kyc_token_contract
        .remove_from_whitelist(config.deployer())
        .unwrap();
    // 10.4.2 Change KYC Token contract owner to Admin contract.
    kyc_token_contract
        .change_ownership(admin_contract.address())
        .unwrap();

    // 11. Onboard a new VA.
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
    
    // 12. The New VA creates voting.
    perform_whitelisting_voting(&mut admin_contract, &config, &reputation_token_contract);
}

fn perform_kyc_voting(
    kyc_voter_contract: &mut KycVoterContractTest,
    config: &TestConfig,
) {
    // 11.1. VA creates voting in KYC Voter contract to mint a KYC Token to a VA candidate user.
    let voting_id: VotingId = 0;
    kyc_voter_contract
        .as_account(config.first_va())
        .create_voting(
            config.va_candidate(),
            config.default_document_hash(),
            config.default_stake,
        )
        .unwrap();

    // 11.2. Every VA votes in favor.
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        kyc_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Informal voting passed, formal voting starts.
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
    // Formal voting passed.
    kyc_voter_contract.finish_voting(voting_id).unwrap();
}

fn perform_mint_voting(
    reputation_voter_contract: &mut ReputationVoterContractTest,
    config: &TestConfig,
) {
    // 11.3. VA creates voting in Reputation Voter contract to mint reputation tokens to the VA candidate user.
    let voting_id: VotingId = 0;
    let amount_to_mint = config.default_balance;

    reputation_voter_contract
        .as_account(config.first_va())
        .create_voting(
            config.va_candidate(),
            reputation_voter::Action::Mint,
            amount_to_mint,
            config.default_document_hash(),
            config.default_stake,
        )
        .unwrap();

    // 11.4.Every VA votes in favor.
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        reputation_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Informal voting passed, formal voting starts.
    reputation_voter_contract.finish_voting(voting_id).unwrap();

    let voting_id: VotingId = voting_id + 1;
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        reputation_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Formal voting passed.
    reputation_voter_contract.finish_voting(voting_id).unwrap();
}

fn perform_onboarding_voting(
    onboarding_voter_contract: &mut OnboardingVoterContractTest,
    config: &TestConfig,
) {
    // 11.5. VA creates voting in Onboarding Voter contract to mint a new VA Token to the VA candidate user.
    let voting_id: VotingId = 0;
    onboarding_voter_contract
        .as_account(config.first_va())
        .create_voting(
            OnboardingAction::Add,
            config.va_candidate(),
            config.default_stake,
        )
        .unwrap();
    // 11.6. Every VA votes in favor.
    // The first voter has voted already while the voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        onboarding_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Informal voting passed, formal voting starts.
    onboarding_voter_contract.finish_voting(voting_id).unwrap();

    let voting_id: VotingId = voting_id + 1;
    // The first voter has voted already while voting creation.
    for n in 1..config.va_count {
        let va = config.get_va(n);
        onboarding_voter_contract
            .as_account(va)
            .vote(voting_id, Choice::InFavor, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // Formal voting passed.
    onboarding_voter_contract.finish_voting(voting_id).unwrap();
}

fn perform_whitelisting_voting(
    admin_contract: &mut AdminContractTest,
    config: &TestConfig,
    reputation_token_contract: &ReputationContractTest
) {
    // 12.1 The newly accepted VA creates voting in Admin contract to whitelist the contract in Reputation Token contract.
    let voting_id: VotingId = 0;
    let admin_contract_address = admin_contract.address();
    admin_contract
        .as_account(config.va_candidate())
        .create_voting(reputation_token_contract.address(), Action::AddToWhitelist, admin_contract_address, config.default_stake)
        .unwrap();

    // 12.2 Every VA votes against.
    for n in 0..config.va_count {
        let va = config.get_va(n);
        admin_contract
            .as_account(va)
            .vote(voting_id, Choice::Against, config.default_stake)
            .unwrap();
    }

    config.wait_until_voting_expires();
    // The proposal is rejected - no formal voting.
    admin_contract.finish_voting(voting_id).unwrap();
}
struct TestConfig<'a> {
    test_env: &'a TestEnv,
    pub default_balance: U256,
    pub default_stake: U256,
    pub va_count: usize,
}

impl<'a> TestConfig<'a> {
    /// Creates a new instance with the default values.
    fn new(env: &'a TestEnv) -> Self {
        Self {
            test_env: env,
            default_balance: DEFAULT_REPUTATION_TOKEN_BALANCE.into(),
            default_stake: DEFAULT_STAKE.into(),
            va_count: VA_COUNT,
        }
    }
}

impl TestConfig<'_> {
    /// Returns the default contracts deployer address.
    fn deployer(&self) -> Address {
        self.test_env.get_account(0)
    }

    /// Returns the first VA address.
    fn first_va(&self) -> Address {
        self.test_env.get_account(1)
    }

    /// Returns a non-VA account address.
    fn va_candidate(&self) -> Address {
        self.test_env.get_account(21)
    }

    /// Returns the default TokenUri.
    fn default_token_uri(&self) -> TokenUri {
        TokenUri::default()
    }

    /// Returns the default DocumentHash.
    fn default_document_hash(&self) -> DocumentHash {
        DocumentHash::default()
    }

    /// Returns n-th VA address.
    fn get_va(&self, n: usize) -> Address {
        self.test_env.get_account(n + 1)
    }

    /// Fast forward to the voting expiration time.
    fn wait_until_voting_expires(&self) {
        self.test_env
            .advance_block_time_by(Duration::from_secs(ONE_DAY_IN_SECS + 1));
    }
}
