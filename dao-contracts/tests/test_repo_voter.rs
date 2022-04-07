mod governance_voting_common;
extern crate speculate;
use speculate::speculate;
use std::time::Duration;

use casper_dao_contracts::{
    voting::{voting::Voting, VotingContractCreated, VotingId},
    RepoVoterContractTest,
};

use casper_dao_utils::{Address, TestEnv};
use casper_types::{
    bytesrepr::{Bytes, FromBytes, ToBytes},
    U256,
};

speculate! {
    context "repo_voter" {
        before {
            let informal_quorum = 500.into();
            let formal_quorum = 750.into();
            let minimum_reputation = 500.into();
            let reputation_to_mint = 10000;
            let informal_voting_time: u64 = 3600;
            let formal_voting_time: u64 = 2*3600;
            let env = TestEnv::new();
            let mut variable_repo_contract = governance_voting_common::get_variable_repo_contract(&env, informal_quorum, formal_quorum, informal_voting_time, formal_voting_time, minimum_reputation);
            let mut reputation_token_contract = governance_voting_common::get_reputation_token_contract(&env, reputation_to_mint);

            #[allow(unused_mut)]
            let mut repo_voter_contract = RepoVoterContractTest::new(
                &env,
                Address::from(variable_repo_contract.get_package_hash()),
                Address::from(reputation_token_contract.get_package_hash()),
            );

            variable_repo_contract
                .add_to_whitelist(repo_voter_contract.address())
                .unwrap();

            reputation_token_contract
                .add_to_whitelist(repo_voter_contract.address())
                .unwrap();
        }

        test "that repo voter has been set up correctly" {
            assert_eq!(repo_voter_contract.get_reputation_token_address(), reputation_token_contract.address());
            assert_eq!(repo_voter_contract.get_variable_repo_address(), variable_repo_contract.address());

            repo_voter_contract.assert_last_event(
                VotingContractCreated {
                    variable_repo: Address::from(variable_repo_contract.get_package_hash()),
                    reputation_token: Address::from(reputation_token_contract.get_package_hash()),
                    voter_contract: Address::from(repo_voter_contract.get_package_hash()),
                },
            );
        }

        context "voting" {
            before {
                repo_voter_contract
                .create_voting(
                    variable_repo_contract.address(),
                    "variable_name".into(),
                    Bytes::from("new_value".to_string().to_bytes().unwrap()),
                    None,
                    minimum_reputation,
                )
                .unwrap();

                let voting_id = VotingId::zero();
                let voting: Voting = repo_voter_contract.get_voting(voting_id);

                // cast votes for informal voting
                repo_voter_contract
                    .as_account(env.get_account(1))
                    .vote(voting_id, true, U256::from(500))
                    .unwrap();

                // fast forward
                env.advance_block_time_by(Duration::from_secs(voting.informal_voting_time() + 1));

                // finish informal voting
                repo_voter_contract
                    .as_account(env.get_account(1))
                    .finish_voting(voting_id)
                    .unwrap();

                let voting_id = VotingId::one();

                // cast votes for formal voting
                repo_voter_contract
                    .as_account(env.get_account(1))
                    .vote(voting_id, true, 1000.into())
                    .unwrap();
                repo_voter_contract
                    .as_account(env.get_account(2))
                    .vote(voting_id, true, 1000.into())
                    .unwrap();
            }

            #[should_panic]
            test "action was not performed before finish" {
                variable_repo_contract.get("variable_name".into()).unwrap();
            }

            #[should_panic]
            test "action was not performed on rejected voting" {
                // vote against
                repo_voter_contract
                    .as_account(env.get_account(3))
                    .vote(voting_id, false, 5000.into())
                    .unwrap();

                env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                repo_voter_contract.finish_voting(voting_id).unwrap();
                variable_repo_contract.get("variable_name".into()).unwrap();
            }

            test "action was performed after finish" {
                env.advance_block_time_by(Duration::from_secs(voting.formal_voting_time() + 1));
                repo_voter_contract.finish_voting(voting_id).unwrap();
                let bytes = variable_repo_contract.get("variable_name".into()).unwrap();
                let (variable, bytes) = String::from_bytes(&bytes).unwrap();
                assert_eq!(bytes.len(), 0);
                assert_eq!(variable, "new_value");
            }
        }
    }
}
