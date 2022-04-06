mod governance_voting_common;
extern crate speculate;

use speculate::speculate;
use std::time::Duration;

use casper_dao_contracts::{
    voting::{
        consts as gv_consts, voting::Voting, Vote, VoteCast, VotingContractCreated, VotingCreated,
        VotingEnded, VotingId,
    },
    MockVoterContractTest, ReputationContractTest, VariableRepositoryContractTest,
};

use casper_dao_utils::{consts, Address, Error, TestEnv};
use casper_types::{RuntimeArgs,runtime_args,
    bytesrepr::{Bytes, ToBytes},
    U256,
};

speculate! {
    context "governance_voting" {
        before {
            let informal_quorum = 500.into();
            let formal_quorum = 750.into();
            let minimum_reputation = 500.into();
            let reputation_to_mint = 10000;
            let informal_voting_time: u64= 3600;
            let formal_voting_time: u64 = 2*3600;
            let env = TestEnv::new();
            let mut variable_repo_contract = governance_voting_common::get_variable_repo_contract(&env, informal_quorum, formal_quorum, informal_voting_time, formal_voting_time, minimum_reputation);
            let mut reputation_token_contract = governance_voting_common::get_reputation_token_contract(&env, reputation_to_mint);
            
            let mut mock_voter_contract = MockVoterContractTest::new(
                &env,
                Address::from(variable_repo_contract.get_package_hash()),
                Address::from(reputation_token_contract.get_package_hash()),
            );

            variable_repo_contract
                .add_to_whitelist(mock_voter_contract.address())
                .unwrap();

            reputation_token_contract
                .add_to_whitelist(mock_voter_contract.address())
                .unwrap();
        }

        test "that mock voter has been set up correctly" {
            assert_eq!(mock_voter_contract.get_reputation_token_address(), reputation_token_contract.address());
            assert_eq!(mock_voter_contract.get_variable_repo_address(), variable_repo_contract.address());

            mock_voter_contract.assert_last_event(
                VotingContractCreated {
                    variable_repo: Address::from(variable_repo_contract.get_package_hash()),
                    reputation_token: Address::from(reputation_token_contract.get_package_hash()),
                    voter_contract: Address::from(mock_voter_contract.get_package_hash()),
                },
            );
        }

        context "informal_voting" {
            #[should_panic]
            test "creating voting is impossible without enough reputation" {
                mock_voter_contract
                    .create_voting("some_value".to_string(), minimum_reputation.saturating_sub(100.into()))
                    .unwrap();
            }
    
            #[should_panic]
            test "creating voting is impossible with too much reputation" {
                mock_voter_contract
                    .create_voting("some_value".to_string(), U256::from(reputation_to_mint).saturating_add(U256::one()))
                    .unwrap();
            }
    
            test "creating voting is possible with enough reputation" {
                mock_voter_contract
                    .create_voting("some_value".to_string(), minimum_reputation)
                    .unwrap();
    
                mock_voter_contract.assert_event_at(-2, VotingCreated {
                    creator: env.get_account(0),
                    voting_id: VotingId::zero(),
                    stake: minimum_reputation,
                });
            }
    
            test "voting is created correctly" {
                mock_voter_contract
                    .create_voting("some_value".to_string(), minimum_reputation)
                    .unwrap();
    
                let event : VotingCreated = mock_voter_contract.event(1);
                let voting_id = event.voting_id;
    
                let voting = mock_voter_contract.get_voting(voting_id);
                assert_eq!(voting.voting_id(), voting_id);
                assert_eq!(voting.voting_id(), VotingId::zero());
                assert_eq!(voting.formal_voting_time(), formal_voting_time);
                assert_eq!(voting.informal_voting_time(), informal_voting_time);
                assert_eq!(voting.formal_voting_quorum(), casper_dao_utils::math::promils_of(reputation_token_contract.total_onboarded(), formal_quorum).unwrap());
                assert_eq!(voting.informal_voting_quorum(), casper_dao_utils::math::promils_of(reputation_token_contract.total_onboarded(), informal_quorum).unwrap());
                assert_eq!(event.voting_id, voting.voting_id());
                assert_eq!(event.creator, env.get_account(0));
                assert_eq!(event.stake, minimum_reputation);
            }

            context "informal_voting_created" {
                before {
                    mock_voter_contract
                    .create_voting("some_value".to_string(), minimum_reputation)
                    .unwrap();

                    let vote_cast_event: VoteCast = mock_voter_contract.event(2);
                    let informal_voting: Voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);
                }

                test "first vote is cast automatically" {
                    let first_vote: Vote = mock_voter_contract.get_vote(informal_voting.voting_id(), env.get_account(0));
                    assert_eq!(first_vote.voting_id, informal_voting.voting_id());
                    assert_eq!(first_vote.voter, Some(env.get_account(0)));
                    assert_eq!(first_vote.choice, true);
                    assert_eq!(first_vote.stake, minimum_reputation);
                }

                test "first vote marked as created by a caller" {
                    assert_eq!(mock_voter_contract.get_voter(informal_voting.voting_id(), 0), env.get_account(0));
                }

                #[should_panic]
                test "only one vote is cast" {
                    mock_voter_contract.get_voter(informal_voting.voting_id(), 1);
                }

                test "if reputation was staked correctly" {
                    assert_eq!(
                        reputation_token_contract.balance_of(mock_voter_contract.address()),
                        minimum_reputation
                    );
                    assert_eq!(
                        reputation_token_contract.balance_of(env.get_account(0)),
                        U256::from(reputation_to_mint).saturating_sub(minimum_reputation)
                    );
                    assert_eq!(
                        reputation_token_contract.balance_of(env.get_account(1)),
                        U256::from(reputation_to_mint)
                    );
                }

                test "voting counter" {
                    mock_voter_contract
                        .create_voting("some_other_value".to_string(), minimum_reputation)
                        .unwrap();
                    let vote_cast_event: VoteCast = mock_voter_contract.event(4);
                    let voting: Voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);
                    assert_eq!(voting.voting_id(), VotingId::from(1));
                }

                test "finishing voting before its end" {
                    env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() - 1));
                    let result = mock_voter_contract.finish_voting(informal_voting.voting_id());
                    assert_eq!(result.unwrap_err(), Error::InformalVotingTimeNotReached);
                }

                #[should_panic]
                test "voting twice for the same voting" {
                    mock_voter_contract.vote(informal_voting.voting_id(), true, minimum_reputation).unwrap();
                }

                test "voting completing in favor with exact and equal votes (a close tie)" {
                    mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), false, minimum_reputation).unwrap();
                    env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                    mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();

                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: informal_voting.voting_id(),
                        result: gv_consts::INFORMAL_VOTING_PASSED.into(),
                        votes_count: U256::from(2),
                        stake_in_favor: minimum_reputation,
                        stake_against: minimum_reputation,
                        informal_voting_id: informal_voting.voting_id(),
                        formal_voting_id: Some(informal_voting.voting_id().saturating_add(U256::one())),
                    });
                }

                test "voting rejected with almost equal votes" {
                    mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), false, minimum_reputation.saturating_add(minimum_reputation)).unwrap();
                    mock_voter_contract.as_account(env.get_account(2)).vote(informal_voting.voting_id(), true, minimum_reputation.saturating_sub(U256::one())).unwrap();
                    env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                    mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();

                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: informal_voting.voting_id(),
                        result: gv_consts::INFORMAL_VOTING_REJECTED.into(),
                        votes_count: U256::from(3),
                        stake_in_favor: minimum_reputation.saturating_add(minimum_reputation).saturating_sub(U256::one()),
                        stake_against: minimum_reputation.saturating_add(minimum_reputation),
                        informal_voting_id: informal_voting.voting_id(),
                        formal_voting_id: None,
                    });
                }

                test "voting completing with everybody voting in favor" {
                    mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), true, minimum_reputation).unwrap();
                    mock_voter_contract.as_account(env.get_account(2)).vote(informal_voting.voting_id(), true, minimum_reputation).unwrap();
                    mock_voter_contract.as_account(env.get_account(3)).vote(informal_voting.voting_id(), true, minimum_reputation).unwrap();
                    env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                    mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();

                    mock_voter_contract.assert_last_event(VotingEnded {
                        voting_id: informal_voting.voting_id(),
                        result: gv_consts::INFORMAL_VOTING_PASSED.into(),
                        votes_count: U256::from(4),
                        stake_in_favor: minimum_reputation.saturating_mul(U256::from(4)),
                        stake_against: U256::zero(),
                        informal_voting_id: informal_voting.voting_id(),
                        formal_voting_id: Some(informal_voting.voting_id().saturating_add(U256::one())),
                    });
                }

                context "informal_voting_ended_without_quorum" {
                    before {
                        env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                        mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();
                    }
        
        
                    test "voting ended event is emitted" {
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
        
                    test "voting is completed" {
                        let informal_voting = mock_voter_contract.get_voting(informal_voting.voting_id());
        
                        assert_eq!((informal_voting.completed()), true);
                    }
        
                    #[should_panic]
                    test "ended voting should not accept votes" {
                        mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), true, minimum_reputation).unwrap();
                    }
        
                    #[should_panic]
                    test "ended voting cannot be finished again" {
                        mock_voter_contract.as_account(env.get_account(2)).finish_voting(informal_voting.voting_id()).unwrap();
                    }
                }

                context "informal_voting_rejected" {
                    before {
                        mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), false, minimum_reputation.saturating_add(minimum_reputation)).unwrap();
                        env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                        mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();
                    }
        
                    #[should_panic]
                    test "no new voting was created" {
                        mock_voter_contract.get_voting(informal_voting.voting_id() + 1);
                    }
                }

                context "informal_voting_completed" {}
            }
        }

        
        
        



        

        context "formal_voting_created" {
            before {
                mock_voter_contract
                .create_voting("some_value".to_string(), minimum_reputation)
                .unwrap();

                let vote_cast_event: VoteCast = mock_voter_contract.event(2);
                let informal_voting: Voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);

                mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), false, minimum_reputation).unwrap();
                env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();

                let voting_created_event: VotingCreated = mock_voter_contract.event(4);
                let vote_cast_event: VoteCast = mock_voter_contract.event(5);
                let formal_voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);
                let first_voter = mock_voter_contract.get_voter(formal_voting.voting_id(), 0);
                let first_vote = mock_voter_contract.get_vote(formal_voting.voting_id(), first_voter);
                let informal_voting: Voting = mock_voter_contract.get_voting(informal_voting.voting_id());
            }

            test "formal voting values" {
                let runtime_args = runtime_args! {
                    "variable" => "some_value".to_string(),
                };

                assert_eq!(voting_created_event.voting_id, VotingId::from(1));
                assert_eq!(voting_created_event.creator, env.get_account(0));
                assert_eq!(voting_created_event.stake, minimum_reputation);

                assert_eq!(formal_voting.voting_id(), voting_created_event.voting_id);
                assert_eq!(formal_voting.completed(), false);
                assert_eq!(formal_voting.stake_in_favor(), minimum_reputation);
                assert_eq!(formal_voting.stake_against(), U256::zero());
                assert_eq!(formal_voting.informal_voting_id(), informal_voting.voting_id());
                assert_eq!(formal_voting.formal_voting_time(), formal_voting_time);
                assert_eq!(formal_voting.informal_voting_time(), informal_voting_time);
                assert_eq!(formal_voting.formal_voting_quorum(), casper_dao_utils::math::promils_of(reputation_token_contract.total_onboarded(), formal_quorum).unwrap());
                assert_eq!(formal_voting.informal_voting_quorum(), casper_dao_utils::math::promils_of(reputation_token_contract.total_onboarded(), informal_quorum).unwrap());
                assert_eq!(formal_voting.minimum_governance_reputation(), minimum_reputation);
                assert_eq!(formal_voting.contract_to_call(), Some(mock_voter_contract.address()));
                assert_eq!(formal_voting.entry_point(), "set_variable");
                assert_eq!(formal_voting.runtime_args(), &runtime_args);
            }

            test "first vote cast" {
                assert_eq!(first_vote.choice, true);
                assert_eq!(first_vote.stake, minimum_reputation);
                assert_eq!(first_vote.voter, Some(first_voter));
                assert_eq!(first_vote.voting_id, formal_voting.voting_id());
            }

            #[should_panic]
            test "only one voter after creation" {
                mock_voter_contract.get_voter(formal_voting.voting_id(), 1);
            }

            test "informal voting updated" {
                assert_eq!(informal_voting.completed(), true);
                assert_eq!(informal_voting.formal_voting_id(), Some(formal_voting.voting_id()));
            }

            test "finishing voting before its end" {
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() - 1));
                let result = mock_voter_contract.finish_voting(formal_voting.voting_id());
                assert_eq!(result.unwrap_err(), Error::FormalVotingTimeNotReached);
            }

            #[should_panic]
            test "voting twice for the same voting" {
                mock_voter_contract.vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
            }

            test "quorum not reached" {
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();

                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_QUORUM_NOT_REACHED.into(),
                    votes_count: U256::one(),
                    stake_in_favor: minimum_reputation,
                    stake_against: U256::zero(),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }

            test "quorum not reached by a bit" {
                mock_voter_contract.as_account(env.get_account(1)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                
                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_QUORUM_NOT_REACHED.into(),
                    votes_count: U256::from(2),
                    stake_in_favor: minimum_reputation.saturating_add(minimum_reputation),
                    stake_against: U256::zero(),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }

            test "quorum reached and voting passed" {
                mock_voter_contract.as_account(env.get_account(1)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
                mock_voter_contract.as_account(env.get_account(2)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                
                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_PASSED.into(),
                    votes_count: U256::from(3),
                    stake_in_favor: minimum_reputation.saturating_mul(U256::from(3)),
                    stake_against: U256::zero(),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }

            test "quorum reached and voting passed with a tie" {
                mock_voter_contract.as_account(env.get_account(1)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
                mock_voter_contract.as_account(env.get_account(2)).vote(formal_voting.voting_id(), false, minimum_reputation.saturating_add(minimum_reputation)).unwrap();
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                
                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_PASSED.into(),
                    votes_count: U256::from(3),
                    stake_in_favor:  minimum_reputation.saturating_add(minimum_reputation),
                    stake_against:  minimum_reputation.saturating_add(minimum_reputation),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }

            test "quorum reached and voting rejected" {
                mock_voter_contract.as_account(env.get_account(1)).vote(formal_voting.voting_id(), false, minimum_reputation).unwrap();
                mock_voter_contract.as_account(env.get_account(2)).vote(formal_voting.voting_id(), false, minimum_reputation).unwrap();
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                
                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_REJECTED.into(),
                    votes_count: U256::from(3),
                    stake_in_favor: minimum_reputation,
                    stake_against: minimum_reputation.saturating_add(minimum_reputation),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }

            test "quorum reached and voting rejected - off by one" {
                mock_voter_contract.as_account(env.get_account(1)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
                mock_voter_contract.as_account(env.get_account(2)).vote(formal_voting.voting_id(), false, minimum_reputation.saturating_add(minimum_reputation).saturating_add(U256::one())).unwrap();
                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
                
                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_REJECTED.into(),
                    votes_count: U256::from(3),
                    stake_in_favor: minimum_reputation.saturating_add(minimum_reputation),
                    stake_against: minimum_reputation.saturating_add(minimum_reputation).saturating_add(U256::one()),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }
        }

        context "formal_voting_ended_without_quorum" {}

        context "formal_voting_rejected" {}

        context "formal_voting_completed" {
            before {
                mock_voter_contract
                .create_voting("some_value".to_string(), minimum_reputation)
                .unwrap();

                let vote_cast_event: VoteCast = mock_voter_contract.event(2);
                let informal_voting: Voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);

                mock_voter_contract.as_account(env.get_account(1)).vote(informal_voting.voting_id(), false, minimum_reputation).unwrap();
                env.advance_block_time_by(Duration::from_secs(informal_voting.informal_voting_time() + 1));
                mock_voter_contract.finish_voting(informal_voting.voting_id()).unwrap();

                let vote_cast_event: VoteCast = mock_voter_contract.event(5);
                let formal_voting = mock_voter_contract.get_voting(vote_cast_event.voting_id);

                mock_voter_contract.as_account(env.get_account(1)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();
                mock_voter_contract.as_account(env.get_account(2)).vote(formal_voting.voting_id(), true, minimum_reputation).unwrap();

                env.advance_block_time_by(Duration::from_secs(formal_voting.formal_voting_time() + 1));
                mock_voter_contract.finish_voting(formal_voting.voting_id()).unwrap();
            }

            test "voting ended event" {
                mock_voter_contract.assert_last_event(VotingEnded {
                    voting_id: formal_voting.voting_id(),
                    result: gv_consts::FORMAL_VOTING_PASSED.into(),
                    votes_count: U256::from(3),
                    stake_in_favor: minimum_reputation.saturating_mul(U256::from(3)),
                    stake_against: U256::zero(),
                    informal_voting_id: informal_voting.voting_id(),
                    formal_voting_id: Some(formal_voting.voting_id()),
                });
            }

            test "action was was performed" {
                let variable = mock_voter_contract.get_variable();
                assert_eq!(variable, "some_value");
            }
        }
    }
}
