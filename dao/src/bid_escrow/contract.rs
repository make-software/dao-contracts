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
//! The first step of the `Bid Escrow` process is `Posting` a [`Job Offer`](JobOffer).
//! It is done by `JobPoster` by sending a query to a `BidEscrow` contract containing:
//! * Expected timeframe for completing a `Job`
//! * Maximum budget for a `Job`
//! With the query, the `JobPoster` sends a `DOS fee` in `CSPR`. The minimum amount of a `DOS fee` is defined in
//! [`Variable Repository Contract`] under the key`PostJobDOSFee`.
//! This action creates a new object in the contract called `Job Offer` and starts the `Bidding process`.
//!
//! # Bidding
//! The `Bidding` process allows `Workers` to post [`Bids`](Bid) with the offer of completing a job.
//! It is divided into two main parts.
//!
//! ### Internal Auction
//! During this part of the Bidding process only the `VAs` can bid. As the `VAs` have `Reputation`, they are bidding using
//! [`Reputation`] as a stake. The `Bid` query to the contract consists of:
//! * Proposed timeframe for completing the `Job`.
//! * Proposed payment for the `Job`.
//! * The amount of `Reputation` the `Internal Worker` stakes on this `Job`.
//!
//! The Bid is then added to a list of available bids in the contract storage and is available for picking by the `Job Poster`.
//! The time of an `Internal Auction` is defined in a [`Governance Variable`] `InternalAuctionTime`.
//! The bidding process can already be completed here, if the `JobPoster` decides to chose one of the posted Bids before `Internal Auction` ends.
//! However, if no `Bid` is picked during this time, the process becomes a `Public Auction`.
//!
//! ### Public Auction
//! If no `Internal Worker` decides to post a `Bid` on a `Job Offer`, or the `Job Poster` did not pick any bid during `Internal Auction`,
//! the `External Workers` have a chance of submitting their bids during `Public Auction` time. As `External Workers` do not have any
//! `Reputation` to stake, they are staking `CSPR`.
//!
//! A query to the contract in case of `External Workers` consists of:
//! * Proposed timeframe for completing the `Job`.
//! * Proposed payment for the `Job`.
//! * Decision if the `Worker` wants to become a `Voting Associate` if the `Job` is completed.
//! * `CSPR` stake sent alongside the query
//! `Internal Workers` by default cannot to submit their bids during `Public Auction`, however this behavior is configurable using
//! `VACanBidOnPublicAuction` [`Governance Variable`].
//!
//! The time of a `Public Auction` is defined in a [`Governance Variable`] `PublicAuctionTime`.
//!
//! # Picking a Bid
//! During the `Auction` process the `Job Poster` can pick a `Bid`.
//! When no `Bid` is posted or selected by `Job Poster` during both auctions, the `Job` is cancelled,
//! `DOS Fee` is returned to the `Job Poster` and stakes sent by the `Bidders` are returned to them.
//!
//! # Submitting a Job Proof
//! Now the `Worker` has the time to complete the `Job` and submit its proof to the contract.
//! After the works have been completed, the `Worker` sends a query to the contract containing
//! the cryptographic hash of a document being a proof of `Work` done for a `Job Poster`.
//!
//! When no `Bid` is posted or selected by the `Job Poster` during both auctions, the `Job` is cancelled,
//! `DOS Fee` is returned to the `Job Poster` and stakes sent by the `Bidders` are returned to them.
//!
//! # Voting
//! The Voting process is managed by [`VotingEngine`].
//!
//! # Voting passed
//! Besides yielding a positive result, the `Voting` passed means that the `Reputation` staked by the losing side is
//! redistributed between the winning side, depending on the type of `Worker`.
//! ### External Worker who wanted to become VA
//! * The `External Worker` becomes VA.
//! * The `CSPR` that were sent by the `External Worker` as a stake is returned to the `External Worker`.
//! * Reputation of the voters who voted `yes` is returned to them.
//! * Reputation of the voters who voted `no` is redistributed between the voters who voted `yes` proportional to the amount of
//! reputation staked in the voting.
//! * Reputation minted for the `External Worker` and used in the voting process is burned.
//!
//! ### Internal Worker
//! * Reputation of the voters who voted `yes` is returned to them.
//! * Reputation of the voters who voted `no` is redistributed between the voters who voted `yes` proportional to the amount of
//! reputation staked in the voting.
//!
//! ### External Worker
//! * The `CSPR` that were sent by the `External Worker` as a stake is returned to the `External Worker`.
//! * Reputation minted for the `External Worker` and used in the voting process is burned.
//! * Reputation of the voters who voted `yes` is returned to them, except for the Reputation minted for the Worker using `CSPR` stake.
//! * Reputation of the voters who voted `no` is redistributed between the voters who voted `yes` proportional to the amount of
//! reputation staked in the voting (External Worker does not receive Reputation in this step).
//!
//! ### Payment CSPR Redistribution
//! Reputation used for the Voting and minted after a successful `Job` has been redistributed during the above process,
//! but there is `CSPR` to redistribute that was allocated to the `Job`. How much resources is redistributed and to whom
//! depends on the type of `Worker` and whether it wanted to become a `VA`.
//!
//! ### Payment pool
//! The `CSPR` to redistribute is calculated using a formula:
//! `payment pool = job price`
//!
//! ### External Worker who wants to become VA
//! As the External Worker now is the VA, it is considered to be an `Internal Worker` in this scenario.
//!
//! ### Internal Worker
//! Firstly the Governance Payment is calculated using a formula:
//!
//! `governance payment = payment pool * BidEscrowPaymentRatio` [Read more](crate::core_contracts::VariableRepositoryContract#available-keys).
//!
//! The `Governance Payment` is then transferred to a multisig wallet, which address is held in the [`Variable Repository Contract`]
//! called [`BidEscrowWalletAddress`](crate::core_contracts::VariableRepositoryContract#available-keys).
//! The rest of the payment is redistributed between all of the `VAs'`.
//!
//! `remaining amount = payment pool - governance payment`
//!
//! ### External Worker
//! If the `Job` was done by an `External Worker` who didn't want to become a `VA`, the first step is the same
//! as in the case of `Internal Worker` - Governance Payment is being made. However the rest is then divided between
//! the `External Worker` and the `VAs’`.
//!
//! Firstly to get the amount that VA’s receive we use a formula:
//!
//! `VA payment amount = remaining amount * DefaultPolicingRate` [Read more](crate::core_contracts::VariableRepositoryContract#available-keys)
//!
//! Then, the rest is transferred to the `External Worker`:
//!
//! `External Worker payment amount = payment pool- governance payment - VA payment amount`
//!
//! # Voting failed
//! Besides yielding a negative result, the Voting passed means that the Reputation staked by the losing side is
//! redistributed between the winning side, depending on the type of Worker.
//!
//! ### External Worker who wanted to become VA
//! * The External Worked DOES NOT become a VA.
//! * The `CSPR` that were sent by the `External Worker` as a stake is redistributed between the `VA`’s.
//! * The Reputation minted for the `External Worker` using `CSPR` stake is burned.
//! * Reputation of the voters who voted `no` is returned to them.
//! * Reputation of the voters who voted `yes` is redistributed between the voters who voted `no` proportional to the amount of
//! reputation staked in the voting.
//!
//! ### Internal Worker
//! * Reputation of the voters who voted `no` is returned to them.
//! * Reputation of the voters who voted `yes` is redistributed between the voters who voted `no` proportional to the amount of
//! reputation staked in the voting.
//!
//! ### External Worker
//! The `CSPR` that were sent by the `External Worker` as a stake is redistributed between the `VA`’s.
//! The Reputation minted for the `External Worker` using `CSPR` stake is burned.
//! Reputation of the voters who voted `no` is returned to them.
//! Reputation of the voters who voted `yes` is redistributed between the voters who voted `no` proportional to the amount of
//! reputation staked in the voting.
//!
//! ### CSPR
//! If the `Voting` fails, the `CSPR` sent to the contract as a payment for `Job` is returned to the `Job Poster`. If the work
//! has been attempted to do by an `External Worker` the `CSPR` that the `Worker` staked during the `Bid` process
//! is redistributed between all `VA`’s.
//!
//! # Quorum not reached
//! When the `Quorum` is not reached during the `Formal Voting`, following things happen:
//! * The process ends here.
//! * `VA’s` stakes are returned to them.
//! * `Job` Poster payment and `DOS fee is returned.
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
//! `Automated Reputation slashing` see [`Slashing Voter`].
//! Then the process enters a `Grace period` (with the timeframe the same as the timeframe of the work for `Worker`).
//! During this period, anyone (`VA`, `External Worker`, even the original `Worker`) can submit the `Job Proof`,
//! becoming the new `Worker` and participating in the reward mechanism. Alongside the `Job Proof`,
//! a `Worker` needs to send a stake in form of `Reputation` (or `CSPR` for `External Worker`).
//! This stake will behave in the same manner as stake sent by the original `Worker`.
//! If nobody submits the `Job Proof` during the grace period, the whole process ends.
//! The `CSPR` paid by the `Job Poster` is returned along with the `DOS Fee`.
//! This is a special implementation of [positional parameters].
//!
//! [`Variable Repository Contract`]: crate::core_contracts::VariableRepositoryContract
//! [`VotingEngine`]: VotingEngine
//! [`Slashing Voter`]: crate::voting_contracts::SlashingVoterContract
//! [`Reputation`]: crate::core_contracts::ReputationContract
//! [`Governance Variable`]: crate::core_contracts::VariableRepositoryContract#available-keys

