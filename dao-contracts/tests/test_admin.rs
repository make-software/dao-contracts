mod governance_voting_common;

use speculate::speculate;

use casper_dao_contracts::voting::{Choice, VotingId};

use casper_dao_utils::Error;
use casper_types::U256;

speculate! {
    describe "admin with voting set up for adding an address to a whitelist" {
        before {
            #[allow(unused_mut)]
            let (mut admin_contract, mut reputation_token_contract) = governance_voting_common::setup_admin();
            #[allow(unused_variables)]
            let voting = admin_contract.get_voting(VotingId::from(1)).unwrap();
        }

        test "address cannot perform action before voting finishes" {
            assert_eq!(
                reputation_token_contract.as_nth_account(1).mint(admin_contract.get_env().get_account(1), U256::from(500)),
                Err(Error::NotWhitelisted)
            );
        }

        describe "when voting is rejected" {
            before {
                admin_contract
                .as_nth_account(2)
                .vote(voting.voting_id(), Choice::Against, 5000.into())
                .unwrap();

                admin_contract.advance_block_time_by(voting.formal_voting_time() + 1);
                admin_contract.finish_voting(voting.voting_id()).unwrap();
            }

            test "address cannot perform action on rejected voting" {
                assert_eq!(
                    reputation_token_contract.as_nth_account(1).mint(admin_contract.get_env().get_account(1), U256::from(500)),
                    Err(Error::NotWhitelisted)
                );
            }
        }

        describe "when voting passes" {
            before {
                admin_contract
                    .as_nth_account(2)
                    .vote(voting.voting_id(), Choice::InFavor, 5000.into())
                    .unwrap();
                admin_contract.advance_block_time_by(voting.formal_voting_time() + 1);
                admin_contract.finish_voting(voting.voting_id()).unwrap();
            }

            test "address cann perform action" {
                reputation_token_contract.as_nth_account(1).mint(admin_contract.get_env().get_account(1), U256::from(500)).unwrap();
                assert_eq!(
                    reputation_token_contract.balance_of(admin_contract.get_env().get_account(1)),
                    U256::from(10500)
                );
            }
        }
    }
}
