use speculate::speculate;

speculate! {
    use casper_dao_contracts::{OnboardingVoterContractTest, DaoOwnedNftContractTest, ReputationContractTest, VariableRepositoryContractTest};
    use casper_types::U256;
    use casper_dao_contracts::voting::onboarding;

    before {
        let env = casper_dao_utils::TestEnv::new();
        #[allow(unused_variables, unused_mut)]
        let mut va_token = DaoOwnedNftContractTest::new(&env, "va token".to_string(), "vat".to_string(), "".to_string());
        let kyc_token = DaoOwnedNftContractTest::new(&env, "kyc token".to_string(), "kyt".to_string(), "".to_string());
        let reputation_token = ReputationContractTest::new(&env);
        let variable_repo = VariableRepositoryContractTest::new(&env);

        #[allow(unused_variables, unused_mut)]
        let mut contract = OnboardingVoterContractTest::new(
            &env,
            variable_repo.address(),
            reputation_token.address(),
            kyc_token.address(),
            va_token.address(),
        );
        #[allow(unused_variables)]
        let va = env.get_account(1);
    }

    describe "voting" {

        context "VA_is_not_onboarded" {

            context "when_there_is_an_ongoing_voting" {
                test "voting_creation_fails" {
                    assert_eq!(
                        contract.create_voting(onboarding::Action::Add, va, 1_000.into()),
                        Err(casper_dao_utils::Error::Unknown)
                    )
                }
            }

            describe "when_removes_a_VA" {
                test "voting_creation_fails" {
                    assert_eq!(
                        contract.create_voting(onboarding::Action::Remove, va, 1_000.into()),
                        Err(casper_dao_utils::Error::Unknown)
                    )
                }
            }

            context "VA_is_not_kyed" {
                test "voting_creation_fails" {
                    assert_eq!(
                        contract.create_voting(onboarding::Action::Add, va, 1_000.into()),
                        Err(casper_dao_utils::Error::Unknown)
                    )
                }
            }

            context "VA_has_no_reputation" {
                test "voting_creation_fails" {
                    assert_eq!(
                        contract.create_voting(onboarding::Action::Add, va, 1_000.into()),
                        Err(casper_dao_utils::Error::Unknown)
                    )
                }
            }

            context "a_voting_is_created" {

                test "that_an_add_voting_cannot_be_created" {
                    assert_eq!(
                        contract.create_voting(onboarding::Action::Add, va, 1_000.into()),
                        Err(casper_dao_utils::Error::Unknown)
                    )
                }

                test "that_a_remove_voting_cannot_be_created" {
                    assert_eq!(
                        contract.create_voting(onboarding::Action::Remove, va, 1_000.into()),
                        Err(casper_dao_utils::Error::Unknown)
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

            test "that_an_add_va_voting_cannot_be_created" {
                assert_eq!(
                    contract.create_voting(onboarding::Action::Add, va, 1_000.into()),
                    Err(casper_dao_utils::Error::VaOnboardedAlready)
                )
            }
        }
    }
}
