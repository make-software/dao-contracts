#[allow(unused_variables)]
mod test {
    extern crate speculate;
    use casper_dao_erc721::{
        events::{Approval, ApprovalForAll, Transfer},
        ERC721Test,
        MockERC721NonReceiverTest,
        MockERC721ReceiverTest,
        Received,
        TokenId,
    };
    use casper_dao_utils::{Address, BytesConversion, Error, TestContract, TestEnv};
    use speculate::speculate;

    speculate! {
        static NAME: &str = "Plascoin";
        static SYMBOL: &str = "PLS";
        static BASE_URI: &str = "some://base/uri";

        context "erc721" {

            before {
                let env = TestEnv::new();
                let mut token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL), String::from(BASE_URI));

                let first_token_id: TokenId = 1.into();
                let non_existent_token_id: TokenId = 999.into();
                let operator = env.get_account(0);
                let token_owner = env.get_account(1);
                let another_user = env.get_account(2);
                let approved = env.get_account(3);
                let recipient = env.get_account(4);
            }

            describe "deploy" {
                context "once the contract is deployed" {
                    #[allow(unused_mut)]
                    it "sets the given name" {
                        assert_eq!(token.name(), String::from(NAME));
                    }
                    #[allow(unused_mut)]
                    it "sets the given symbole" {
                        assert_eq!(token.symbol(), String::from(SYMBOL));
                    }
                    #[allow(unused_mut)]
                    it "has no initial supply" {
                        assert_eq!(token.total_supply(), 0.into());
                    }
                }
            }

            describe "mint" {
                before {
                    token.mint(token_owner, first_token_id).unwrap();
                }

                it "increases total supply" {
                    assert_eq!(token.total_supply(), 1.into());
                }

                it "increases balance of the owner" {
                    assert_eq!(token.balance_of(token_owner), 1.into());
                }

                it "sets the ownership to the new token owner" {
                    assert_eq!(token.owner_of(first_token_id), Some(token_owner));
                }

                it "emits a Transfer event" {
                    token.assert_event_at(
                        0,
                        Transfer {
                            from: None,
                            to: Some(token_owner),
                            token_id: first_token_id,
                        },
                    );
                }

                context "minting an existing token" {
                    it "reverts" {
                        assert_eq!(
                            token.mint(token_owner, first_token_id),
                            Err(Error::TokenAlreadyExists)
                        );
                    }
                }
            }

