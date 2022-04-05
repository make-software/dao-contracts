extern crate speculate;
use speculate::speculate;
use std::borrow::BorrowMut;

use casper_dao_erc721::{
    events::{Approval, ApprovalForAll, Transfer},
    ERC721Test, MockERC721NonReceiverTest, MockERC721ReceiverTest, TokenId,
};
use casper_dao_utils::{Address, Error, TestEnv};

speculate! {
    static NAME: &str = "Plascoin";
    static SYMBOL: &str = "PLS";

    context "erc721" {

        before {
            let env = TestEnv::new();
            let mut token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));

            let first_token_id: casper_dao_erc721::TokenId = 1.into();
            let second_token_id: casper_dao_erc721::TokenId = 2.into();
            let non_existent_token_id: casper_dao_erc721::TokenId = 999.into();
            let operator = env.get_account(0);
            let token_owner = env.get_account(1);
            let another_user = env.get_account(2);
            let approved = env.get_account(3);
            let recipient = env.get_account(3);
        }

        test "mint works" {
            // When mint a new token
            token.mint(token_owner, first_token_id).unwrap();

            // Then total supply and the minter balance increases, token ownership is set
            assert_eq!(token.total_supply(), 1.into());
            assert_eq!(token.balance_of(token_owner), 1.into());
            assert_eq!(token.owner_of(first_token_id).unwrap(), token_owner);

            // Then emits Transfer event
            token.assert_event_at(
                0,
                Transfer {
                    from: None,
                    to: Some(token_owner),
                    token_id: first_token_id,
                },
            );

            // When mint a token with exisiting id
            let result = token.mint(token_owner, first_token_id);

            // Then it raises an error
            assert_eq!(result, Err(Error::TokenAlreadyExists));
        }

        describe "with minted tokens" {
            before {
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
                        assert_eq!(token.owner_of(token_id).unwrap(), token_owner);
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

                fn assert_event_emitted(token: &ERC721Test, token_id: TokenId, owner: Address, address: Option<Address>) {
                    token.assert_last_event(Approval { owner: Some(owner), operator: address, token_id });
                }

                context "when clearing approval" {
                    context "when there was no prior approval" {
                        before {
                            token.as_account(token_owner).approve(None, first_token_id).unwrap();
                        }

                        test "_" {
                            assert_no_approval(&token, first_token_id);
                            assert_event_emitted(&token, first_token_id, token_owner, None);
                        }
                    }

                    context "when there was a prior approval" {
                        before {
                            token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                            token.as_account(token_owner).approve(None, first_token_id).unwrap();
                        }

                        test "_" {
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

                        test "_" {
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

                    test "_" {
                        assest_approval(&token, first_token_id, operator);
                        assert_event_emitted(&token, first_token_id, token_owner, Some(operator));
                    }
                }

                context "when there was a prior approval to a different address" {
                    before {
                      token.as_account(token_owner).approve(Some(another_user), first_token_id).unwrap();
                      token.as_account(token_owner).approve(Some(operator), first_token_id).unwrap();
                    }

                    test "_" {
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
                    it "_" {
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
                            token.assert_last_event(ApprovalForAll { owner: token_owner, operator, approved: true });
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
                        token.assert_last_event(ApprovalForAll { owner: token_owner, operator, approved: true });
                    }

                    it "can unset the operator approval" {
                        token.as_account(token_owner).set_approval_for_all(operator, false).unwrap();
                        assert_eq!(token.is_approved_for_all(token_owner, operator), false);
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
                        token.assert_last_event(ApprovalForAll { owner: token_owner, operator, approved: true });
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

                fn transfer_was_successful(token: ERC721Test, owner: Address, token_id: TokenId, approved: Option<Address>) {
                    //transfers the ownership of the given token ID to the given address
                    assert_eq!(
                        token.owner_of(token_id),
                        approved
                    );

                    //emits a Transfer event
                    token.assert_event_at(-2, Transfer {
                        from: Some(owner),
                        to: approved,
                        token_id
                     });

                    //clears the approval for the token id
                    assert_eq!(
                        token.get_approved(token_id),
                        None
                    );

                    //emits an Approval event
                    token.assert_last_event(Approval {
                        owner: Some(owner),
                        operator: None,
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
                            token.as_account(token_owner).transfer_from(token_owner, recipient, first_token_id).unwrap();
                            transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                        }
                    }

                    context "when called by the approved individual" {
                        it "works" {
                            token.as_account(approved).transfer_from(token_owner, recipient, first_token_id).unwrap();
                            transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                        }
                    }

                    context "when called by the operator" {
                        it "works" {
                            token.as_account(operator).transfer_from(token_owner, recipient, first_token_id).unwrap();
                            transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                        }
                    }

                    context "when called by the owner without an approved user" {
                        before {
                            token.as_account(token_owner).approve(None, first_token_id).unwrap();
                        }
                        it "works" {
                            token.as_account(operator).transfer_from(token_owner, recipient, first_token_id).unwrap();
                            transfer_was_successful(token, token_owner, first_token_id, None);
                        }
                    }

                    context "when sent to the owner" {
                        before {
                            token.as_account(token_owner).transfer_from(token_owner, token_owner, first_token_id).unwrap();
                        }

                        it "keeps ownership of the token" {
                            assert_eq!(
                                token.owner_of(first_token_id).unwrap(),
                                token_owner,
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
                                token.as_account(token_owner).transfer_from(token_owner, recipient, first_token_id),
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

                    context "when the given token ID does not exist" {
                        it "reverts" {
                            assert_eq!(
                                token.as_account(token_owner).transfer_from(token_owner, another_user, first_token_id),
                                Err(Error::TokenDoesNotExist)
                            );
                        }
                    }
                }

                describe "safe transfer from" {
                    context "to a user account" {
                        context "when called by the owner" {
                            it "works" {
                                token.as_account(token_owner).safe_transfer_from(token_owner, recipient, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                            }
                        }

                        context "when called by the approved individual" {
                            it "works" {
                                token.as_account(approved).safe_transfer_from(token_owner, recipient, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                            }
                        }

                        context "when called by the operator" {
                            it "works" {
                                token.as_account(operator).safe_transfer_from(token_owner, recipient, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                            }
                        }

                        context "when called by the owner without an approved user" {
                            before {
                                token.as_account(token_owner).approve(None, first_token_id).unwrap();
                            }
                            it "works" {
                                token.as_account(operator).safe_transfer_from(token_owner, recipient, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, None);
                            }
                        }

                        context "when sent to the owner" {
                            before {
                                token.as_account(token_owner).safe_transfer_from(token_owner, token_owner, first_token_id).unwrap();
                            }

                            it "keeps ownership of the token" {
                                assert_eq!(
                                    token.owner_of(first_token_id).unwrap(),
                                    token_owner,
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
                                    token.as_account(token_owner).safe_transfer_from(token_owner, recipient, first_token_id),
                                    Err(Error::TransferFromIncorrectOwner)
                                );
                            }
                        }

                        context "when the sender is not authorized for the token id" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(another_user).safe_transfer_from(token_owner, another_user, first_token_id),
                                    Err(Error::CallerIsNotOwnerNorApproved)
                                );
                            }
                        }

                        context "when the given token ID does not exist" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(token_owner).safe_transfer_from(token_owner, another_user, first_token_id),
                                    Err(Error::TokenDoesNotExist)
                                );
                            }
                        }
                    }
                    context "to a valid receiver contract" {

                        before {
                            let receiver_contract = MockERC721ReceiverTest::new(&env);
                            let receiver_contract_address = Address::from(receiver_contract.get_package_hash());

                            let non_receiver = MockERC721NonReceiverTest::new(&env);
                        }

                        context "when called by the owner" {
                            it "works" {
                                token.as_account(token_owner).safe_transfer_from(token_owner, receiver_contract_address, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                            }
                        }

                        context "when called by the approved individual" {
                            it "works" {
                                token.as_account(approved).safe_transfer_from(token_owner, receiver_contract_address, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                            }
                        }

                        context "when called by the operator" {
                            it "works" {
                                token.as_account(operator).safe_transfer_from(token_owner, receiver_contract_address, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, Some(approved));
                            }
                        }

                        context "when called by the owner without an approved user" {
                            before {
                                token.as_account(token_owner).approve(None, first_token_id).unwrap();
                            }
                            it "works" {
                                token.as_account(operator).safe_transfer_from(token_owner, receiver_contract_address, first_token_id).unwrap();
                                transfer_was_successful(token, token_owner, first_token_id, None);
                            }
                        }


                        context "when the address of the previous owner is incorrect" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(token_owner).safe_transfer_from(token_owner, receiver_contract_address, first_token_id),
                                    Err(Error::TransferFromIncorrectOwner)
                                );
                            }
                        }

                        context "when the sender is not authorized for the token id" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(receiver_contract_address).safe_transfer_from(token_owner, receiver_contract_address, first_token_id),
                                    Err(Error::CallerIsNotOwnerNorApproved)
                                );
                            }
                        }

                        context "when the given token id does not exist" {
                            it "reverts" {
                                assert_eq!(
                                    token.as_account(token_owner).safe_transfer_from(token_owner, receiver_contract_address, first_token_id),
                                    Err(Error::TokenDoesNotExist)
                                );
                            }
                        }

                        it "calls on_ERC721_received" {
                            token.as_account(token_owner).safe_transfer_from(token_owner, receiver_contract_address, first_token_id).unwrap();

                            receiver_contract.assert_last_event(casper_dao_erc721::Received {
                                operator,
                                from: token_owner,
                                token_id: first_token_id,
                                data: None
                            });
                        }

                        it "calls on_ERC721_received from approved" {
                            token.as_account(approved).safe_transfer_from(token_owner, receiver_contract_address, first_token_id).unwrap();

                            receiver_contract.assert_last_event(casper_dao_erc721::Received {
                                operator,
                                from: token_owner,
                                token_id: first_token_id,
                                data: None
                            });
                        }

                        context "without data" {

                        }

                        context "with data" {

                        }
                    }
                }
            }
        }
    }
}
// it('calls onERC721Received', async function () {
//     const receipt = await transferFun.call(this, owner, this.receiver.address, tokenId, { from: owner });

//     await expectEvent.inTransaction(receipt.tx, ERC721ReceiverMock, 'Received', {
//       operator: owner,
//       from: owner,
//       tokenId: tokenId,
//       data: data,
//     });
//   });

//   it('calls onERC721Received from approved', async function () {
//     const receipt = await transferFun.call(this, owner, this.receiver.address, tokenId, { from: approved });

//     await expectEvent.inTransaction(receipt.tx, ERC721ReceiverMock, 'Received', {
//       operator: approved,
//       from: owner,
//       tokenId: tokenId,
//       data: data,
//     });
//   });

//   describe('with an invalid token id', function () {
//     it('reverts', async function () {
//       await expectRevert(
//         transferFun.call(
//           this,
//           owner,
//           this.receiver.address,
//           nonExistentTokenId,
//           { from: owner },
//         ),
//         'ERC721: operator query for nonexistent token',
//       );
//     });
//   });
// });
// };
