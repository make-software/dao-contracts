#[allow(unused_variables, unused_mut)]
mod test {
    extern crate speculate;
    use casper_dao_contracts::mocks::test::MockOwnerContractTest;
    use casper_dao_modules::events::OwnerChanged;
    use casper_dao_utils::{Error, TestEnv};
    use speculate::speculate;

    speculate! {
        before {
            let env = TestEnv::new();
            let mut contract = MockOwnerContractTest::new(&env);
            let deployer = env.get_account(0);
            let owner = env.get_account(1);
            let non_owner = env.get_account(2);
        }
        context "when the module is uninitialized" {
            describe "ensure ownerhip" {
                it "reverts" {
                    assert_eq!(
                        contract.ensure_owner(),
                        Err(Error::OwnerIsNotInitialized)
                    );
                }
            }

            describe "get owner" {
                it "returns None" {
                    assert_eq!(
                        contract.get_owner(),
                        None
                    )
                }
            }

            describe "change ownership" {
                before {
                    contract.change_ownership(owner).unwrap();
                }
                it "works" {
                    assert_eq!(
                        contract.get_owner(),
                        Some(owner)
                    );
                }

                it "emits a OwnerChange event" {
                    contract.assert_last_event(OwnerChanged { new_owner: owner})
                }
            }
        }

        context "when init the module" {
            before {
                contract.as_account(owner).initialize_module(owner).unwrap();
            }

            it "sets an owner" {
                assert_eq!(
                    contract.get_owner().unwrap(),
                    owner
                );
            }

            it "emits a OwnerChange event" {
                contract.assert_last_event(OwnerChanged { new_owner: owner})
            }

            describe "ensure owner" {
                context "when called as the owner" {
                    it "works" {
                        assert_eq!(
                            contract.as_account(owner).ensure_owner(),
                            Ok(())
                        );
                    }
                }

                context "when called as a non-owner" {
                    it "reverts" {
                        assert_eq!(
                            contract.as_account(non_owner).ensure_owner(),
                            Err(Error::NotAnOwner)
                        );
                    }
                }
            }

            describe "change ownership" {
                before {
                    let new_owner = env.get_account(2);
                    contract.change_ownership(new_owner).unwrap();
                }
                it "works" {
                    assert_eq!(
                        contract.get_owner(),
                        Some(new_owner)
                    );
                }

                it "emits a OwnerChange event" {
                    contract.assert_last_event(OwnerChanged { new_owner })
                }
            }
        }
    }
}