            describe "with minted tokens" {
                before {
                    let second_token_id: TokenId = 2.into();
                    token.mint(token_owner, first_token_id).unwrap();
                    token.mint(token_owner, second_token_id).unwrap();

                    assert_eq!(token.total_supply(), 2.into());
                    assert_eq!(token.balance_of(token_owner), 2.into());
                }

                describe "balance of" {
                    context "when the given address owns some tokens" {
                        it "returns the amount of tokens owned by the given address" {
                            assert_eq!(token.balance_of(token_owner), 2.into());
                        }
                    }

                    context "when the given address does not own any tokens" {
                        it "returns the amount of tokens owned by the given address" {
                            assert_eq!(token.balance_of(another_user), 0.into());
                        }
                    }
                }

                describe "owner of" {
                    context "when the given token id was tracked by this token" {
                        it "returns the owner of the given token ID" {
                            let token_id = first_token_id;
                            assert_eq!(token.owner_of(token_id), Some(token_owner));
                        }

                    }
                    context "when the given token id was not tracked by this token" {
                        #[should_panic(expected = "TokenDoesNotExist")]
                        it "should panic" {
                            let token_id = non_existent_token_id;
                            token.owner_of(token_id);
                        }
                    }
                }

                describe "approve" {
                    fn assert_no_approval(token: &ERC721Test, token_id: TokenId) {
                        assert_eq!(token.get_approved(token_id), None);
                    }

                    fn assest_approval(token: &ERC721Test, token_id: TokenId, address: Address) {
                        assert_eq!(token.get_approved(token_id), Some(address));
                    }

                    fn assert_event_emitted(
                        token: &ERC721Test,
                        token_id: TokenId,
                        owner: Address,
                        address: Option<Address>)
                    {
                        token.assert_last_event(Approval {
                            owner: Some(owner),
                            approved: address,
                            token_id
                        });
                    }

                    context "when clearing approval" {
                        context "when there was no prior approval" {
                            before {
                                token.as_account(token_owner).approve(None, first_token_id).unwrap();
                            }

                            it "works" {
                                assert_no_approval(&token, first_token_id);
                                assert_event_emitted(&token, first_token_id, token_owner, None);
                            }
                        }

                        context "when there was a prior approval" {
                            before {
                                token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                                token.as_account(token_owner).approve(None, first_token_id).unwrap();
                            }

                            it "works" {
                                assert_no_approval(&token, first_token_id);
                                assert_event_emitted(&token, first_token_id, token_owner, None);
                            }
                        }
                    }

                    context "when approving a non-zero address" {
                        context "when there was no prior approval" {
                            before {
                                token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                            }

                            it "works" {
                                assest_approval(&token, first_token_id, operator);
                                assert_event_emitted(&token, first_token_id, token_owner, Some(operator));
                            }
                        }
                    }

                    context "when there was a prior approval to the same address" {
                        before {
                          token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                          token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                        }

                        it "works" {
                            assest_approval(&token, first_token_id, operator);
                            assert_event_emitted(&token, first_token_id, token_owner, Some(operator));
                        }
                    }

                    context "when there was a prior approval to a different address" {
                        before {
                          token.as_account(token_owner).approve(Some(another_user), first_token_id).unwrap();
                          token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                        }

                        it "works" {
                            assest_approval(&token, first_token_id, operator);
                            assert_event_emitted(&token, first_token_id, token_owner, Some(operator));
                        }
                    }

                    context "when the address that receives the approval is the owner" {
                        it "reverts" {
                            assert_eq!(
                                token.as_account(token_owner).approve(Some(token_owner), first_token_id),
                                Err(Error::ApprovalToCurrentOwner)
                            );
                        }
                    }

                    context "when the sender does not own the given token ID" {
                        it "reverts" {
                            assert_eq!(
                                token.as_account(another_user).approve(Some(operator), first_token_id),
                                Err(Error::ApproveCallerIsNotOwnerNorApprovedForAll)
                            );
                        }
                    }

                    context "when the sender is approved for the given token ID" {
                        it "reverts" {
                            token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                            assert_eq!(
                                token.as_account(operator).approve(Some(another_user), first_token_id),
                                Err(Error::ApproveCallerIsNotOwnerNorApprovedForAll)
                            );
                        }
                    }

                    context "when the sender is an operator" {
                        before {
                            token.as_account(token_owner).set_approval_for_all(operator, true,).unwrap();
                            token.as_account(operator).approve(Some(another_user), first_token_id).unwrap();
                        }
                        it "works" {
                            assest_approval(&token, first_token_id, another_user);
                            assert_event_emitted(&token, first_token_id, token_owner, Some(another_user));
                        }
                    }

                    context "when the given token id does not exist" {
                        it "reverts" {
                            assert_eq!(
                                token.as_account(token_owner).approve(Some(operator), non_existent_token_id),
                                Err(Error::TokenDoesNotExist)
                            );
                        }
                    }
                }

                describe "set approval for all" {
                    context "when the operator willing to approve is not the owner" {
                        context "when there is no operator approval set by the sender" {
                            it "approves the operator" {
                                token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                                assert!(token.is_approved_for_all(token_owner, operator));
                            }

                            it "emits an approval event" {
                                token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                                token.assert_last_event(ApprovalForAll {
                                    owner: token_owner,
                                    operator,
                                    approved: true
                                });
                            }
                        }
                    }

                    context "when the operator was set as not approved" {
                        before {
                            token.as_account(token_owner).set_approval_for_all(operator, false).unwrap();
                        }

                        it "approves the operator" {
                            token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                            assert!(token.is_approved_for_all(token_owner, operator));
                        }

                        it "emits an approval event" {
                            token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                            token.assert_last_event(ApprovalForAll {
                                owner: token_owner,
                                operator,
                                approved: true
                            });
                        }

                        it "can unset the operator approval" {
                            token.as_account(token_owner).set_approval_for_all(operator, false).unwrap();
                            assert!(!token.is_approved_for_all(token_owner, operator));
                        }
                    }

                    context "when the operator was already approved" {
                        before {
                            token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                        }

                        it "keeps the approval to the given address" {
                            token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                            assert!(token.is_approved_for_all(token_owner, operator));
                        }

                        it "emits an approval event" {
                            token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                            token.assert_last_event(ApprovalForAll {
                                owner: token_owner,
                                operator,
                                approved: true
                            });
                        }
                    }

                    context "when the operator is the owner" {
                        it "reverts" {
                            assert_eq!(
                                token.as_account(token_owner).set_approval_for_all(token_owner, true),
                                Err(Error::ApproveToCaller)
                            );
                        }
                    }
                }

                describe "get approved" {
                    context "when token is not minted" {
                        #[should_panic(expected = "TokenDoesNotExist")]
                        it "should panic" {
                            token.get_approved(non_existent_token_id);
                        }
                    }


                    context "when token has been minted" {
                        it "reverts" {
                            assert_eq!(
                                token.get_approved(first_token_id),
                                None
                            );
                        }

                        context "when account has been approved" {
                            before {
                                token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                            }

                            it "returns approved account" {
                                assert_eq!(
                                    token.get_approved(first_token_id),
                                    Some(operator)
                                );
                            }
                        }
                    }
                }

                describe "transfers" {
                    before {
                        token.as_account(token_owner).approve(Some(approved), first_token_id).unwrap();
                        token.as_account(token_owner).set_approval_for_all(operator, true).unwrap();
                    }

                    fn transfer_was_successful(
                        token: ERC721Test,
                        owner: Address,
                        token_id: TokenId,
                        new_owner: Option<Address>
                    ) {
                        //transfers the ownership of the given token id to the given address
                        assert_eq!(
                            token.owner_of(token_id),
                            new_owner,
                        );

                        //emits a Transfer event
                        token.assert_last_event(Transfer {
                            from: Some(owner),
                            to: new_owner,
                            token_id
                         });

                        //clears the approval for the token id
                        assert_eq!(
                            token.get_approved(token_id),
                            None
                        );

                        //emits an Approval event
                        token.assert_event_at(-2, Approval {
                            owner: Some(owner),
                            approved: None,
                            token_id
                        });

                        //adjusts owners balances
                        assert_eq!(
                            token.balance_of(owner),
                            1.into()
                        );
                    }

                    describe "transfer from" {
                        context "when called by the owner" {
                            it "works" {
                                token.as_account(token_owner)
                                    .transfer_from(token_owner, recipient, first_token_id)
                                    .unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                            }
                        }

                        context "when called by the approved individual" {
                            it "works" {
                                token.as_account(approved)
                                    .transfer_from(token_owner, recipient, first_token_id)
                                    .unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                            }
                        }

                        context "when called by the operator" {
                            it "works" {
                                token.as_account(operator)
                                    .transfer_from(token_owner, recipient, first_token_id)
                                    .unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                            }
                        }

                        context "when called by the owner without an approved user" {
                            before {
                                token.as_account(token_owner).approve(None, first_token_id).unwrap();
                            }
                            it "works" {
                                token.as_account(operator)
                                    .transfer_from(token_owner, recipient, first_token_id)
                                    .unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                            }
                        }

                        context "when sent to the owner" {
                            before {
                                token.as_account(token_owner)
                                    .transfer_from(token_owner, token_owner, first_token_id)
                                    .unwrap();
                            }

                            it "keeps ownership of the token" {
                                assert_eq!(
                                    token.owner_of(first_token_id),
                                    Some(token_owner),
                                );
                            }
                            it "clears the approval for the token id" {
                                assert_eq!(
                                    token.get_approved(first_token_id),
                                    None,
                                );
                            }
                            it "emits only a transfer event" {
                                token.assert_last_event(Transfer {
                                    from: Some(token_owner),
                                    to: Some(token_owner),
                                    token_id: first_token_id
                                });
                            }

                            it "keeps the owner balance" {
                                assert_eq!(
                                    token.balance_of(token_owner),
                                    2.into(),
                                );
                            }
                        }

                        context "when the address of the previous owner is incorrect" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(token_owner).transfer_from(another_user, another_user, first_token_id),
                                    Err(Error::TransferFromIncorrectOwner)
                                );
                            }
                        }

