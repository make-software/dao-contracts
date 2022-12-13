use casper_dao_utils::{casper_env::call_contract, Address};
use casper_types::RuntimeArgs;

use super::VotingId;

/// Calls a contract at `voting_ids_address` to generate a next voting id.
///
/// Reverts if the address is not a valid contract address, or contract call fails.
pub fn get_next_voting_id(voting_ids_address: Address) -> VotingId {
    call_contract(voting_ids_address, "next_voting_id", RuntimeArgs::new())
}
