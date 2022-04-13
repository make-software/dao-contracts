mod governance_voting_common;
extern crate speculate;
use speculate::speculate;
use std::time::Duration;

use casper_dao_contracts::{
    action::Action,
    voting::{voting::Voting, VotingContractCreated, VotingId},
    AdminContractTest,
};

use casper_dao_utils::{Address, TestEnv};
use casper_types::U256;

// TODO: Remove speculate 
speculate! {
    context "admin" {
        before {
            let informal_quorum = 500.into();
            let formal_quorum = 750.into();
            let minimum_reputation = 500.into();
            let reputation_to_mint = 10_000;
            let informal_voting_time: u64 = 3_600;
            let formal_voting_time: u64 = 2 * informal_voting_time;
            let env = TestEnv::new();
            let mut variable_repo_contract = governance_voting_common::setup_variable_repo_contract(&env, informal_quorum, formal_quorum, informal_voting_time, formal_voting_time, minimum_reputation);
            let mut reputation_token_contract = governance_voting_common::setup_reputation_token_contract(&env, reputation_to_mint);

            #[allow(unused_variables)]
            let deployer = env.get_account(0);
            #[allow(unused_variables)]
            let account1 = env.get_account(1);
            #[allow(unused_variables)]
            let account2 = env.get_account(2);
            #[allow(unused_variables)]
            let account3 = env.get_account(3);

            #[allow(unused_mut)]
            let mut admin_contract = AdminContractTest::new(
                &env,
                variable_repo_contract.address(),
                reputation_token_contract.address(),
            );

            variable_repo_contract
                .as_account(deployer)
                .change_ownership(admin_contract.address())
                .unwrap();
            
            reputation_token_contract
                .as_account(deployer)
                .change_ownership(admin_contract.address())
                .unwrap();
        }

        test "that repo voter has been set up correctly" {
            assert_eq!(admin_contract.get_reputation_token_address(), reputation_token_contract.address());
            assert_eq!(admin_contract.get_variable_repo_address(), variable_repo_contract.address());

            admin_contract.assert_last_event(
                VotingContractCreated {
                    variable_repo: variable_repo_contract.address(),
                    reputation_token: reputation_token_contract.address(),
                    voter_contract: admin_contract.address(),
                },
            );
        }

        context "voting" {
            before {
                admin_contract
                    .create_voting(
                        reputation_token_contract.address(),
                        Action::AddToWhitelist,
                        account1,
                        minimum_reputation,
                    )
                    .unwrap();

                let informal_voting_id = VotingId::zero();
                let voting: Voting = admin_contract.get_voting(informal_voting_id);

                // cast votes for informal voting
                // TODO: Enum Choice::Against,For
                // TODO: Remove magic numbers.
                admin_contract
                    .as_account(account1)
                    .vote(informal_voting_id, true, U256::from(500))
                    .unwrap();

                // fast forward
                env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

                // finish informal voting
                admin_contract
                    .as_account(account1)
                    .finish_voting(informal_voting_id)
                    .unwrap();

                let formal_voting_id = VotingId::one();

                // cast votes for formal voting
                admin_contract
                    .as_account(account1)
                    .vote(formal_voting_id, true, 1000.into())
                    .unwrap();
                admin_contract
                    .as_account(account2)
                    .vote(formal_voting_id, true, 1000.into())
                    .unwrap();
            }

            test "action was performed after finish" {
                env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                admin_contract.finish_voting(formal_voting_id).unwrap();

                assert!(reputation_token_contract.is_whitelisted(account1));
            }
        }
    }
}
