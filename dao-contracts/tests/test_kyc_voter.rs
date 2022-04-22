use casper_dao_contracts::{
    DaoOwnedNftContractTest, KycVoterContractTest, ReputationContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{Address, TestContract, TestEnv};
use casper_types::U256;
use speculate::speculate;

speculate! {
    use casper_types::U256;
    use casper_dao_utils::Error;
    use std::time::Duration;

    before {
        #[allow(unused_variables, unused_mut)]
        let (
            applicant,
            another_applicant,
            voter,
            second_voter,
            mint_amount,
            vote_amount,
            document_hash,
            mut kyc_token,
            mut reputation_token,
            mut variable_repo,
            mut contract,
            mut env
        ) = setup();
    }

    describe "voting" {

        test "kyc_token_address_is_set" {
            assert_eq!(
                contract.get_kyc_token_address(),
                kyc_token.address()
            )
        }

        context "applicant_is_not_kyced" {
            before {
                assert_eq!(kyc_token.balance_of(applicant), U256::zero());
            }

            test "voting_creation_succeeds" {
                assert_eq!(
                    contract.as_account(voter).create_voting(applicant, document_hash, vote_amount),
                    Ok(())
                );
            }

            context "voting_is_created" {
                before {
                    contract.as_account(voter).create_voting(applicant, document_hash, vote_amount).unwrap();
                }

                test "cannot_create_next_voting_on_the_same_applicant" {

                }

                test "can_create_next_voting_on_a_different_applicant" {

                }

                test "document_hash_is_available" {

                }

                context "voting_finished" {

                    test "document_hash_is_available" {

                    }

                    context "passed" {
                        test "applicant_owns_kyc_token" {

                        }
                    }

                    context "rejected" {
                        test "next_voting_creation_on_the_same_applicant_succeds" {

                        }
                    }
                }
            }
        }

        context "applicant_is_kyced" {
            before {
                assert_eq!(kyc_token.balance_of(applicant), U256::one());
            }

            test "voting_cannot_be_created" {
                assert_eq!(
                    contract.as_account(voter).create_voting(applicant, document_hash, vote_amount),
                    Err(Error::VaNotKyced)
                );
            }
        }
    }
}

fn setup() -> (
    Address,
    Address,
    Address,
    Address,
    U256,
    U256,
    String,
    DaoOwnedNftContractTest,
    ReputationContractTest,
    VariableRepositoryContractTest,
    KycVoterContractTest,
    TestEnv,
) {
    let env = TestEnv::new();

    let kyc_token = DaoOwnedNftContractTest::new(
        &env,
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );
    let mut reputation_token = ReputationContractTest::new(&env);
    let mut variable_repo = VariableRepositoryContractTest::new(&env);

    let onboarding_voter = KycVoterContractTest::new(
        &env,
        variable_repo.address(),
        reputation_token.address(),
        kyc_token.address(),
    );

    // Voter Contract becomes the owner of Variable Repo and Reputation Token
    variable_repo
        .change_ownership(onboarding_voter.address())
        .unwrap();

    reputation_token
        .change_ownership(onboarding_voter.address())
        .unwrap();
    let applicant = env.get_account(1);
    let voter = env.get_account(2);
    let second_voter = env.get_account(3);
    // The voter has to have some tokens
    let mint_amount = 10_000.into();
    let vote_amount = 1_000.into();
    reputation_token.mint(voter, mint_amount).unwrap();
    reputation_token.mint(second_voter, mint_amount).unwrap();

    (
        applicant,
        another_applicant,
        voter,
        second_voter,
        mint_amount,
        vote_amount,
        "hash".to_string(),
        kyc_token,
        reputation_token,
        variable_repo,
        onboarding_voter,
        env,
    )
}
