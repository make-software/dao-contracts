mod governance_voting_common;

use speculate::speculate;

use casper_dao_contracts::{voting::Choice, voting::VotingId};

use casper_dao_utils::BytesConversion;
use casper_types::{bytesrepr::FromBytes, U256};

speculate! {
    context "repo_voter" {
        before {
            let (mut repo_voter_contract, variable_repo_contract) = governance_voting_common::setup_repo_voter("variable_name".into(), U256::from(123).convert_to_bytes().unwrap());
            let voting_id = VotingId::one();
            let voting = repo_voter_contract.get_voting(voting_id).unwrap();
            repo_voter_contract
                .as_nth_account(1)
                .vote(voting.voting_id(), Choice::InFavor, 1000.into())
                .unwrap();
        }

        test "action was not performed before finish" {
            assert_eq!(
                variable_repo_contract.get("variable_name".into()),
                None
            );
        }

        test "action was not performed on rejected voting" {
            repo_voter_contract
                .as_nth_account(2)
                .vote(voting.voting_id(), Choice::Against, 5000.into())
                .unwrap();

            repo_voter_contract.advance_block_time_by(voting.formal_voting_time() + 1);
            repo_voter_contract.finish_voting(voting.voting_id()).unwrap();

            assert_eq!(
                variable_repo_contract.get("variable_name".into()),
                None
            );
        }

        test "action was performed after finish" {
            repo_voter_contract.advance_block_time_by(voting.formal_voting_time() + 1);
            repo_voter_contract.finish_voting(voting.voting_id()).unwrap();
            let bytes = variable_repo_contract.get("variable_name".into()).unwrap();
            let (variable, bytes) = U256::from_bytes(&bytes).unwrap();

            assert_eq!(bytes.len(), 0);
            assert_eq!(variable, U256::from(123));
        }
    }
}
