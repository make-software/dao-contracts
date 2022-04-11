#[allow(unused_variables, unused_mut, clippy::redundant_clone)]
mod test {
    extern crate speculate;
    use casper_dao_contracts::mocks::test::MockRepositoryContractTest;
    use casper_dao_modules::events::ValueUpdated;
    use casper_dao_utils::{BytesConversion, Error, TestEnv};
    use speculate::speculate;
    use std::time::Duration;

    speculate! {
        before {
            let env = TestEnv::new();
            let mut contract = MockRepositoryContractTest::new(&env);
            let deployer = env.get_account(0);
            let block_time = env.get_block_time();
            let key = String::from("key");
            let non_existent_key = String::from("elo");
            let value = "value".to_string().convert_to_bytes().unwrap();
            let next_value = "value2".to_string().convert_to_bytes().unwrap();
        }

        test "get non existent value" {
            assert_eq!(
                contract.get(non_existent_key.clone()),
                None
            );
            assert_eq!(
                contract.get_full_value(non_existent_key),
                None
            );
        }

        context "when update immediately" {
            before {
                contract.update_at(key.clone(), value.clone(), None).unwrap();
            }

            it "emits a ValueUpdate event" {
                contract.assert_last_event(ValueUpdated { key, value, activation_time: None });
            }

            it "sets the value properly" {
                assert_eq!(
                    contract.get(key),
                    Some(value)
                )
            }

            test "the future value is None" {
                assert_eq!(
                    contract.get_full_value(key).unwrap().1,
                    None
                )
            }
        }

        context "when update with a date in the past" {
            before {
                let date_in_past = Some(env.get_block_time());
                env.advance_block_time_by(Duration::from_secs(1));
            }

            it "reverts" {
                assert_eq!(
                    contract.update_at(key, value, date_in_past),
                    Err(Error::ActivationTimeInPast)
                );
            }
        }


        context "when update with a future date" {
            context "no current value is set" {
                test "reverts" {
                    let block_time = Duration::from_secs(env.get_block_time());
                    let date_in_future = (block_time + Duration::from_secs(10)).as_secs();
                    assert_eq!(
                        contract.update_at(key, next_value, Some(date_in_future)),
                        Err(Error::ValueNotAvailable)
                    );
                }
            }

            context "the current value is set" {
                before {
                    contract.update_at(key.clone(), value.clone(), None).unwrap();
                    let value_to_set = "value3".to_string().convert_to_bytes().unwrap();
                }

                context "when the next value is set but the time has passed" {
                    before {
                        contract.update_at(key.clone(), next_value.clone(), Some(5)).unwrap();
                        let block_time = Duration::from_secs(env.get_block_time());
                        env.advance_block_time_by(Duration::from_secs(6));
                        let date_in_future = (block_time + Duration::from_secs(10)).as_secs();
                        contract.update_at(key.clone(), value_to_set.clone(), Some(date_in_future)).unwrap();
                    }

                    it "emits a ValueUpdate event" {
                        contract.assert_last_event(ValueUpdated {
                            key,
                            value: value_to_set,
                            activation_time: Some(date_in_future)
                        });
                    }

                    test "prev next value becomes the current value" {
                        assert_eq!(
                            contract.get(key),
                            Some(next_value)
                        )
                    }

                    test "the future value is set" {
                        assert_eq!(
                            contract.get_full_value(key).unwrap().1,
                            Some((value_to_set, date_in_future))
                        )
                    }
                }

                context "when the next value is set in the future" {
                    before {
                        contract.update_at(key.clone(), next_value.clone(), Some(5)).unwrap();
                        let block_time = Duration::from_secs(env.get_block_time());
                        env.advance_block_time_by(Duration::from_secs(4));
                        let date_in_future = (block_time + Duration::from_secs(10)).as_secs();
                        contract.update_at(key.clone(), value_to_set.clone(), Some(date_in_future)).unwrap();
                    }

                    it "emits a ValueUpdate event" {
                        contract.assert_last_event(ValueUpdated {
                            key,
                            value: value_to_set,
                            activation_time: Some(date_in_future)
                        });
                    }

                    test "the current value doesn't change" {
                        assert_eq!(
                            contract.get(key),
                            Some(value)
                        )
                    }

                    test "the future value is set" {
                        assert_eq!(
                            contract.get_full_value(key).unwrap().1,
                            Some((value_to_set, date_in_future))
                        )
                    }
                }
            }
        }
    }
}
