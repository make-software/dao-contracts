use casper_dao_contracts::{
    DaoOwnedNftContractTest, OnboardingVoterContractTest, ReputationContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_utils::{Address, TestContract, TestEnv};
use casper_types::U256;
use speculate::speculate;

speculate! {
    use casper_types::U256;
    use casper_dao_contracts::voting::{Choice, onboarding_info::OnboardingAction};
    use casper_dao_utils::Error;
    use std::time::Duration;

    before {
        #[allow(unused_variables, unused_mut)]
        let (
            user,
            va,
            second_va,
            mint_amount,
            vote_amount,
            mut va_token,
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

        test "va_token_address_it_set" {
            assert_eq!(
                contract.get_va_token_address(),
                va_token.address()
            )
        }

        context "user_is_not_onboarded" {
            before {
                assert_eq!(va_token.balance_of(user), U256::zero());
            }

            test "remove_user_voting_creation_fails" {
                assert_eq!(
                    contract.as_account(va).create_voting(OnboardingAction::Remove, user, vote_amount),
                    Err(Error::VaNotOnboarded)
                )
            }

            context "user_is_not_kyced" {
                before {
                    reputation_token.mint(user, mint_amount).unwrap();
                    assert_eq!(kyc_token.balance_of(user), U256::zero());
                }

                test "voting_creation_fails" {
                    assert_eq!(
                        contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount),
                        Err(Error::VaNotKyced)
                    )
                }
            }

            context "user_has_no_reputation" {
                before {
                    kyc_token.mint(user, 1.into()).unwrap();
                    assert_eq!(reputation_token.balance_of(user), U256::zero());
                }

                test "voting_creation_fails" {
                    assert_eq!(
                        contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount),
                        Err(Error::InsufficientBalance)
                    )
                }
            }

            context "voting_is_created" {
                before {
                    reputation_token.mint(user, mint_amount).unwrap();
                    kyc_token.mint(user, 1.into()).unwrap();
                    contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount).unwrap();
                }

                test "that_add_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                test "that_remove_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(va).create_voting(OnboardingAction::Remove, user, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                context "informal_voting_passed" {
                    before {
                        let voting_id = 0.into();
                        let voting = contract.get_voting(voting_id).unwrap();
                        env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));
                        contract.as_account(va).finish_voting(voting_id).unwrap();
                        let voting_id = 1.into();
                    }

                    context "voting_passed" {
                        before {
                            contract.as_account(second_va).vote(voting_id, Choice::InFavor, vote_amount).unwrap();
                            env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                            contract.as_account(va).finish_voting(voting_id).unwrap();
                        }

                        test "user_owns_a_va_token" {
                            assert_eq!(va_token.balance_of(user), U256::one())
                        }

                        test "remove_voting_creation_succeeds" {
                            assert_eq!(
                                contract.as_account(va).create_voting(OnboardingAction::Remove, user, vote_amount),
                                Ok(())
                            );
                        }
                    }

                    context "voting_rejected" {
                        before {
                            contract.as_account(second_va).vote(voting_id, Choice::Against, vote_amount + U256::one()).unwrap();
                            env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                            contract.as_account(va).finish_voting(voting_id).unwrap();
                        }

                        test "user_does_not_own_va_token" {
                            assert_eq!(va_token.balance_of(user), U256::zero())
                        }

                        test "next_add_voting_creation_succeeds" {
                            assert_eq!(
                                contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount),
                                Ok(())
                            );
                        }
                    }
                }
            }
        }

        context "user_is_already_onboarded" {
            before {
                let token_id = 1.into();
                va_token.mint(user, token_id).unwrap();
                va_token.as_account(user).set_approval_for_all(contract.address(), true).unwrap();
            }

            test "that_add_user_voting_cannot_be_created" {
                assert_eq!(
                    contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount),
                    Err(Error::VaOnboardedAlready)
                )
            }

            context "when_user_has_no_reputation" {
                before {
                    assert_eq!(
                        reputation_token.balance_of(user),
                        U256::zero()
                    );
                }
                test "that_voting_creation_fails" {
                    assert_eq!(
                        contract.as_account(va)
                            .create_voting(OnboardingAction::Remove, user, vote_amount),
                        Err(Error::InsufficientBalance)
                    )
                }
            }

            context "remove_voting_is_created" {
                before {
                    reputation_token.mint(user, mint_amount).unwrap();
                    contract.as_account(va)
                        .create_voting(OnboardingAction::Remove, user, vote_amount)
                        .unwrap();
                }

                test "that_add_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(va)
                            .create_voting(OnboardingAction::Add, user, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                test "that_remove_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(va)
                            .create_voting(OnboardingAction::Remove, user, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                context "informal_voting_passed" {
                    before {
                        let voting_id = 0.into();
                        let voting = contract.get_voting(voting_id).unwrap();
                        env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));
                        contract.as_account(va).finish_voting(voting_id).unwrap();
                        let voting_id = 1.into();
                    }

                    context "voting_passed" {
                        before {
                            contract.as_account(second_va).vote(voting_id, Choice::InFavor, vote_amount).unwrap();
                            env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                            contract.as_account(va).finish_voting(voting_id).unwrap();
                        }

                        test "that_user_has_no_va_token" {
                            assert_eq!(va_token.balance_of(user), U256::zero())
                        }

                        test "add_voting_creation_succeeds" {
                            kyc_token.mint(user, 1.into()).unwrap();
                            assert_eq!(
                                contract.as_account(va).create_voting(OnboardingAction::Add, user, vote_amount),
                                Ok(())
                            );
                        }
                    }

                    context "voting_rejected" {
                        before {
                            contract.as_account(second_va).vote(voting_id, Choice::Against, vote_amount + U256::one()).unwrap();
                            env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                            contract.as_account(va).finish_voting(voting_id).unwrap();
                        }

                        test "that_user_still_owns_va_token" {
                            assert_eq!(va_token.balance_of(user), U256::one())
                        }

                        test "next_remove_voting_creation_succeeds" {
                            assert_eq!(
                                contract.as_account(va).create_voting(OnboardingAction::Remove, user, vote_amount),
                                Ok(())
                            );
                        }
                    }
                }
            }

        }
    }
}