use crate::bid_escrow::bid::Bid;
use crate::bid_escrow::bid_engine::BidEngine;
use crate::bid_escrow::events::BidEscrowSlashResults;
use crate::bid_escrow::job::Job;
use crate::bid_escrow::job_engine::JobEngine;
use crate::bid_escrow::job_offer::JobOffer;
use crate::bid_escrow::types::{BidId, JobId, JobOfferId};
use crate::modules::refs::ContractRefs;
use crate::modules::AccessControl;
use crate::utils::types::DocumentHash;
use crate::voting::ballot::{Ballot, Choice};
use crate::voting::types::VotingId;
use crate::voting::voting_engine::voting_state_machine::{
    VotingStateMachine, VotingSummary, VotingType,
};
use crate::voting::voting_engine::VotingEngine;
use odra::contract_env::{caller, self_balance};
use odra::types::{event::OdraEvent, Address, Balance, BlockTime};

use super::storage::{BidStorage, JobStorage};

/// A contract that manages the full `Bid Escrow` process.
/// Uses [`VotingEngine`](crate::voting::voting_engine::VotingEngine) to conduct the voting process.
///
/// For details see [BidEscrowContract](BidEscrowContract).
#[odra::module]
#[allow(dead_code)]
pub struct BidEscrowContract {
    refs: ContractRefs,
    access_control: AccessControl,
    #[odra(using = "refs, voting_engine, job_storage, bid_storage")]
    job_engine: JobEngine,
    #[odra(using = "refs, job_storage, bid_storage")]
    bid_engine: BidEngine,
    #[odra(using = "refs")]
    voting_engine: VotingEngine,
    job_storage: JobStorage,
    bid_storage: BidStorage,
}

