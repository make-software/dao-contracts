mod governance_voting_common;

use casper_dao_contracts::voting::{
    consts as gv_consts, Ballot, Choice, VoteCast, VotingContractCreated, VotingCreated,
    VotingEnded, VotingId,
};
use casper_dao_utils::Error;
use casper_dao_utils::TestContract;
use casper_types::U256;
use speculate::speculate;

speculate! {
    context "governance voting" {
        before {
            let informal_quorum = U256::from(500);
            let formal_quorum = U256::from(750);
            let total_onboarded = 4;
            #[allow(unused_variables)]
            let minimum_reputation = U256::from(500);
            #[allow(unused_variables)]
            let minted_reputation = 10_000;
            let informal_voting_time: u64 = 3_600;
            let formal_voting_time: u64 = 2 * informal_voting_time;
            #[allow(unused_variables)]
            let after_informal_voting_time = informal_voting_time + 1;
            #[allow(unused_variables)]
            let after_formal_voting_time = formal_voting_time + 1;
        }

        describe "voting contact" {
            before {
                #[allow(unused_mut, unused_variables)]
                let (mut mock_voter_contract, variable_repo_contract, reputation_token_contract) = governance_voting_common::setup_voting_contract(informal_quorum, formal_quorum, total_onboarded);
            }

            it "emits event with correct values" {
                assert_eq!(mock_voter_contract.get_reputation_token_address(), reputation_token_contract.address());
                assert_eq!(mock_voter_contract.get_variable_repo_address(), variable_repo_contract.address());

                mock_voter_contract.assert_last_event(
                    VotingContractCreated {
                        variable_repo: variable_repo_contract.address(),
                        reputation_token: reputation_token_contract.address(),
                        voter_contract: mock_voter_contract.address(),
                    },
                );
            }

            it "disallows creating voting with not enough reputation staked" {
                assert_eq!(
                mock_voter_contract
                    .create_voting("some_value".to_string(), minimum_reputation - U256::one()),
                    Err(Error::NotEnoughReputation)
                );
            }

            it "disallows creating voting with reputation that creator doesn't have" {
                assert_eq!(
                mock_voter_contract
                    .create_voting("some_value".to_string(), U256::from(minted_reputation) + U256::one()),
                    Err(Error::InsufficientBalance)
                );
            }

            it "can count votings" {
                mock_voter_contract
                    .create_voting("some_value".to_string(), minimum_reputation)
                    .unwrap(); // id = 0
                mock_voter_contract
                    .create_voting("another_value".to_string(), minimum_reputation)
                    .unwrap(); // id = 1
                mock_voter_contract
                    .create_voting("yet_another_value".to_string(), minimum_reputation)
                    .unwrap(); // id = 2

                let voting_created_event: VotingCreated = mock_voter_contract.event(-2);

                assert_eq!(voting_created_event.voting_id, VotingId::from(2));
            }
        }

        describe "creating informal voting" {
            before {
                #[allow(unused_mut, unused_variables)]
                let (mut mock_voter_contract, reputation_token_contract, informal_voting) = governance_voting_common::setup_voting_contract_with_informal_voting(informal_quorum, formal_quorum, total_onboarded);
                #[allow(unused_variables)]
                let creator = mock_voter_contract.get_env().get_account(0);
            }

            it "emits an event" {
                mock_voter_contract.assert_event_at(-2, VotingCreated {
                    creator,
                    voting_id: VotingId::zero(),
                    stake: minimum_reputation,
                });
            }

            test "that voting is created correctly" {
                let voting_created_event : VotingCreated = mock_voter_contract.event(-2);
                let ballot_cast_event: VoteCast = mock_voter_contract.event(-1);
                let first_ballot: Ballot = mock_voter_contract.get_ballot(informal_voting.voting_id(), creator).unwrap();

                assert_eq!(informal_voting.voting_id(), VotingId::zero());
                assert_eq!(informal_voting.formal_voting_time(), formal_voting_time);
                assert_eq!(informal_voting.informal_voting_time(), informal_voting_time);
                assert_eq!(informal_voting.formal_voting_quorum(), casper_dao_utils::math::promils_of(U256::from(total_onboarded), formal_quorum).unwrap());
                assert_eq!(informal_voting.informal_voting_quorum(), casper_dao_utils::math::promils_of(U256::from(total_onboarded), informal_quorum).unwrap());
                assert_eq!(voting_created_event.voting_id, informal_voting.voting_id());
                assert_eq!(voting_created_event.creator, creator);
                assert_eq!(voting_created_event.stake, minimum_reputation);

                // first vote is cast automatically
                assert_eq!(first_ballot.voting_id, informal_voting.voting_id());
                assert_eq!(first_ballot.voter, Some(creator));
                assert_eq!(first_ballot.choice, Choice::InFavor);
                assert_eq!(first_ballot.stake, minimum_reputation);
                assert_eq!(ballot_cast_event, VoteCast { voter: creator, voting_id: informal_voting.voting_id(), choice: Choice::InFavor, stake: minimum_reputation });
                assert_eq!(mock_voter_contract.get_voter(informal_voting.voting_id(), 0).unwrap(), creator);

                // only one vote is cast TODO: Check harder
                assert_eq!(mock_voter_contract.get_voter(informal_voting.voting_id(), 1), None);
            }

            test "that creator cannot vote on his own voting" {
                assert_eq!(
                    mock_voter_contract.as_account(creator).vote(informal_voting.voting_id(), casper_dao_contracts::voting::Choice::InFavor, minimum_reputation),
                    Err(Error::CannotVoteTwice)
                );
            }

            test "that someone else cannot vote twice on the same voting" {
                mock_voter_contract.as_nth_account(1).vote(informal_voting.voting_id(), casper_dao_contracts::voting::Choice::InFavor, minimum_reputation).unwrap();
                assert_eq!(
                    mock_voter_contract.as_nth_account(1).vote(informal_voting.voting_id(), casper_dao_contracts::voting::Choice::InFavor, minimum_reputation),
                    Err(Error::CannotVoteTwice)
                );
            }

            describe "when informal voting ends without reaching quorum" {
                before {
                    mock_voter_contract.advance_block_time_by(after_informal_voting_time);
                    mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();
                }

                it "emits proper event" {
                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: informal_voting.voting_id(),
                        result: gv_consts::INFORMAL_VOTING_QUORUM_NOT_REACHED.into(),
                        votes_count: U256::from(1),
                        stake_in_favor: minimum_reputation,
                        stake_against: U256::zero(),
                        informal_voting_id: VotingId::zero(),
                        formal_voting_id: None,
                    });
                }

                it "is completed" {
                    governance_voting_common::assert_voting_completed(&mut mock_voter_contract, informal_voting.voting_id());
                }

                it "doesn't create new voting" {
                    assert_eq!(
                        mock_voter_contract.get_voting(informal_voting.voting_id() + 1),
                        None
                    );
                }
            }

            describe "when informal voting is rejected" {
                before {
                    governance_voting_common::mass_vote(0, 3, &mut mock_voter_contract, &informal_voting);
                    mock_voter_contract.advance_block_time_by(after_informal_voting_time);
                    mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();
                }

                it "emits proper event" {
                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: informal_voting.voting_id(),
                        result: gv_consts::INFORMAL_VOTING_REJECTED.into(),
                        votes_count: U256::from(4),
                        stake_in_favor: minimum_reputation,
                        stake_against: minimum_reputation * 3,
                        informal_voting_id: VotingId::zero(),
                        formal_voting_id: None,
                    });
                }

                it "is completed" {
                    governance_voting_common::assert_voting_completed(&mut mock_voter_contract, informal_voting.voting_id());
                }

                it "doesn't create new voting" {
                    assert_eq!(
                        mock_voter_contract.get_voting(informal_voting.voting_id() + 1),
                        None
                    );
                }
            }

            describe "when informal voting is completed" {
                before {
                    governance_voting_common::mass_vote(3, 1, &mut mock_voter_contract, &informal_voting);
                    mock_voter_contract.advance_block_time_by(after_informal_voting_time);
                    mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();
                }

                it "emits proper event" {
                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: informal_voting.voting_id(),
                        result: gv_consts::INFORMAL_VOTING_PASSED.into(),
                        votes_count: U256::from(4),
                        stake_in_favor: minimum_reputation * 3,
                        stake_against: minimum_reputation,
                        informal_voting_id: informal_voting.voting_id(),
                        formal_voting_id: Some(informal_voting.voting_id() + 1),
                    });
                }

                it "is completed" {
                    governance_voting_common::assert_voting_completed(&mut mock_voter_contract, informal_voting.voting_id());
                }

                it "created new formal voting" {
                    let formal_voting = mock_voter_contract.get_voting(informal_voting.voting_id() + 1).unwrap();
                    assert_eq!(formal_voting.voting_id(), informal_voting.voting_id()+1);
                    assert_eq!(formal_voting.informal_voting_id(), informal_voting.voting_id());
                    assert_eq!(formal_voting.formal_voting_id(), Some(formal_voting.voting_id()));
                    assert!(!formal_voting.completed());
                }
            }

        }

        describe "creating formal voting" {
            before {
                #[allow(unused_mut, unused_variables)]
                let (mut mock_voter_contract, reputation_token_contract, formal_voting) = governance_voting_common::setup_voting_contract_with_formal_voting(informal_quorum, formal_quorum, total_onboarded);
                #[allow(unused_variables)]
                let creator = mock_voter_contract.get_env().get_account(0);
            }

            it "emits proper event" {
                mock_voter_contract.assert_event_at(-3, VotingCreated {
                    creator,
                    voting_id: VotingId::one(),
                    stake: minimum_reputation,
                });
            }

            describe "when formal voting ends without reaching quorum" {
                before {
                    mock_voter_contract.advance_block_time_by(after_formal_voting_time);
                    mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                }

                it "emits proper event" {
                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: formal_voting.voting_id(),
                        result: gv_consts::FORMAL_VOTING_QUORUM_NOT_REACHED.into(),
                        votes_count: U256::from(1),
                        stake_in_favor: minimum_reputation,
                        stake_against: U256::zero(),
                        informal_voting_id: VotingId::zero(),
                        formal_voting_id: Some(VotingId::one()),
                    });
                }

                it "is completed" {
                    governance_voting_common::assert_voting_completed(&mut mock_voter_contract, formal_voting.voting_id());
                }

                it "does not perform its action" {
                    let variable = mock_voter_contract.get_variable();
                    assert_eq!(variable, "");
                }
            }

            describe "when formal voting is rejected" {
                before {
                    governance_voting_common::mass_vote(1, 3, &mut mock_voter_contract, &formal_voting);
                    mock_voter_contract.advance_block_time_by(after_formal_voting_time);
                    mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                }

                it "emits proper event" {
                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: formal_voting.voting_id(),
                        result: gv_consts::FORMAL_VOTING_REJECTED.into(),
                        votes_count: U256::from(4),
                        stake_in_favor: minimum_reputation,
                        stake_against: minimum_reputation * 3,
                        informal_voting_id: VotingId::zero(),
                        formal_voting_id: Some(VotingId::one()),
                    });
                }

                it "is completed" {
                    governance_voting_common::assert_voting_completed(&mut mock_voter_contract, formal_voting.voting_id());
                }

                it "does not perform its action" {
                    let variable = mock_voter_contract.get_variable();
                    assert_eq!(variable, "");
                }
            }

            describe "when formal voting is completed" {
                before {
                    governance_voting_common::mass_vote(3, 1, &mut mock_voter_contract, &formal_voting);
                    mock_voter_contract.advance_block_time_by(after_formal_voting_time);
                    mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                }

                it "emits proper event" {
                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: formal_voting.voting_id(),
                        result: gv_consts::FORMAL_VOTING_PASSED.into(),
                        votes_count: U256::from(4),
                        stake_in_favor: minimum_reputation * 3,
                        stake_against: minimum_reputation,
                        informal_voting_id: VotingId::zero(),
                        formal_voting_id: Some(VotingId::one()),
                    });
                }

                it "is completed" {
                    governance_voting_common::assert_voting_completed(&mut mock_voter_contract, formal_voting.voting_id());
                }

                it "does perform its action" {
                    let variable = mock_voter_contract.get_variable();
                    assert_ne!(variable, "");
                }
            }
        }
    }
}
