mod governance_voting_common;
extern crate speculate;
use speculate::speculate;
use casper_dao_utils::{TestContract, Error};
use casper_dao_contracts::bid::{types::BidId, events::{JobCreated, JobSubmitted, JobAccepted}, job::JobStatus};
use casper_types::U256;

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
        }

        it "cannot create a job for caller" {
            let result = bid_escrow_contract.as_account(job_poster).pick_bid(job_poster, job_description, job_time, None);
            assert_eq!(result, Err(Error::CannotPostJobForSelf));
        }

        it "cannot create a job if creator is not kycd" {
            kyc_token.mint(worker, U256::from(1)).unwrap();
            dbg!(kyc_token.balance_of(worker));
            dbg!(kyc_token.balance_of(job_poster));
            let result = bid_escrow_contract.as_account(job_poster).pick_bid(worker, job_description, job_time, None);
            assert_eq!(result, Err(Error::JobPosterNotKycd));
        }

        it "cannot create a job if worker is not kycd" {
            kyc_token.mint(job_poster, U256::from(1)).unwrap();
            let result = bid_escrow_contract.as_account(job_poster).pick_bid(worker, job_description, job_time, None);
            assert_eq!(result, Err(Error::WorkerNotKycd));
        }

        describe "with picked bid for non VA" {
            before {
                kyc_token.mint(job_poster, U256::from(1)).unwrap();
                kyc_token.mint(worker, U256::from(2)).unwrap();
                #[allow(clippy::redundant_clone)]
                bid_escrow_contract.as_account(job_poster).pick_bid(worker, job_description.clone(), job_time, None).unwrap();
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

            it "emits correct events" {
                assert_eq!(job_created_event, JobCreated { bid_id, job_poster, worker, description: job_description, finish_time: block_time + job_time, required_stake: None });
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
                bid_escrow_contract.as_account(worker).submit_result(bid_id, job_result.clone()).unwrap();
                let job_submitted_event: JobSubmitted = bid_escrow_contract.event(-1);
                assert_eq!(job_submitted_event, JobSubmitted { bid_id, job_poster, worker, result: job_result })
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

                it "can be completed by a job poster" {
                    bid_escrow_contract.as_account(job_poster).submit_result(bid_id, job_result).unwrap();
                }

                it "cannot be completed by anyone" {
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
                bid_escrow_contract.as_account(job_poster).pick_bid(worker, job_description, job_time, None).unwrap();
                #[allow(unused_variables)]
                let job_created_event: JobCreated = bid_escrow_contract.event(-1);
                let bid_id: BidId = 0;
                #[allow(unused_variables)]
                let job = bid_escrow_contract.get_job(bid_id).unwrap();
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
                }
            }

        }
    }
}
