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
    use casper_dao_contracts::voting::VotingId;
    use casper_dao_contracts::voting::Choice;

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

                test "cannot_create_next_voting_for_the_same_applicant" {
                    assert_eq!(
                        contract.as_account(voter).create_voting(applicant, document_hash, vote_amount),
                        Err(Error::KycAlreadyInProgress)
                    );
                }

                test "can_create_next_voting_for_a_different_applicant" {
                    assert_eq!(
                        contract.as_account(voter).create_voting(another_applicant, document_hash, vote_amount),
                        Ok(())
                    );
                }

                context "informal_voting_passed" {
                    before {
                        let voting_id = 0.into();
                        let voting = contract.get_voting(voting_id).unwrap();
                        env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));
                        contract.as_account(voter).finish_voting(voting_id).unwrap();
                        #[allow(unused_variables)]
                        let voting_id: VotingId = 1.into();
                    }
                    test "cannot_create_next_voting_for_the_same_applicant" {
                        assert_eq!(
                            contract.as_account(voter).create_voting(applicant, document_hash, vote_amount),
                            Err(Error::KycAlreadyInProgress)
                        );
                    }

                    context "passed" {
                        before {
                            contract.as_account(second_voter).vote(voting_id, Choice::InFavor,  vote_amount).unwrap();
                            env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                            contract.as_account(voter).finish_voting(voting_id).unwrap();
                        }

                        test "applicant_owns_kyc_token" {
                            assert_eq!(
                                kyc_token.balance_of(applicant),
                                U256::one()
                            );
                        }
                    }

                    context "rejected" {
                        before {
                            contract.as_account(second_voter).vote(voting_id, Choice::Against, vote_amount + U256::one()).unwrap();
                            env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                            contract.as_account(voter).finish_voting(voting_id).unwrap();
                        }
                        test "next_voting_creation_for_the_same_applicant_succeeds" {
                            assert_eq!(
                                contract.as_account(voter).create_voting(applicant, document_hash, vote_amount),
                                Ok(())
                            );
                        }

                        test "applicant_does_not_own_kyc_token" {
                            assert_eq!(
                                kyc_token.balance_of(applicant),
                                U256::zero()
                            );
                        }
                    }
                }
            }
        }

        context "applicant_is_kyced" {
            before {
                kyc_token.mint(applicant, 1.into()).unwrap();
            }

            test "voting_cannot_be_created" {
                assert_eq!(
                    contract.as_account(voter).create_voting(applicant, document_hash, vote_amount),
                    Err(Error::UserKycedAlready)
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
    U256,
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
    let another_applicant = env.get_account(2);
    let voter = env.get_account(3);
    let second_voter = env.get_account(4);
    // The voter has to have some tokens
    let mint_amount = 10_000.into();
    let vote_amount = 1_000.into();
    reputation_token.mint(voter, mint_amount).unwrap();
    reputation_token.mint(second_voter, mint_amount).unwrap();
    let document_hash = 1234.into();

    (
        applicant,
        another_applicant,
        voter,
        second_voter,
        mint_amount,
        vote_amount,
        document_hash,
        kyc_token,
        reputation_token,
        variable_repo,
        onboarding_voter,
        env,
    )
}
