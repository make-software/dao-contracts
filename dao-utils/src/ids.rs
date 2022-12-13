use casper_types::{RuntimeArgs, U512};

use crate::{casper_env::call_contract, Address};

pub fn get_next_voting_id(voting_ids_address: Address) -> U512 {
    call_contract(voting_ids_address, "next_voting_id", RuntimeArgs::new())
}
