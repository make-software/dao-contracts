mod governance_voting_common;
extern crate speculate;
use casper_dao_contracts::bid::{
    events::{JobAccepted, JobCreated, JobSubmitted},
    job::JobStatus,
    types::BidId,
};
use casper_dao_contracts::voting::{Choice, VotingCreated};
use casper_dao_utils::{Error, TestContract};
use casper_types::{U256, U512};
use speculate::speculate;

speculate! {
    describe "bid escrow contract" {
        before {
          #[allow(unused_mut, unused_variables)]
          let (mut bid_escrow_contract, _reputation_token_contract, mut va_token, mut kyc_token) = governance_voting_common::setup_bid_escrow();
          let job_poster = bid_escrow_contract.get_env().get_account(1);
          #[allow(unused_variables)]
          let worker = bid_escrow_contract.get_env().get_account(2);
          #[allow(unused_variables)]
          let anyone = bid_escrow_contract.get_env().get_account(3);
          let job_time : casper_dao_utils::BlockTime = 60;
          let job_description = "Job Description".to_string();
          #[allow(unused_variables)]
          let job_result = "Job result".to_string();
          let cspr_amount = U512::from(100);
          #[allow(unused_variables)]
          let informal_voting_time: u64 = 3_600;
          #[allow(unused_variables)]
          let formal_voting_time: u64 = 2 * informal_voting_time;
          #[allow(unused_variables)]
          let job_poster_initial_cspr = bid_escrow_contract.get_env().get_account_cspr_balance(job_poster);
          #[allow(unused_variables)]
          let worker_initial_cspr = bid_escrow_contract.get_env().get_account_cspr_balance(worker);
        }

        #[should_panic]
        it "cannot create a job for caller" {
            bid_escrow_contract.as_account(job_poster).pick_bid_with_cspr_amount(job_poster, job_description, job_time, None, cspr_amount);
        }

        #[should_panic]
        it "cannot create a job if creator is not kycd" {
            kyc_token.mint(worker, U256::from(1)).unwrap();
            bid_escrow_contract.as_account(job_poster).pick_bid_with_cspr_amount(worker, job_description, job_time, None, cspr_amount);
        }

        #[should_panic]
        it "cannot create a job if worker is not kycd" {
            kyc_token.mint(job_poster, U256::from(1)).unwrap();
            bid_escrow_contract.as_account(job_poster).pick_bid_with_cspr_amount(worker, job_description, job_time, None, cspr_amount);
        }

        describe "with picked bid for non VA" {
            before {
                kyc_token.mint(job_poster, U256::from(1)).unwrap();
                kyc_token.mint(worker, U256::from(2)).unwrap();
                #[allow(clippy::redundant_clone)]
                bid_escrow_contract.as_account(job_poster).pick_bid_with_cspr_amount(worker, job_description.clone(), job_time, None, cspr_amount);
                #[allow(unused_variables)]
                let block_time = bid_escrow_contract.get_env().get_block_time();
                #[allow(unused_variables)]
                let job_created_event: JobCreated = bid_escrow_contract.event(-2);
                #[allow(unused_variables)]
                let job_accepted_event: JobAccepted = bid_escrow_contract.event(-1);
                let bid_id: BidId = 0;
                #[allow(unused_variables)]
                let job = bid_escrow_contract.get_job(bid_id).unwrap();
            }

            it "transfers cspr from job poster to the contract" {
                assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(job_poster), job_poster_initial_cspr - cspr_amount);
                assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(worker), worker_initial_cspr);
                assert_eq!(bid_escrow_contract.get_cspr_balance(), cspr_amount);
            }

            it "emits correct events" {
                assert_eq!(job_created_event, JobCreated { bid_id, job_poster, worker, description: job_description, finish_time: block_time + job_time, required_stake: None, cspr_amount });
                assert_eq!(job_accepted_event, JobAccepted { bid_id, job_poster, worker});
            }

            it "creates correct job" {
                assert_eq!(job.poster(), job_poster);
                assert_eq!(job.worker(), worker);
                assert_eq!(job.bid_id(), bid_id);
                assert_eq!(job.description(), "Job Description");
                assert_eq!(job.result(), None);
                assert_eq!(job.required_stake(), None);
                assert_eq!(job.status(), JobStatus::Accepted);
            }

            it "cannot be cancelled" {
                let result = bid_escrow_contract.as_account(job_poster).cancel_job(bid_id);
                assert_eq!(result, Err(Error::CannotCancelJob));
            }

            it "cannot be accepted" {
                let result = bid_escrow_contract.as_account(worker).accept_job(bid_id);
                assert_eq!(result, Err(Error::CannotAcceptJob));
            }

            it "can be finished by worker before the time ends" {
                bid_escrow_contract.as_account(worker).submit_result(bid_id, job_result).unwrap();
                let job = bid_escrow_contract.get_job(bid_id).unwrap();

                assert_eq!(job.status(), JobStatus::Submitted);
            }

            it "cannot be finished by anyone else before the time ends" {
                let result = bid_escrow_contract.as_account(job_poster).submit_result(bid_id, job_result.clone());
                assert_eq!(result, Err(Error::NotAuthorizedToSubmitResult));
                let result = bid_escrow_contract.as_account(anyone).submit_result(bid_id, job_result);
                assert_eq!(result, Err(Error::NotAuthorizedToSubmitResult));
            }

            describe "after time has ended" {
                before {
                    bid_escrow_contract.advance_block_time_by(job_time);
                }

                it "can be submitted by a job poster" {
                    bid_escrow_contract.as_account(job_poster).submit_result(bid_id, job_result).unwrap();
                    let job = bid_escrow_contract.get_job(bid_id).unwrap();

                    assert_eq!(job.status(), JobStatus::Submitted);
                }

                it "can be submitted by a worker" {
                    bid_escrow_contract.as_account(worker).submit_result(bid_id, job_result).unwrap();
                    let job = bid_escrow_contract.get_job(bid_id).unwrap();

                    assert_eq!(job.status(), JobStatus::Submitted);
                }

                it "cannot be submitted by anyone else" {
                    let result = bid_escrow_contract.as_account(anyone).submit_result(bid_id, job_result);
                    assert_eq!(result, Err(Error::NotAuthorizedToSubmitResult))
                }
            }
        }

        describe "with picked job for the VA" {
            before {
                kyc_token.mint(job_poster, U256::from(1)).unwrap();
                kyc_token.mint(worker, U256::from(2)).unwrap();
                va_token.mint(worker, U256::from(1)).unwrap();
                bid_escrow_contract.as_account(job_poster).pick_bid_with_cspr_amount(worker, job_description, job_time, None, cspr_amount);
                #[allow(unused_variables)]
                let job_created_event: JobCreated = bid_escrow_contract.event(-1);
                let bid_id: BidId = 0;
                #[allow(unused_variables)]
                let job = bid_escrow_contract.get_job(bid_id).unwrap();
            }

            it "transfers cspr from job poster to the contract" {
                assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(job_poster), job_poster_initial_cspr - cspr_amount);
                assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(worker), worker_initial_cspr);
                assert_eq!(bid_escrow_contract.get_cspr_balance(), cspr_amount);
            }

            it "is not automatically accepted" {
                assert_eq!(job.status(), JobStatus::Created);
            }

            it "can be cancelled by the job poster" {
                bid_escrow_contract.as_account(job_poster).cancel_job(bid_id).unwrap();
                let job = bid_escrow_contract.get_job(bid_id).unwrap();

                assert_eq!(job.status(), JobStatus::Cancelled);
            }

            test "the job cannot be completed if not accepted" {
                let result = bid_escrow_contract.as_account(worker).submit_result(bid_id, job_result);
                assert_eq!(result, Err(Error::NotAuthorizedToSubmitResult))
            }

            test "the VA can accept the job" {
                bid_escrow_contract.as_account(worker).accept_job(bid_id).unwrap();
                let job = bid_escrow_contract.get_job(bid_id).unwrap();

                assert_eq!(job.status(), JobStatus::Accepted);
            }

            describe "the time has ended but job was not accepted" {
                before {
                    bid_escrow_contract.advance_block_time_by(job_time);
                }

                test "the job can be cancelled" {
                    bid_escrow_contract.as_account(job_poster).cancel_job(bid_id).unwrap();
                    let job = bid_escrow_contract.get_job(bid_id).unwrap();

                    assert_eq!(job.status(), JobStatus::Cancelled);
                }

                test "the job cannot be accepted" {
                    let result = bid_escrow_contract.as_account(worker).accept_job(bid_id);
                    assert_eq!(result, Err(Error::CannotAcceptJob));
                }
            }

            describe "the job was accepted" {
                before {
                    bid_escrow_contract.as_account(worker).accept_job(bid_id).unwrap();
                }

                it "cannot be cancelled" {
                    let result = bid_escrow_contract.as_account(job_poster).cancel_job(bid_id);
                    assert_eq!(result, Err(Error::CannotCancelJob));
                }

                describe "the time has ended" {
                    before {
                        bid_escrow_contract.advance_block_time_by(job_time);
                    }

                    test "the job can be finished by the job creator" {
                        bid_escrow_contract.as_account(job_poster).submit_result(bid_id, job_result).unwrap();
                        let job = bid_escrow_contract.get_job(bid_id).unwrap();

                        assert_eq!(job.status(), JobStatus::Submitted);
                    }

                    test "the job can be finished by the worker" {
                        bid_escrow_contract.as_account(worker).submit_result(bid_id, job_result).unwrap();
                        let job = bid_escrow_contract.get_job(bid_id).unwrap();

                        assert_eq!(job.status(), JobStatus::Submitted);
                    }
                }
            }

        }

        describe "with job submitted" {
            before {
                let bid_id: BidId = 0;
                #[allow(unused_variables)]
                let informal_voting_id = U256::zero();
                #[allow(unused_variables)]
                let formal_voting_id = U256::one();

                kyc_token.mint(job_poster, U256::from(1)).unwrap();
                kyc_token.mint(worker, U256::from(2)).unwrap();
                va_token.mint(worker, U256::from(1)).unwrap();
                bid_escrow_contract.as_account(job_poster).pick_bid_with_cspr_amount(worker, job_description, job_time, None, cspr_amount);
                bid_escrow_contract.as_account(worker).accept_job(bid_id).unwrap();
                #[allow(clippy::redundant_clone)]
                bid_escrow_contract.as_account(worker).submit_result(bid_id, job_result.clone()).unwrap();

                #[allow(unused_variables)]
                let job = bid_escrow_contract.get_job(bid_id).unwrap();
                #[allow(unused_variables)]
                let anyone2 = bid_escrow_contract.get_env().get_account(4);
                #[allow(unused_variables)]
                let anyone3 = bid_escrow_contract.get_env().get_account(5);
            }

            it "emits proper events" {
                // last event is first empty vote of the creator
                let voting_created_event: VotingCreated = bid_escrow_contract.event(-2);
                let job_submitted_event: JobSubmitted = bid_escrow_contract.event(-3);
                assert_eq!(job_submitted_event, JobSubmitted { bid_id, job_poster, worker, result: job_result });
                assert_eq!(voting_created_event, VotingCreated { creator: job_poster, voting_id: casper_dao_contracts::voting::VotingId::zero(), stake: U256::from(0) });
            }

            it "prevents job poster and worker from voting" {
                let result = bid_escrow_contract.as_account(worker).vote(bid_id, informal_voting_id, Choice::InFavor, U256::from(10));
                assert_eq!(result, Err(Error::CannotVoteOnOwnJob));
                let result = bid_escrow_contract.as_account(job_poster).vote(bid_id, informal_voting_id, Choice::InFavor, U256::from(10));
                assert_eq!(result, Err(Error::CannotVoteOnOwnJob));
            }

            it "allows anyone else to vote" {
                let result = bid_escrow_contract.as_account(anyone).vote(bid_id, informal_voting_id, Choice::InFavor, U256::from(10));
                assert_eq!(result, Ok(()));
            }

            describe "when vote passes" {
                before {
                    bid_escrow_contract.as_account(anyone).vote(bid_id, informal_voting_id, Choice::InFavor, U256::from(10)).unwrap();
                    bid_escrow_contract.as_account(anyone2).vote(bid_id, informal_voting_id, Choice::InFavor, U256::from(10)).unwrap();
                    bid_escrow_contract.as_account(anyone3).vote(bid_id, informal_voting_id, Choice::InFavor, U256::from(10)).unwrap();
                    bid_escrow_contract.advance_block_time_by(informal_voting_time);
                    bid_escrow_contract.as_account(worker).finish_voting(bid_id, informal_voting_id).unwrap();
                    bid_escrow_contract.as_account(anyone).vote(bid_id, formal_voting_id, Choice::InFavor, U256::from(10)).unwrap();
                    bid_escrow_contract.as_account(anyone2).vote(bid_id, formal_voting_id, Choice::InFavor, U256::from(10)).unwrap();
                    bid_escrow_contract.as_account(anyone3).vote(bid_id, formal_voting_id, Choice::InFavor, U256::from(10)).unwrap();
                    bid_escrow_contract.advance_block_time_by(formal_voting_time);
                    #[allow(unused_variables)]
                    let voting_summary = bid_escrow_contract.as_account(worker).finish_voting(bid_id, formal_voting_id);
                }

                it "transfers cspr from the contract to the worker" {
                    assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(job_poster), job_poster_initial_cspr - cspr_amount);
                    assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(worker), worker_initial_cspr + cspr_amount);
                    assert_eq!(bid_escrow_contract.get_cspr_balance(), U512::zero());
                }

                it "changes job status to completed" {
                    let job = bid_escrow_contract.get_job(bid_id).unwrap();
                    assert_eq!(job.status(), JobStatus::Completed);
                }
            }

            describe "when vote fails" {
                before {
                    bid_escrow_contract.as_account(anyone).vote(bid_id, informal_voting_id, Choice::Against, U256::from(10)).unwrap();
                    bid_escrow_contract.as_account(anyone2).vote(bid_id, informal_voting_id, Choice::Against, U256::from(10)).unwrap();
                    bid_escrow_contract.as_account(anyone3).vote(bid_id, informal_voting_id, Choice::Against, U256::from(10)).unwrap();
                    bid_escrow_contract.advance_block_time_by(informal_voting_time);
                    bid_escrow_contract.as_account(job_poster).finish_voting(bid_id, informal_voting_id).unwrap();
                }

                it "changes job status to not completed" {
                    let job = bid_escrow_contract.get_job(bid_id).unwrap();
                    assert_eq!(job.status(), JobStatus::NotCompleted);
                }

                it "transfers cspr from the contract to the job poster" {
                    assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(job_poster), job_poster_initial_cspr);
                    assert_eq!(bid_escrow_contract.get_env().get_account_cspr_balance(worker), worker_initial_cspr);
                    assert_eq!(bid_escrow_contract.get_cspr_balance(), U512::zero());
                }
            }
        }
    }
}
