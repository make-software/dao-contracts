#[allow(unused_variables, unused_mut)]
mod test {
    extern crate speculate;
    use casper_dao_contracts::mocks::test::MockStakingContractTest;
    use casper_dao_modules::events::{Burn, Mint, TokensStaked, TokensUnstaked, Transfer};
    use casper_dao_utils::{Error, TestEnv};
    use casper_types::U256;
    use speculate::speculate;

    speculate! {
        before {
            let env = TestEnv::new();
            let mut contract = MockStakingContractTest::new(&env);
            let deployer = env.get_account(0);
        }

        context "when initialized" {
            it "has no initial supply" {
                assert_eq!(contract.total_supply(), U256::zero());
            }
            it "has zero balance" {
                assert_eq!(contract.balance_of(deployer), U256::zero());
            }
        }

        describe "mint" {
            before {
                let first_recipient = env.get_account(1);
                let second_recipient = env.get_account(2);
                let first_recipient_supply: U256 = 1_000.into();
                let second_recipient_supply: U256 = 2_000.into();
                let total_supply: U256 = 3_000.into();
                contract.mint(first_recipient, first_recipient_supply).unwrap();
                contract.mint(second_recipient, second_recipient_supply).unwrap();
            }

            it "adjusts recipient's balance" {
                assert_eq!(contract.balance_of(first_recipient), first_recipient_supply);
                assert_eq!(contract.balance_of(second_recipient), second_recipient_supply);
            }

            it "adjusts total supply" {
                assert_eq!(contract.total_supply(), total_supply);
            }

            it "emits a Mint event" {
                contract.assert_last_event(Mint {
                    recipient: second_recipient,
                    value: second_recipient_supply,
                })
            }

            context "when mint amount exceeding U256 capacity" {
                it "reverts" {
                    assert_eq!(
                        contract.mint(first_recipient, U256::max_value()),
                        Err(Error::TotalSupplyOverflow)
                    );
                }
            }
        }

        context "tokens minted" {
            before {
                let holder = env.get_account(1);
                let supply: U256 = 1_000.into();
                let amount: U256 = 100.into();
                contract.mint(holder, supply).unwrap();
            }

            describe "raw transfer" {
                before {
                    let recipient = env.get_account(2);
                    let amount: U256 = 100.into();
                }

                context "when transfer tokens to a recipient" {
                    before {
                        contract.raw_transfer(holder, recipient, amount).unwrap();
                    }

                    it "adjusts balances of both parties" {
                        assert_eq!(contract.balance_of(holder), supply - amount);
                        assert_eq!(contract.balance_of(recipient), amount);
                    }

                    it "emits a Tranfer event" {
                        contract.assert_last_event(Transfer {
                            from: holder,
                            to: recipient,
                            value: amount
                        });
                    }
                }
                context "when transfers amount exceeding balance" {
                    it "reverts" {
                        assert_eq!(
                            contract.raw_transfer(holder, recipient, supply + U256::one()),
                            Err(Error::InsufficientBalance)
                        );
                    }
                }
            }

            describe "staking" {
                before {
                    let amount = 1_000.into();
                    contract.stake(holder, amount).unwrap();
                }
                context "when a holder owns enough tokens" {
                    it "updates staked balance" {
                        assert_eq!(
                            contract.get_stake_of(holder),
                            amount
                        );
                    }

                    it "emits a TokensStaked token staked event" {
                        contract.assert_last_event(TokensStaked { address: holder, amount })
                    }
                }

                context "when a holder doen't have enough tokens" {
                    it "reverts" {
                        assert_eq!(
                            contract.stake(holder, 1.into()),
                            Err(Error::InsufficientBalance)
                        );
                    }
                }

                it "remains balance untouched" {
                    assert_eq!(
                        contract.balance_of(holder),
                        supply
                    );
                }
            }

            describe "unstake" {
                before {
                    let staked_amount = 1_000.into();
                    contract.stake(holder, staked_amount).unwrap();
                    contract.unstake(holder, staked_amount).unwrap();
                }

                context "when a holder owns enough tokens" {
                    it "updates staked balance" {
                        assert_eq!(
                            contract.get_stake_of(holder),
                            U256::zero()
                        );
                    }

                    it "emits a TokensUnstaked token staked event" {
                        contract.assert_last_event(TokensUnstaked { address: holder, amount: staked_amount })
                    }
                }

                context "when a holder doen't have enough tokens" {
                    it "reverts" {
                        assert_eq!(
                            contract.unstake(holder, 1.into()),
                            Err(Error::InsufficientBalance)
                        );
                    }
                }

                it "remains balance untouched" {
                    assert_eq!(
                        contract.balance_of(holder),
                        supply
                    );
                }
            }

            describe "burn" {
                context "when burns tokens" {
                    before {
                        let amount = 100.into();
                        contract.burn(holder, amount).unwrap();
                    }

                    it "adjusts holder's balance" {
                        assert_eq!(contract.balance_of(holder), supply - amount);
                    }

                    it "adjusts total supply" {
                        assert_eq!(contract.total_supply(), supply - amount);
                    }

                    it "emits a Burn event" {
                        contract.assert_last_event(Burn {
                            owner: holder,
                            value: amount,
                        })
                    }
                }

                context "when burns amount exceeding balance" {
                    it "reverts" {
                        assert_eq!(
                            contract.burn(holder, supply + U256::one()),
                            Err(Error::InsufficientBalance)
                        );
                    }
                }
            }
        }
    }
}