fn setup() -> (
    Address,
    Address,
    Address,
    U256,
    U256,
    DaoOwnedNftContractTest,
    DaoOwnedNftContractTest,
    ReputationContractTest,
    VariableRepositoryContractTest,
    OnboardingVoterContractTest,
    TestEnv,
) {
    let env = TestEnv::new();

    let va_token = DaoOwnedNftContractTest::new(
        &env,
        "user token".to_string(),
        "usert".to_string(),
        "".to_string(),
    );
    let kyc_token = DaoOwnedNftContractTest::new(
        &env,
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );
    let mut reputation_token = ReputationContractTest::new(&env);
    let mut variable_repo = VariableRepositoryContractTest::new(&env);

    let onboarding_voter = OnboardingVoterContractTest::new(
        &env,
        variable_repo.address(),
        reputation_token.address(),
        kyc_token.address(),
        va_token.address(),
    );

    // Voter Contract becomes the owner of Variable Repo and Reputation Token
    variable_repo
        .change_ownership(onboarding_voter.address())
        .unwrap();

    reputation_token
        .change_ownership(onboarding_voter.address())
        .unwrap();
    let user = env.get_account(1);
    let va = env.get_account(2);
    let second_va = env.get_account(3);
    // The voter has to have some tokens
    let mint_amount = 10_000.into();
    let vote_amount = 1_000.into();
    reputation_token.mint(va, mint_amount).unwrap();
    reputation_token.mint(second_va, mint_amount).unwrap();

    (
        user,
        va,
        second_va,
        mint_amount,
        vote_amount,
        va_token,
        kyc_token,
        reputation_token,
        variable_repo,
        onboarding_voter,
        env,
    )
}
