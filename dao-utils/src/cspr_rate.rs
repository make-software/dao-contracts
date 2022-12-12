use casper_types::{RuntimeArgs, U512};

use crate::{casper_env::call_contract, Address};

pub fn convert_to_fiat(cspr: U512, fiat_conversion_rate_address: Address) -> U512 {
    let rate: U512 = call_contract(fiat_conversion_rate_address, "get_rate", RuntimeArgs::new());

    cspr.checked_div(rate).unwrap()
}