#[odra::module]
impl BidEscrowContract {
    delegate! {
        to self.voting_engine {
            /// Checks if voting of a given type and id exists.
            pub fn voting_exists(&self, voting_id: VotingId, voting_type: VotingType) -> bool;

            /// Returns the Voter's [`Ballot`].
            pub fn get_ballot(
                &self,
                voting_id: VotingId,
                voting_type: VotingType,
                address: Address,
            ) -> Option<Ballot>;

            /// Returns the address of nth voter who voted on Voting with `voting_id`.
            pub fn get_voter(&self, voting_id: VotingId, voting_type: VotingType, at: u32) -> Option<Address>;

            /// Returns [Voting](VotingStateMachine) for given id.
            pub fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
        }


        to self.bid_engine {
            /// Job Poster post a new Job Offer.
            ///
            /// # Errors
            /// * [`NotKyced`](crate::utils::Error::NotKyced) - if the caller is not KYCed
            /// * [`DosFeeTooLow`](crate::utils::Error::DosFeeTooLow) - if the caller has not sent enough DOS Fee
            ///
            /// # Events
            /// * [`JobOfferCreated`](crate::bid_escrow::events::JobOfferCreated)
            #[odra(payable)]
            pub fn post_job_offer(&mut self, expected_timeframe: BlockTime, budget: Balance, dos_fee: Balance);

            /// Job poster picks a bid. This creates a new Job object and saves it in a storage.
            ///
            /// # Events
            /// * [`JobCreated`](crate::bid_escrow::events::JobCreated)
            /// * [`Unstake`](crate::core_contracts::Unstake) - in case there were other bids on the same Job Offer
            ///
            /// # Errors
            /// * [`OnlyJobPosterCanPickABid`](crate::utils::Error::OnlyJobPosterCanPickABid) - if the caller is not the Job Poster
            /// * [`PurseBalanceMismatch`](crate::utils::Error::PurseBalanceMismatch) - if the purse balance does not match the bid amount
            #[odra(payable)]
            pub fn pick_bid(&mut self, job_offer_id: JobOfferId, bid_id: BidId, cspr_amount: Balance);

            /// Worker submits a [Bid] for a [Job].
            ///
            /// # Events
            /// * [`BidSubmitted`](crate::bid_escrow::events::BidSubmitted)
            /// * [`Stake`](crate::core_contracts::Stake)
            ///
            /// # Errors
            /// * [`NotKyced`](crate::utils::Error::NotKyced) - if the caller is not KYCed
            /// * [`CannotBidOnOwnJob`](crate::utils::Error::CannotBidOnOwnJob) - if the caller is the Job Poster
            /// * [`VaOnboardedAlready`](crate::utils::Error::VaOnboardedAlready) - if the caller has already been onboarded, but is trying to
            /// onboard again
            /// * [`PaymentExceedsMaxBudget`](crate::utils::Error::PaymentExceedsMaxBudget) - if the proposed payment exceeds the maximum budget
            /// * [`AuctionNotRunning`](crate::utils::Error::AuctionNotRunning) - if the auction is not running
            /// * [`OnlyOnboardedWorkerCanBid`](crate::utils::Error::OnlyOnboardedWorkerCanBid) - if the Worker is not onboarded and the
            /// auction is internal
            /// * [`OnboardedWorkerCannotBid`](crate::utils::Error::OnboardedWorkerCannotBid) - if the Worker is onboarded, auction is public
            /// and the configuration forbids bidding by onboarded Workers on such auctions
            /// * [`InsufficientBalance`](crate::utils::Error::InsufficientBalance) - if the Worker has not enough balance to pay for the
            /// bid
            /// * [`ZeroStake`](crate::utils::Error::ZeroStake) - if the Worker tries to stake 0 reputation
            /// * [`NotWhitelisted`](crate::utils::Error::NotWhitelisted) - if the contract is not whitelisted for Reputation Staking
            #[odra(payable)]
            pub fn submit_bid(
                &mut self,
                job_offer_id: JobOfferId,
                time: BlockTime,
                payment: Balance,
                reputation_stake: Balance,
                onboard: bool,
                cspr_stake: Option<Balance>
            );

            /// Worker cancels a [Bid] for a [Job].
            ///
            /// Bid can be cancelled only after VABidAcceptanceTimeout time has passed after submitting a Bid.
            ///
            /// # Events
            /// * [`BidCancelled`](crate::bid_escrow::events::BidCancelled)
            /// * [`Unstake`](crate::core_contracts::Unstake)
            ///
            /// # Errors:
            /// * [`CannotCancelNotOwnedBid`](crate::utils::Error::CannotCancelNotOwnedBid) when trying to cancel a Bid
            /// that is not owned by the Worker
            /// * [`CannotCancelBidOnCompletedJobOffer`](crate::utils::Error::CannotCancelBidOnCompletedJobOffer) when
            /// trying to cancel a Bid on a Job Offer that is already completed
            /// * [`CannotCancelBidBeforeAcceptanceTimeout`](crate::utils::Error::CannotCancelBidBeforeAcceptanceTimeout)
            /// when trying to cancel a Bid before VABidAcceptanceTimeout time has passed
            pub fn cancel_bid(&mut self, bid_id: BidId);

            /// Invalidates the [`Job Offer`](JobOffer), returns `DOS Fee` to the `Job Poster`, returns funds to `Bidders`.
            /// [`Read more`](BidEngine::cancel_job_offer()).
            pub fn cancel_job_offer(&mut self, job_offer_id: JobOfferId);

            /// Returns the total number of job offers.
            pub fn job_offers_count(&self) -> u32;

            /// Returns the total number of bids.
            pub fn bids_count(&self) -> u32;

            /// Returns a JobOffer with given [JobOfferId].
            pub fn get_job_offer(&self, job_offer_id: JobOfferId) -> Option<JobOffer>;

            /// Returns a Bid with given [BidId].
            pub fn get_bid(&self, bid_id: BidId) -> Option<Bid>;
        }

        to self.job_engine {
            /// Submits a job proof. This is called by a `Worker` or any KYC'd user during Grace Period.
            /// This starts a new voting over the result.
            ///
            /// # Events
            /// * [`JobSubmitted`](crate::bid_escrow::events::JobSubmitted)
            /// * [`Unstake`](crate::core_contracts::Unstake) - Stake is used in the voting
            /// * [`BallotCast`](crate::voting::voting_engine::events::BallotCast) - first vote is cast by the Worker
            ///
            /// # Errors
            /// Throws [`JobAlreadySubmitted`](crate::utils::Error::JobAlreadySubmitted) if job was already submitted.
            /// Throws [`OnlyWorkerCanSubmitProof`](crate::utils::Error::OnlyWorkerCanSubmitProof) if the caller is not the Worker
            /// and the grace period is not ongoing.
            pub fn submit_job_proof(&mut self, job_id: JobId, proof: DocumentHash);

            /// Updates the old [`Bid`] and [`Job`], the job is assigned to a new `Worker`. The rest goes the same
            /// as regular proof submission. See [submit_job_proof()][Self::submit_job_proof].
            /// The old `Worker` who didn't submit the proof in time, is getting slashed.
            #[odra(payable)]
            pub fn submit_job_proof_during_grace_period(
                &mut self,
                job_id: JobId,
                proof: DocumentHash,
                reputation_stake: Balance,
                onboard: bool,
            );

            pub fn cancel_job(&mut self, job_id: JobId);

            /// Casts a vote over a job.
            ///
            /// # Events
            /// * [`BallotCast`](crate::voting::voting_engine::events::BallotCast)
            ///
            /// # Errors
            /// * [`CannotVoteOnOwnJob`](crate::utils::Error::CannotVoteOnOwnJob) if the voter is either of Job Poster or Worker
            /// * [`VotingNotStarted`](crate::utils::Error::VotingNotStarted) if the voting was not yet started for this job
            pub fn vote(&mut self, voting_id: VotingId, voting_type: VotingType, choice: Choice, stake: Balance);

            /// Finishes voting. Depending on type of voting, different actions are performed.
            /// [Read more](VotingEngine::finish_voting())
            ///
            /// # Events
            /// * [`VotingEnded`](crate::voting::voting_engine::events::VotingEnded)
            /// * [`BallotCast`](crate::voting::voting_engine::events::BallotCast) - when formal voting starts
            /// * [`Unstake`](crate::core_contracts::Unstake)
            /// * [`Stake`](crate::core_contracts::Stake)
            /// * [`Mint`](crate::core_contracts::Mint)
            ///
            /// # Errors
            /// * [`BidNotFound`](crate::utils::Error::BidNotFound) if the bid was not found
            /// * [`VotingDoesNotExist`](crate::utils::Error::VotingDoesNotExist) if the voting does not exist
            /// * [`VotingWithGivenTypeNotInProgress`](crate::utils::Error::VotingWithGivenTypeNotInProgress) if the voting
            /// is not in progress
            /// * [`FinishingCompletedVotingNotAllowed`](crate::utils::Error::FinishingCompletedVotingNotAllowed) if the
            /// voting is already completed
            pub fn finish_voting(&mut self, voting_id: VotingId, voting_type: VotingType) -> VotingSummary;

            /// Returns the total number of jobs.
            pub fn jobs_count(&self) -> u32;

            /// Returns a job with given [JobId].
            pub fn get_job(&self, job_id: JobId) -> Option<Job>;
        }

        to self.access_control {
            /// Changes the ownership of the contract. Transfers ownership to the `owner`.
            /// Only the current owner is permitted to call this method.
            /// [`Read more`](AccessControl::change_ownership())
            pub fn change_ownership(&mut self, owner: Address);

            /// Adds a new address to the whitelist.
            /// [`Read more`](AccessControl::add_to_whitelist())
            pub fn add_to_whitelist(&mut self, address: Address);

            /// Remove address from the whitelist.
            /// [`Read more`](AccessControl::remove_from_whitelist());
            pub fn remove_from_whitelist(&mut self, address: Address);

            /// Checks whether the given address is added to the whitelist.
            /// [`Read more`](AccessControl::is_whitelisted()).
            pub fn is_whitelisted(&self, address: Address) -> bool;

            /// Returns the address of the current owner.
            /// [`Read more`](AccessControl::get_owner()).
            pub fn get_owner(&self) -> Option<Address>;
        }
    }

