#[allow(unused_variables, unused_mut)]
mod test {
    extern crate speculate;
    use casper_dao_contracts::mocks::test::MockWhitelistContractTest;
    use casper_dao_modules::events::{AddedToWhitelist, RemovedFromWhitelist};
    use casper_dao_utils::{Error, TestEnv};
    use speculate::speculate;

    speculate! {
        before {
            let env = TestEnv::new();
            let mut contract = MockWhitelistContractTest::new(&env);
            let user = env.get_account(1);
            let another_user = env.get_account(2);
        }

        test "adding to whitelist works" {
            assert_eq!(
                contract.add_to_whitelist(user),
                Ok(())
            );
        }

        context "removing a not added user" {
            before {
                contract.remove_from_whitelist(user).unwrap();
            }
            test "emits an event" {
                contract.assert_last_event(
                    RemovedFromWhitelist {
                        address: user
                    }
                );
            }
        }

        context "when a user is whitelisted" {
            before {
                contract.add_to_whitelist(user).unwrap();
            }

            it "emits an AddedToWhitelist event" {
                contract.assert_last_event(
                    AddedToWhitelist {
                        address: user
                    }
                );
            }

            test "the user can ensure is whitelisted" {
                assert_eq!(
                    contract.as_account(user).ensure_whitelisted(),
                    Ok(())
                );
            }

            test "anyone can check another user is whitelisted" {
                assert_eq!(
                    contract.as_account(another_user).is_whitelisted(user),
                    true
                );
            }

            context "when whitelisting already added user" {
                before {
                    contract.add_to_whitelist(user).unwrap();
                }

                it "emits an AddedToWhitelist event" {
                    contract.assert_last_event(
                        AddedToWhitelist {
                            address: user
                        }
                    );
                }

                test "keeps the user whitelisted" {
                    assert_eq!(
                        contract.is_whitelisted(user),
                        true
                    );
                    assert_eq!(
                        contract.as_account(user).ensure_whitelisted(),
                        Ok(())
                    );
                }
            }

            describe "when removes from the whitelist" {
                before {
                    contract.remove_from_whitelist(user).unwrap();
                }

                it "emits a RemovedFromWhitelist event" {
                    contract.assert_last_event(
                        RemovedFromWhitelist {
                            address: user
                        }
                    );
                }

                it "self-check if whitelisted reverts" {
                    assert_eq!(
                        contract.as_account(user).ensure_whitelisted(),
                        Err(Error::NotWhitelisted)
                    )
                }

                it "anyone can check the user is not whitelisted" {
                    assert_eq!(
                        contract.as_account(another_user).is_whitelisted(user),
                        false
                    )
                }
            }
        }
    }
}
