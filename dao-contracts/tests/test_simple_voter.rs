mod common;

use speculate::speculate;

use casper_dao_contracts::voting::Choice;

use casper_dao_utils::DocumentHash;
use casper_dao_utils::TestContract;
use casper_types::U256;

speculate! {
    context "simple_voter" {
        before {
            let mut simple_voter_contract = common::setup::setup_simple_voter();
        }

        context "with informal voting in progress" {
            before {
                simple_voter_contract.as_nth_account(0).create_voting(DocumentHash::from(vec![123]), U256::from(500)).unwrap();
                let voting_id = 0;
                #[allow(unused_variables)]
                let voting = simple_voter_contract.get_voting(voting_id).unwrap();
            }

            test "document hash is saved in the contract" {
                let document_hash = simple_voter_contract.get_document_hash(voting_id);
                assert_eq!(document_hash, Some(DocumentHash::from(vec![123])));
            }

            context "when informal voting fails" {
                before {
                    simple_voter_contract
                        .as_nth_account(1)
                        .vote(voting.voting_id(), Choice::Against, 1000.into())
                        .unwrap();

                    simple_voter_contract
                        .as_nth_account(2)
                        .vote(voting.voting_id(), Choice::Against, 1000.into())
                        .unwrap();

                    simple_voter_contract
                        .advance_block_time_by(voting.informal_voting_time() + 1);

                    simple_voter_contract
                        .as_nth_account(2)
                        .finish_voting(voting.voting_id())
                        .unwrap();
                }

                test "there is no new voting" {
                    let formal_voting_id = 1;
                    let document_hash = simple_voter_contract.get_document_hash(formal_voting_id);
                    assert_eq!(document_hash, None);
                }
            }

            context "when informal voting succeeds" {
                before {
                    simple_voter_contract
                        .as_nth_account(1)
                        .vote(voting.voting_id(), Choice::InFavor, 1000.into())
                        .unwrap();

                    simple_voter_contract
                        .as_nth_account(2)
                        .vote(voting.voting_id(), Choice::InFavor, 1000.into())
                        .unwrap();

                    simple_voter_contract
                        .advance_block_time_by(voting.informal_voting_time() + 1);

                    simple_voter_contract
                        .as_nth_account(2)
                        .finish_voting(voting.voting_id())
                        .unwrap();

                    let formal_voting_id = 1;
                }

                test "there is a new voting with the same document hash" {
                    let document_hash = simple_voter_contract.get_document_hash(formal_voting_id);
                    assert_eq!(document_hash, Some(DocumentHash::from(vec![123])));
                }
            }
        }
    }
}