    /// Constructor function.
    ///
    /// # Note
    /// Initializes contract elements:
    /// * Sets up the contract by saving addresses of [`Variable Repository`](crate::core_contracts::VariableRepositoryContract),
    /// [`Reputation Token`](crate::core_contracts::ReputationContract), [`VA Token`](crate::core_contracts::VaNftContract), [`KYC Token`](crate::core_contracts::KycNftContract).
    /// * Sets [`caller`] as the owner of the contract.
    /// * Adds [`caller`] to the whitelist.
    ///
    /// # Events
    /// Emits:
    /// * [`OwnerChanged`](crate::modules::owner::events::OwnerChanged),
    /// * [`AddedToWhitelist`](crate::modules::whitelist::events::AddedToWhitelist),
    #[odra(init)]
    pub fn init(
        &mut self,
        variable_repository: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    ) {
        self.refs.set_variable_repository(variable_repository);
        self.refs.set_reputation_token(reputation_token);
        self.refs.set_kyc_token(kyc_token);
        self.refs.set_va_token(va_token);
        self.access_control.init(caller());
    }

    /// Returns the CSPR balance of the contract.
    pub fn get_cspr_balance(&self) -> Balance {
        self_balance()
    }

    /// Erases the VA from the all bids, offers and jobs.
    /// Only a whitelisted account is permitted to call this method.
    /// Interacts with [Reputation Token Contract](crate::core_contracts::ReputationContract).
    ///
    /// # Errors
    /// * [crate::utils::Error::BidNotFound]
    /// * [crate::utils::Error::JobOfferNotFound]
    /// * [crate::utils::Error::CannotCancelBidOnCompletedJobOffer]
    /// * [crate::utils::Error::NotWhitelisted]
    ///
    /// # Events
    /// * [`BidEscrowSlashResults`](BidEscrowSlashResults)
    pub fn slash_voter(&mut self, voter: Address) {
        self.access_control.ensure_whitelisted();
        let (slashed_job_offers, slashed_bids) = self.bid_engine.slash_voter(voter);
        let (slashed_jobs, canceled_votings, affected_votings) = self.job_engine.slash_voter(voter);

        BidEscrowSlashResults {
            slashed_job_offers,
            slashed_bids,
            slashed_jobs,
            canceled_votings,
            affected_votings,
        }
        .emit();
    }
}
