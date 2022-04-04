extern crate speculate;
use speculate::speculate;
use std::borrow::BorrowMut;

use casper_dao_erc721::{
    events::{Approval, ApprovalForAll, Transfer},
    ERC721Test, MockERC721NonReceiverTest, MockERC721ReceiverTest,
};
use casper_dao_utils::{Address, Error, TestEnv};
use casper_types::{bytesrepr::Bytes, U256};

speculate! {
    static NAME: &str = "Plascoin";
    static SYMBOL: &str = "PLS";

    static TOKEN_ID_1: u32 = 1;
    static TOKEN_ID_2: u32 = 2;
    static UNKNOWN_TOKEN_ID: u32 = 999;

    fn _mint_tokens(env: &TestEnv, erc721: &mut ERC721Test) {
        erc721.mint(env.get_account(1), TOKEN_ID_1.into()).unwrap();
        erc721.mint(env.get_account(1), TOKEN_ID_2.into()).unwrap();

        assert_eq!(erc721.total_supply(), 2.into());
        assert_eq!(erc721.balance_of(env.get_account(1)), 2.into());
    }

    context "erc721" {

        before {
            let env = TestEnv::new();
            let mut token = ERC721Test::new(&env, String::from(NAME), String::from(SYMBOL));
            let receiver = MockERC721ReceiverTest::new(&env);
            let non_receiver = MockERC721NonReceiverTest::new(&env);

            let first_token_id: casper_dao_erc721::TokenId = 1.into();
            let second_token_id: casper_dao_erc721::TokenId = 2.into();
            let operator = env.get_account(0);
            let token_owner = env.get_account(1);
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

        context "with minted tokens" {

            before {
                _mint_tokens(&env, token.borrow_mut());
            }

            context "balance of" {
                it "returns the amount of tokens owned by the given address" {
                    assert_eq!(token.balance_of(token_owner), 2.into());
                }
            }

            test "sa" {
                // When approves the token's owner
                let result = token.approve(Some(token_owner), first_token_id);

                // Then raises an error
                assert_eq!(result, Err(Error::ApprovalToCurrentOwner));

                // When the caller is not the owner and approved for all
                let result = token.approve(Some(operator), first_token_id);

                // Then raises an error
                assert_eq!(result, Err(Error::ApproveCallerIsNotOwnerNorApprovedForAll));

                // When the owner approves a different address
                token
                    .as_account(token_owner)
                    .approve(Some(operator), first_token_id)
                    .unwrap();

                // Then the given address should be approved
                assert_eq!(token.get_approved(first_token_id).unwrap(), env.get_account(0));

                // Then an Approval event is emitted
                token.assert_event_at(
                    2,
                    Approval {
                        owner: Some(token_owner),
                        operator: Some(env.get_account(0)),
                        token_id: first_token_id,
                    },
                );
            }
        }
    }
}
