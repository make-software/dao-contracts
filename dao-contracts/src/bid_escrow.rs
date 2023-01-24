//! Contains Bid Escrow Contract definition and related abstractions.
//!
//! # Definitions
//! * Job Offer - A description of a Job posted by JobPoster
//! * Bid - on offer that can be accepted by the Job Poster
//! * JobPoster - user of the system that posts a Job Offer; it has to be KYC’d
//! * Worker - the user who does a job
//! * Internal Worker - a Worker who completed the KYC and was voted to be a VotingAssociate
//! * External Worker - a Worker who completed the KYC and is not a Voting Associate
//! * Voting Associate (or VA) - users of the system with Reputation and permissions to vote
//! * KYC - Know Your Customer, a process that validates that the user can be the user of the system
//! * Bid Escrow Voting - Mints reputation
//!
//! # Posting
//! The first step of the `Bid Escrow` process is `Posting` a `Job Offer`.
//! It is done by `JobPoster` by sending a query to a `BidEscrow` contract containing:
//! * Expected timeframe for completing a `Job`
//! * Maximum budget for a `Job`
//! With the query, the `JobPoster` sends a `DOS fee` in `CSPR`. The minimum amount of a `DOS fee` is defined in
//! [`Variable Repository Contract`] under the key`PostJobDOSFee`.
//! This action creates a new object in the contract called `Job Offer` and starts the `Bidding process`.
//!
//! # Bidding
//! The `Bidding` process allows `Workers` to post `Bids` with the offer of completing a job.
//!
//! During the `Auction` process the `Job Poster` can pick a `Bid`.
//! When no `Bid` is posted or selected by `Job Poster` during both auctions, the `Job` is cancelled,
//! `DOS Fee` is returned to the `Job Poster` and stakes sent by the `Bidders` are returned to them.
//!
//! # Submitting a Job Proof
//! Now the `Worker` has the time to complete the `Job` and submit its proof to the contract.
//! After the works have been completed, the `Worker` sends a query to the contract containing
//! the cryptographic hash of a document being a proof of `Work` done for a `Job Poster`.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! # Voting passed
//! Besides yielding a positive result, the `Voting` passed means that the `Reputation` staked by the losing side is
//! redistributed between the winning side, depending on the type of `Worker`.
//! ## External Worker who wanted to become VA
//! * The `External Worker` becomes VA.
//! * The `CSPR` that were sent by the `External Worker` as a stake is returned to the `External Worker`.
//! * Reputation of the voters who voted `yes` is returned to them
//! * Reputation of the voters who voted `no` is redistributed between the voters who voted `yes` proportional to the amount of 
//! reputation staked in the voting
//! * Reputation minted for the `External Worker` and used in the voting process is burned.
//! ## Internal Worker
//! * Reputation of the voters who voted `yes` is returned to them
//! * Reputation of the voters who voted `no` is redistributed between the voters who voted `yes` proportional to the amount of 
//! reputation staked in the voting
//! ## External Worker
//! * The `CSPR` that were sent by the `External Worker` as a stake is returned to the `External Worker`.
//! * Reputation minted for the `External Worker` and used in the voting process is burned.
//! * Reputation of the voters who voted `yes` is returned to them, except for the Reputation minted for the Worker using `CSPR` stake
//! * Reputation of the voters who voted `no` is redistributed between the voters who voted `yes` proportional to the amount of 
//! reputation staked in the voting (External Worker does not receive Reputation in this step)
//!
//! # Voting failed
//! Besides yielding a negative result, the Voting passed means that the Reputation staked by the losing side is
//! redistributed between the winning side, depending on the type of Worker.
//!
//! ## External Worker who wanted to become VA
//! * The External Worked DOES NOT become a VA
//! * The `CSPR` that were sent by the `External Worker` as a stake is redistributed between the `VA`’s
//! * The Reputation minted for the `External Worker` using `CSPR` stake is burned.
//! * Reputation of the voters who voted `no` is returned to them
//! * Reputation of the voters who voted `yes` is redistributed between the voters who voted `no` proportional to the amount of 
//! reputation staked in the voting
//! ## Internal Worker
//! * Reputation of the voters who voted `no` is returned to them
//! * Reputation of the voters who voted `yes` is redistributed between the voters who voted `no` proportional to the amount of 
//! reputation staked in the voting
//! ## External Worker
//! The `CSPR` that were sent by the `External Worker` as a stake is redistributed between the `VA`’s
//! The Reputation minted for the `External Worker` using `CSPR` stake is burned.
//! Reputation of the voters who voted `no` is returned to them
//! Reputation of the voters who voted `yes` is redistributed between the voters who voted `no` proportional to the amount of 
//! reputation staked in the voting
//! ## CSPR
//! If the `Voting` fails, the `CSPR` sent to the contract as a payment for `Job` is returned to the `Job Poster`. If the work
//! has been attempted to do by an `External Worker` the `CSPR` that the `Worker` staked during the `Bid` process
//! is redistributed between all `VA`’s.
//!
//! # Quorum not reached
//! When the `Quorum` is not reached during the `Formal Voting`, following things happen:
//! * The process ends here.
//! * `VA’s` stakes are returned to them
//! * `Job` Poster payment and `DOS fee is returned
//! * `Internal Worker`’s Reputation and `External Worker`’s `CSPR` stake is returned.
//! * `External Worker`’s Reputation that was minted using `CSPR` stake is burned.
//!
//! # Returning DOS Fee
//! The final step of the process is returning the `CSPR` `DOS Fee` to the `Job Poster`.
//!
//! # Grace Period
//! However, if `External Worker` do not post a `Job Proof` in time, his `CSPR` stake is redistributed
//! between all `VA’s`.
//! In case of `Internal Worker`, his staked `Reputation` gets burned and it undergoes the
//! `Automated Reputation slashing` see [`JobEngine.slash_worker()`].
//! Then the process enters a `Grace period` (with the timeframe the same as the timeframe of the work for `Worker`).
//! During this period, anyone (`VA`, `External Worker`, even the original `Worker`) can submit the `Job Proof`,
//! becoming the new `Worker` and participating in the reward mechanism. Alongside the `Job Proof`,
//! a `Worker` needs to send a stake in form of `Reputation` (or `CSPR` for `External Worker`).
//! This stake will behave in the same manner as stake sent by the original `Worker`.
//! If nobody submits the `Job Proof` during the grace period, the whole process ends.
//! The `CSPR` paid by the `Job Poster` is returned along with the `DOS Fee`.
//!
//! [`Variable Repository Contract`]: crate::variable_repository::VariableRepositoryContractInterface
//! [`VotingEngine`]: crate::voting::VotingEngine
//! [`JobEngine.slash_worker()`]: crate::bid_escrow::job_engine::JobEngine::slash_worker()
use std::borrow::Borrow;

