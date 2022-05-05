mod governance_voting_common;
extern crate speculate;
use speculate::speculate;
use casper_dao_utils::TestContract;
use casper_dao_contracts::bid::{types::BidId, events::{JobCreated}, job::JobStatus};
use casper_types::U256;

speculate! {
    describe "bid escrow contract" {
        before {
          let (mut bid_escrow_contract, _reputation_token_contract, mut va_token, mut kyc_token) = governance_voting_common::setup_bid_escrow();
          let job_poster = bid_escrow_contract.get_env().get_account(1);
          let worker = bid_escrow_contract.get_env().get_account(2);
        }

        it "cannot create a job for caller" {
            let result = bid_escrow_contract.as_account(job_poster).pick_bid(job_poster, "Job Description".to_string(), None);
            assert_eq!(result, Err(casper_dao_utils::Error::CannotPostJobForSelf));
        }

        it "cannot create a job if creator is not kycd" {
            kyc_token.mint(worker, U256::from(1)).unwrap();
            let result = bid_escrow_contract.as_account(job_poster).pick_bid(worker, "Job Description".to_string(), None);
            assert_eq!(result, Err(casper_dao_utils::Error::JobPosterNotKycd));
        }

        it "cannot create a job if worker is not kycd" {
            kyc_token.mint(job_poster, U256::from(1)).unwrap();
            let result = bid_escrow_contract.as_account(job_poster).pick_bid(worker, "Job Description".to_string(), None);
            assert_eq!(result, Err(casper_dao_utils::Error::WorkerNotKycd));
        }

        describe "with picked job for non VA" {
            before {
                kyc_token.mint(job_poster, U256::from(1)).unwrap();
                kyc_token.mint(worker, U256::from(2)).unwrap();
                bid_escrow_contract.as_account(job_poster).pick_bid(worker, "Job Description".to_string(), None).unwrap();
                let job_created_event: JobCreated = bid_escrow_contract.event(-1);
                let bid_id: BidId = 0;
                let job = bid_escrow_contract.get_job(bid_id).unwrap();
            }

            it "emits correct event" {
                assert_eq!(job_created_event, JobCreated { bid_id, job_poster, worker, description: "Job Description".to_string(), required_stake: None })
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
        }

        describe "with picked job for VA" {
            before {
                kyc_token.mint(job_poster, U256::from(1)).unwrap();
                kyc_token.mint(worker, U256::from(2)).unwrap();
                va_token.mint(worker, U256::from(1)).unwrap();
                bid_escrow_contract.as_account(job_poster).pick_bid(worker, "Job Description".to_string(), None).unwrap();
                let job_created_event: JobCreated = bid_escrow_contract.event(-1);
                let bid_id: BidId = 0;
                let job = bid_escrow_contract.get_job(bid_id).unwrap();
            }
        }
    }
}
