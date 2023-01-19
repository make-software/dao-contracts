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
    escrow::{
        bid::Bid,
        bid_engine::BidEngine,
        job::Job,
        job_engine::JobEngine,
        job_offer::{JobOffer, JobOfferStatus},
        types::{BidId, JobId, JobOfferId},
    },
    refs::{ContractRefs, ContractRefsWithKycStorage},
    voting::{
        voting_state_machine::{VotingStateMachine, VotingType},
        Ballot,
        Choice,
        VotingId,
    },
    ReputationContractInterface,
};

#[casper_contract_interface]
pub trait BidEscrowContractInterface {
    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up [`ContractRefsWithKycStorage`] by writing addresses of [`Variable Repository`](crate::VariableRepositoryContract),
    /// [`Reputation Token`](crate::ReputationContract), [`VA Token`](crate::VaNftContract), [`KYC Token`](crate::KycNftContract).
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
    /// Emits [`VotingEnded`](crate::voting::voting_engine::events::VotingEnded), [`VotingCreated`](crate::voting::voting_engine::events::VotingCreated)
    /// # Errors
    /// Throws [`VotingNotStarted`](Error::VotingNotStarted) if the voting was not yet started for this job
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType);
    /// Returns the address of [Variable Repository](crate::VariableRepositoryContract) contract.
    fn variable_repository_address(&self) -> Address;
    /// Returns the address of [Reputation Token](crate::ReputationContract) contract.
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

#[derive(Instance)]
pub struct BidEscrowContract {
    refs: ContractRefsWithKycStorage,
    access_control: AccessControl,
    job_engine: JobEngine,
    bid_engine: BidEngine,
}

impl BidEscrowContractInterface for BidEscrowContract {
    delegate! {
        to self.job_engine.voting {
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
        self.job_engine.voting.slash_voter(voter, voting_id);
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