                        context "when the sender is not authorized for the token id" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(another_user).transfer_from(token_owner, another_user, first_token_id),
                                    Err(Error::CallerIsNotOwnerNorApproved)
                                );
                            }
                        }

                        context "when the given token id does not exist" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(token_owner).transfer_from(token_owner, another_user, non_existent_token_id),
                                    Err(Error::TokenDoesNotExist)
                                );
                            }
                        }
                    }

                    describe "safe transfer from" {
                        context "to a user account" {
                            context "when called by the owner" {
                                it "works" {
                                    token.as_account(token_owner)
                                        .safe_transfer_from(token_owner, recipient, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                                }
                            }

                            context "when called by the approved individual" {
                                it "works" {
                                    token.as_account(approved)
                                        .safe_transfer_from(token_owner, recipient, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                                }
                            }

                            context "when called by the operator" {
                                it "works" {
                                    token.as_account(operator)
                                        .safe_transfer_from(token_owner, recipient, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                                }
                            }

                            context "when called by the owner without an approved user" {
                                before {
                                    token.as_account(token_owner).approve(None, first_token_id).unwrap();
                                }
                                it "works" {
                                    token.as_account(operator)
                                        .safe_transfer_from(token_owner, recipient, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(recipient));
                                }
                            }

                            context "when sent to the owner" {
                                before {
                                    token.as_account(token_owner)
                                        .safe_transfer_from(token_owner, token_owner, first_token_id, None)
                                        .unwrap();
                                }

                                it "keeps ownership of the token" {
                                    assert_eq!(
                                        token.owner_of(first_token_id),
                                        Some(token_owner),
                                    );
                                }
                                it "clears the approval for the token id" {
                                    assert_eq!(
                                        token.get_approved(first_token_id),
                                        None,
                                    );
                                }
                                it "emits only a transfer event" {
                                    token.assert_last_event(Transfer {
                                        from: Some(token_owner),
                                        to: Some(token_owner),
                                        token_id: first_token_id
                                    });
                                }

                                it "keeps the owner balance" {
                                    assert_eq!(
                                        token.balance_of(token_owner),
                                        2.into(),
                                    );
                                }
                            }

                            context "when the address of the previous owner is incorrect" {
                                it "reverts" {
                                    assert_eq!(
                                        token.as_account(token_owner).safe_transfer_from(another_user, recipient, first_token_id, None),
                                        Err(Error::TransferFromIncorrectOwner)
                                    );
                                }
                            }

                            context "when the sender is not authorized for the token id" {
                                it "reverts" {
                                    assert_eq!(
                                        token.as_account(another_user).safe_transfer_from(token_owner, another_user, first_token_id, None),
                                        Err(Error::CallerIsNotOwnerNorApproved)
                                    );
                                }
                            }

                            context "when the given token id does not exist" {
                                it "reverts" {
                                    assert_eq!(
                                        token.as_account(token_owner).safe_transfer_from(token_owner, another_user, non_existent_token_id, None),
                                        Err(Error::TokenDoesNotExist)
                                    );
                                }
                            }
                        }
                        context "to a valid receiver contract" {
                            before {
                                let receiver_contract = MockERC721ReceiverTest::new(&env);
                                let receiver_contract_address = Address::from(receiver_contract.get_package_hash());
                            }

                            context "when called by the owner" {
                                it "works" {
                                    token.as_account(token_owner)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(receiver_contract_address));
                                }
                            }

                            context "when called by the approved individual" {
                                it "works" {
                                    token.as_account(approved)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(receiver_contract_address));
                                }
                            }

                            context "when called by the operator" {
                                it "works" {
                                    token.as_account(operator)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(receiver_contract_address));
                                }
                            }

                            context "when called by the owner without an approved user" {
                                before {
                                    token.as_account(token_owner).approve(None, first_token_id).unwrap();
                                }
                                it "works" {
                                    token.as_account(operator)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None)
                                        .unwrap();
                                    transfer_was_successful(token, token_owner, first_token_id, Some(receiver_contract_address));
                                }
                            }


                            context "when the address of the previous owner is incorrect" {
                                it "reverts" {
                                    assert_eq!(
                                        token.as_account(token_owner).safe_transfer_from(another_user, receiver_contract_address, first_token_id, None),
                                        Err(Error::TransferFromIncorrectOwner)
                                    );
                                }
                            }

                            context "when the sender is not authorized for the token id" {
                                it "reverts" {
                                    assert_eq!(
                                        token.as_account(another_user).safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None),
                                        Err(Error::CallerIsNotOwnerNorApproved)
                                    );
                                }
                            }

                            context "when the given token id does not exist" {
                                it "reverts" {
                                    assert_eq!(
                                        token.as_account(token_owner).safe_transfer_from(token_owner, receiver_contract_address, non_existent_token_id, None),
                                        Err(Error::TokenDoesNotExist)
                                    );
                                }
                            }
                            context "without data" {
                                it "calls on_ERC721_received" {
                                    token.as_account(token_owner)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None)
                                        .unwrap();
                                    receiver_contract.assert_last_event(Received {
                                        operator: token_owner,
                                        from: token_owner,
                                        token_id: first_token_id,
                                        data: None
                                    });
                                }

                                it "calls on_ERC721_received from approved" {
                                    token.as_account(approved)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, None)
                                        .unwrap();
                                    receiver_contract.assert_last_event(Received {
                                        operator: approved,
                                        from: token_owner,
                                        token_id: first_token_id,
                                        data: None
                                    });
                                }
                            }

                            context "with data" {
                                before {
                                    let magic_value = BytesConversion::convert_to_bytes(&"value".to_string()).unwrap();
                                }
                                it "calls on_ERC721_received" {
                                    token.as_account(token_owner)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, Some(magic_value.clone()))
                                        .unwrap();
                                    receiver_contract.assert_last_event(Received {
                                        operator: token_owner,
                                        from: token_owner,
                                        token_id: first_token_id,
                                        data: Some(magic_value)
                                    });
                                }

                                it "calls on_ERC721_received from approved" {
                                    token.as_account(approved)
                                        .safe_transfer_from(token_owner, receiver_contract_address, first_token_id, Some(magic_value.clone()))
                                        .unwrap();
                                    receiver_contract.assert_last_event(Received {
                                        operator: approved,
                                        from: token_owner,
                                        token_id: first_token_id,
                                        data: Some(magic_value)
                                    });
                                }
                            }
                        }

                        describe "to a contract that does not implement the required function" {
                            before {
                                let invalid_receiver = MockERC721NonReceiverTest::new(&env);
                                let invalid_receiver_address = Address::from(invalid_receiver.get_package_hash());
                            }

                            it "reverts" {
                                assert_eq!(
                                    token.as_account(approved).safe_transfer_from(token_owner, invalid_receiver_address, first_token_id, None),
                                    Err(Error::NoSuchMethod("on_erc_721_received".to_string()))
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
