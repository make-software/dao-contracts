use speculate::speculate;

speculate! {
    use casper_dao_contracts::{OnboardingVoterContractTest, DaoOwnedNftContractTest, ReputationContractTest, VariableRepositoryContractTest};
    use casper_types::U256;
    use casper_dao_contracts::voting::onboarding;
    use casper_dao_utils::Error;

    before {
        let env = casper_dao_utils::TestEnv::new();
        #[allow(unused_variables, unused_mut)]
        let mut va_token = DaoOwnedNftContractTest::new(&env, "va token".to_string(), "vat".to_string(), "".to_string());
        #[allow(unused_variables, unused_mut)]
        let mut kyc_token = DaoOwnedNftContractTest::new(&env, "kyc token".to_string(), "kyt".to_string(), "".to_string());
        let mut reputation_token = ReputationContractTest::new(&env);
        let mut variable_repo = VariableRepositoryContractTest::new(&env);

        let mint_amount = 10_000.into();
        let vote_amount = 1_000.into();

        #[allow(unused_variables, unused_mut)]
        let mut contract = OnboardingVoterContractTest::new(
            &env,
            variable_repo.address(),
            reputation_token.address(),
            kyc_token.address(),
            va_token.address(),
        );
        // Voter Contract becomes the owner of Variable Repo and Reputation Token
        variable_repo
                .change_ownership(contract.address())
                .unwrap();

        reputation_token
                .change_ownership(contract.address())
                .unwrap();
        #[allow(unused_variables)]
        let va = env.get_account(1);
        let member = env.get_account(2);
        // The voter has to have some tokens
        reputation_token.mint(member, mint_amount).unwrap();
    }

    describe "voting" {
        context "VA_is_not_onboarded" {
            before {
                assert_eq!(va_token.balance_of(va), U256::zero());
            }

            test "remove_va_voting_creation_fails" {
                assert_eq!(
                    contract.as_account(member).create_voting(onboarding::Action::Remove, va, vote_amount),
                    Err(Error::VaNotOnboarded)
                )
            }

            context "VA_is_not_kyced" {
                before {
                    reputation_token.mint(va, mint_amount).unwrap();
                    assert_eq!(kyc_token.balance_of(va), U256::zero());
                }

                test "voting_creation_fails" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Add, va, vote_amount),
                        Err(Error::VaNotKyced)
                    )
                }
            }

            context "VA_has_no_reputation" {
                before {
                    kyc_token.mint(va, 1.into()).unwrap();
                    assert_eq!(reputation_token.balance_of(va), U256::zero());
                }

                test "voting_creation_fails" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Add, va, vote_amount),
                        Err(Error::InsufficientBalance)
                    )
                }
            }

            context "voting_is_created" {
                before {
                    reputation_token.mint(va, mint_amount).unwrap();
                    kyc_token.mint(va, 1.into()).unwrap();
                    contract.as_account(member).create_voting(onboarding::Action::Add, va, vote_amount).unwrap();
                }

                test "that_add_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Add, va, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                test "that_remove_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Remove, va, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                context "voting_passed" {
                    test "VA_owns_a_va_token" {
                        assert_eq!(va_token.balance_of(va), U256::one())
                    }
                }

                context "voting_rejected" {
                    test "VA_does_not_own_va_token" {
                        assert_eq!(va_token.balance_of(va), U256::zero())
                    }
                }
            }
        }

        context "VA_is_already_onboarded" {
            before {
                va_token.mint(va, 1.into()).unwrap();
            }

            test "that_add_va_voting_cannot_be_created" {
                assert_eq!(
                    contract.as_account(member).create_voting(onboarding::Action::Add, va, vote_amount),
                    Err(Error::VaOnboardedAlready)
                )
            }

            context "when_VA_has_no_reputation" {
                before {
                    assert_eq!(
                        reputation_token.balance_of(va),
                        U256::zero()
                    );
                }
                test "that_voting_creation_fails" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Remove, va, vote_amount),
                        Err(Error::InsufficientBalance)
                    )
                }
            }

            context "a_remove_voting_is_created" {
                before {
                    reputation_token.mint(va, mint_amount).unwrap();
                    contract.as_account(member).create_voting(onboarding::Action::Remove, va, vote_amount).unwrap();
                }

                test "that_add_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Add, va, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                test "that_remove_voting_cannot_be_created" {
                    assert_eq!(
                        contract.as_account(member).create_voting(onboarding::Action::Remove, va, vote_amount),
                        Err(Error::OnboardingAlreadyInProgress)
                    )
                }

                context "voting_passed" {
                    test "that_VA_has_no_va_token" {
                        assert_eq!(va_token.balance_of(va), U256::zero())
                    }
                }

                context "voting_rejected" {
                    test "that_VA_still_owns_va_token" {
                        assert_eq!(va_token.balance_of(va), U256::one())
                    }
                }
            }

        }
    }
}
