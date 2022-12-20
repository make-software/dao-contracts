use std::collections::BTreeMap;

use casper_dao_utils::{casper_dao_macros::Event, Address};
use casper_types::U512;

use crate::{
    voting::{ballot::Choice, types::VotingId, Ballot},
    Configuration,
};

/// Event thrown after voting contract is created
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingContractCreated {
    pub voter_contract: Address,
    pub variable_repo: Address,
    pub reputation_token: Address,
}

/// Event thrown after ballot is cast
#[derive(Debug, PartialEq, Eq, Event)]
pub struct BallotCast {
    pub voter: Address,
    pub voting_id: VotingId,
    pub choice: Choice,
    pub stake: U512,
}

impl BallotCast {
    pub fn new(ballot: &Ballot) -> Self {
        BallotCast {
            voter: ballot.voter,
            voting_id: ballot.voting_id,
            choice: ballot.choice,
            stake: ballot.stake,
        }
    }
}

/// Event thrown after voting is created
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingCreated {
    pub creator: Address,
    pub voting_id: VotingId,
    pub informal_voting_id: VotingId,
    pub formal_voting_id: Option<VotingId>,
    pub config_formal_voting_quorum: u32,
    pub config_formal_voting_time: u64,
    pub config_informal_voting_quorum: u32,
    pub config_informal_voting_time: u64,
}

impl VotingCreated {
    pub fn new(
        creator: &Address,
        voting_id: VotingId,
        informal_voting_id: VotingId,
        formal_voting_id: Option<VotingId>,
        config: &Configuration,
    ) -> Self {
        VotingCreated {
            creator: *creator,
            voting_id,
            informal_voting_id,
            formal_voting_id,
            config_formal_voting_quorum: config.formal_voting_quorum(),
            config_formal_voting_time: config.formal_voting_time(),
            config_informal_voting_quorum: config.informal_voting_quorum(),
            config_informal_voting_time: config.informal_voting_time(),
        }
    }
}

/// Event thrown when voting ends
#[derive(Debug, PartialEq, Eq, Event)]
pub struct VotingEnded {
    pub voting_id: VotingId,
    pub informal_voting_id: VotingId,
    pub formal_voting_id: Option<VotingId>,
    pub result: String,
    pub votes_count: U512,
    pub stake_in_favor: U512,
    pub stake_against: U512,
    pub transfers: BTreeMap<Address, U512>,
    pub burns: BTreeMap<Address, U512>,
    pub mints: BTreeMap<Address, U512>,
}

// Build based on joboffer
struct JobOfferPostedInfo {
    // config
    // cspr_transfer
}

struct JobOfferCanceledInfo {
    // job_offer_id
    // unstakes
    // cspr_transfers
}

// Build based on bid
struct BidSubmittedInfo {
    // cspr_transfer
    // stakes
}

// Build based on bid
struct BidCanceledInfo {
    // cspr_transfer
    // stakes
}

struct BidPicked {
    // job_offer_id
    // bid_id
    // job
    // unstakes
    // cspr_transfers
}

struct JobCanceled {
    // job_id
    // cspr_transfer
    // unstakes
    // burns (worker - slash, stake)
}

struct JobProofSubmittedInfo {
    // voting_created_info
    // job changed info
}

struct JobProofSubmittedWithGrace {
    // old_job canceled
    // new_bid
    // new_job (bid_picked)
    // job_proof_submitterd
}

struct VotingCreatedInfo2 {
    // voting_state
    // stakes (account, amount, reason)
}

struct BallotCastInfo {
    // voting_id
    // ballot
    // stakes (account, amount, reason)
}

struct VotingFinishedInfo {
    // voting_summary,
    // unstakes,
    // stakes
    // minty (account, amount, reason)
    // burny (account, amount, reason)
    // cspr_transfer (account, amount, reason)
}

struct SingleBallotSlashed {
    // voting_id
    // unstakes
}

struct VotingCreatorSlashed {
    // voting_id
    // crator,
    // unstakes
}