use casper_dao_modules::AccessControl;
#[cfg(feature = "test-support")]
use casper_dao_utils::TestContract;
use casper_dao_utils::{
    casper_contract::unwrap_or_revert::UnwrapOrRevert,
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::{caller, revert},
    cspr,
    Address,
    BlockTime,
    DocumentHash,
    Error,
};
use casper_types::{URef, U512};
use delegate::delegate;

use crate::{
    bid_escrow::{
        bid::Bid,
        job::Job,
        job_offer::{JobOffer, JobOfferStatus},
        types::{BidId, JobId, JobOfferId},
    },
    reputation::ReputationContractInterface,
    voting::{
        refs::{ContractRefs, ContractRefsWithKycStorage},
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingId,
    },
};

pub mod bid;
mod bid_engine;
pub mod events;
pub mod job;
mod job_engine;
pub mod job_offer;
pub mod storage;
pub mod types;

pub use bid_engine::BidEngine;
pub use job_engine::JobEngine;

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsWithKycStorage`] by writing addresses of [`Variable Repository`](crate::variable_repository::VariableRepositoryContract),
    /// [`Reputation Token`](crate::reputation::ReputationContract), [`VA Token`](crate::va_nft::VaNftContract), [`KYC Token`](crate::KycNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](casper_dao_modules::events::OwnerChanged),
    /// * [`AddedToWhitelist`](casper_dao_modules::events::AddedToWhitelist),
    fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );

    /// Job Poster post a new Job Offer
    /// Parameters:
    /// expected_timeframe - Expected timeframe for completing a Job
    /// budget - Maximum budget for a Job
    /// Alongside Job Offer, Job Poster also sends DOS Fee in CSPR
    ///
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`JobOfferCreated`](crate::escrow::events::JobOfferCreated)
    fn post_job_offer(&mut self, expected_timeframe: BlockTime, budget: U512, purse: URef);
    /// Worker submits a Bid for a Job
    /// Parameters:
    /// time - proposed timeframe for completing a Job
    /// payment - proposed payment for a Job
    /// reputation_stake - reputation stake for a Job if Worker is an Internal Worker
    /// onboard - if Worker is an External Worker, then Worker can request to be onboarded after
    /// completing a Job
    /// purse: purse containing stake from External Worker
    ///
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`BidSubmitted`](crate::escrow::events::BidSubmitted)
    fn submit_bid(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    );
    /// Worker cancels a Bid for a Job
    /// Parameters:
    /// bid_id - Bid Id
    ///
    /// Bid can be cancelled only after VABidAcceptanceTimeout time has passed after submitting a Bid
    fn cancel_bid(&mut self, bid_id: BidId);
    /// Job poster picks a bid. This creates a new Job object and saves it in a storage.
    /// If worker is not onboarded, the job is accepted automatically.
    /// Otherwise, worker needs to accept job (see [accept_job](accept_job))
    ///
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`JobCreated`](JobCreated)
    ///
    /// Emits [`JobAccepted`](JobAccepted)
    ///
    /// # Errors
    /// Throws [`CannotPostJobForSelf`](Error::CannotPostJobForSelf) when trying to create job for
    /// self
    ///
    /// Throws [`JobPosterNotKycd`](Error::JobPosterNotKycd) or [`Error::WorkerNotKycd`](Error::WorkerNotKycd)
    /// When either Job Poster or Worker has not completed the KYC process
    fn pick_bid(&mut self, job_offer_id: u32, bid_id: u32, purse: URef);
    /// Submits a job proof. This is called by a Worker or any KYC'd user during Grace Period.
    /// This starts a new voting over the result.
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`JobProofSubmitted`](JobProofSubmitted)
    ///
    /// Emits [`VotingCreated`](crate::voting::voting_engine::events::VotingCreated)
    ///
    /// # Errors
    /// Throws [`JobAlreadySubmitted`](Error::JobAlreadySubmitted) if job was already submitted
    /// Throws [`NotAuthorizedToSubmitResult`](Error::NotAuthorizedToSubmitResult) if one of the constraints for
    /// job submission is not met.
    fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash);
    fn submit_job_proof_during_grace_period(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: U512,
        onboard: bool,
        purse: Option<URef>,
    );
    /// Casts a vote over a job
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`BallotCast`](crate::voting::voting_engine::events::BallotCast)

    /// # Errors
    /// Throws [`CannotVoteOnOwnJob`](Error::CannotVoteOnOwnJob) if the voter is either of Job Poster or Worker
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
    /// Returns a job with given JobId
    fn get_job(&self, job_id: JobId) -> Option<Job>;
    /// Returns a JobOffer with given JobOfferId
    fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;
    /// Returns a Bid with given BidId
    fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
    /// Finishes voting stage. Depending on stage, the voting can be converted to a formal one, end
    /// with a refund or pay the worker.
    /// # Events
    /// // TODO: Fix events documentation
    /// Emits [`VotingEnded`](crate::voting::voting_engine::events::VotingEnded), [`VotingCreated`](crate::voting::voting_engine::events::VotingCreated)
    /// # Errors
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// Returns the address of [Variable Repository](crate::variable_repository::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::reputation::ReputationContract) contract.
    fn reputation_token_address(&self) -> Address;
    /// see [VotingEngine](VotingEngine)
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    /// see [VotingEngine](VotingEngine)
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: VotingType,
        address: Address,
    ) -> Option<Ballot>;
    /// see [VotingEngine](VotingEngine)
    /// // TODO: Fix documentation link
    fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;

    /// Returns the CSPR balance of the contract
    fn get_cspr_balance(&self) -> U512;

    fn job_offers_count(&self) -> u32;

    fn jobs_count(&self) -> u32;

    fn bids_count(&self) -> u32;

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId);

    fn cancel_job_offer(&mut self, job_offer_id: JobOfferId);

    fn cancel_job(&mut self, job_id: JobId);

    fn slash_all_active_job_offers(&mut self, bidder: Address);

    fn slash_bid(&mut self, bid_id: BidId);

    // Whitelisting set.
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;

    fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
    fn slash_voter(&mut self, voter: Address, voting_id: VotingId);
}

/// A contract that manages the full `Bid Escrow` process.
/// Uses [`VotingEngine`](crate::voting::VotingEngine) to conduct the voting process.
///
/// For details see [BidEscrowContractInterface](BidEscrowContractInterface).
#[derive(Instance)]
pub struct BidEscrowContract {
    refs: ContractRefsWithKycStorage,
    access_control: AccessControl,
    job_engine: JobEngine,
    bid_engine: BidEngine,
}

impl BidEscrowContractInterface for BidEscrowContract {
    delegate! {
        to self.job_engine.voting_engine {
            fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;
            fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;
            fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;
            fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
        }

        to self.bid_engine {
            fn post_job_offer(&mut self, expected_timeframe: BlockTime, budget: U512, purse: URef);
            fn submit_bid(
                &mut self,
                job_offer_id: JobOfferId,
                time: BlockTime,
                payment: U512,
                reputation_stake: U512,
                onboard: bool,
                purse: Option<URef>,
            );
            fn cancel_bid(&mut self, bid_id: BidId);
            fn cancel_job_offer(&mut self, job_offer_id: JobOfferId);
            fn pick_bid(&mut self, job_offer_id: u32, bid_id: u32, purse: URef);

            fn job_offers_count(&self) -> u32;
            fn bids_count(&self) -> u32;
            fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;
            fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
        }

        to self.job_engine {
            fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash);
            fn submit_job_proof_during_grace_period(
                &mut self,
                job_id: JobId,
                proof: DocumentHash,
                reputation_stake: U512,
                onboard: bool,
                purse: Option<URef>,
            );
            fn cancel_job(&mut self, job_id: JobId);
            fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: U512);
            fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
        }

        to self.access_control {
            fn change_ownership(&mut self, owner: Address);
            fn add_to_whitelist(&mut self, address: Address);
            fn remove_from_whitelist(&mut self, address: Address);
            fn is_whitelisted(&self, address: Address) -> bool;
            fn get_owner(&self) -> Option<Address>;
        }

        to self.job_engine {
            fn jobs_count(&self) -> u32;
            fn get_job(&self, job_id: JobId) -> Option<Job>;
        }

        to self.refs {
            fn variable_repository_address(&self) -> Address;
            fn reputation_token_address(&self) -> Address;
        }
    }

    fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.refs
            .init(variable_repository, reputation_token, va_token, kyc_token);
        self.access_control.init(caller());
    }

    fn get_cspr_balance(&self) -> U512 {
        cspr::main_purse_balance()
    }

    fn cancel_voter(&mut self, voter: Address, voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        self.job_engine.voting_engine.slash_voter(voter, voting_id);
    }

    fn slash_all_active_job_offers(&mut self, bidder: Address) {
        self.access_control.ensure_whitelisted();
        // Cancel job offers created by the bidder.
        let job_offer_ids = self.bid_engine.clear_active_job_offers_ids(&bidder);
        for job_offer_id in job_offer_ids {
            self.bid_engine.cancel_all_bids(job_offer_id);
            self.bid_engine
                .return_job_offer_poster_dos_fee(job_offer_id);
        }
    }

    fn slash_bid(&mut self, bid_id: BidId) {
        self.access_control.ensure_whitelisted();

        let mut bid = self
            .get_bid(bid_id)
            .unwrap_or_revert_with(Error::BidNotFound);

        let job_offer = self
            .get_job_offer(bid.job_offer_id)
            .unwrap_or_revert_with(Error::JobOfferNotFound);

        if job_offer.status != JobOfferStatus::Created {
            revert(Error::CannotCancelBidOnCompletedJobOffer);
        }

        bid.cancel_without_validation();

        self.refs
            .reputation_token()
            .unstake_bid(bid.borrow().into());

        // TODO: Implement Event
        self.bid_engine.store_bid(bid);
    }

    fn slash_voter(&mut self, _voter: Address, _voting_id: VotingId) {
        self.access_control.ensure_whitelisted();
        unimplemented!()
    }
}

#[cfg(feature = "test-support")]
impl BidEscrowContractTest {
    pub fn pick_bid_with_cspr_amount(
        &mut self,
        job_offer_id: u32,
        bid_id: u32,
        cspr_amount: U512,
    ) -> Result<(), Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "pick_bid.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "job_offer_id" => job_offer_id,
                "bid_id" => bid_id,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }

    pub fn post_job_offer_with_cspr_amount(
        &mut self,
        expected_timeframe: BlockTime,
        budget: U512,
        cspr_amount: U512,
    ) -> Result<(), Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "post_job_offer.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "expected_timeframe" => expected_timeframe,
                "budget" => budget,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }

    pub fn submit_bid_with_cspr_amount(
        &mut self,
        job_offer_id: JobOfferId,
        time: BlockTime,
        payment: U512,
        reputation_stake: U512,
        onboard: bool,
        cspr_amount: U512,
    ) -> Result<(), Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "submit_bid.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "job_offer_id" => job_offer_id,
                "time" => time,
                "payment" => payment,
                "reputation_stake" => reputation_stake,
                "onboard" => onboard,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }

    pub fn submit_job_proof_during_grace_period_with_cspr_amount(
        &mut self,
        job_id: JobId,
        proof: DocumentHash,
        reputation_stake: U512,
        onboard: bool,
        cspr_amount: U512,
    ) -> Result<(), Error> {
        use casper_types::{runtime_args, RuntimeArgs};
        self.env.deploy_wasm_file(
            "submit_job_proof_during_grace_period.wasm",
            runtime_args! {
                "bid_escrow_address" => self.address(),
                "job_id" => job_id,
                "proof" => proof,
                "reputation_stake" => reputation_stake,
                "onboard" => onboard,
                "cspr_amount" => cspr_amount,
                "amount" => cspr_amount,
            },
        )
    }
}
