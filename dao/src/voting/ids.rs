//! Voting ids helper function.
use crate::voting::types::VotingId;
use odra::call_contract;
use odra::types::{Address, CallArgs};

/// Calls a contract at `voting_ids_address` to generate a next voting id.
///
/// Reverts if the address is not a valid contract address, or contract call fails.
pub fn get_next_voting_id(voting_ids_address: Address) -> VotingId {
    call_contract(voting_ids_address, "next_voting_id", &CallArgs::new(), None)
}
