use casper_dao_contracts::bid_escrow::{BidEscrowContractCaller, BidEscrowContractInterface};
use casper_dao_utils::{
    casper_contract::contract_api::{
        account::get_main_purse,
        runtime::get_named_arg,
        system::create_purse,
    },
    cspr::transfer_p2p,
    Address,
};
use casper_types::{URef, U512};

#[no_mangle]
fn call() {
    let bid_escrow_address: Address = get_named_arg("bid_escrow_address");
    let job_offer_id: u32 = get_named_arg("job_offer_id");
    let bid_id: u32 = get_named_arg("bid_id");
    let token_amount: U512 = get_named_arg("cspr_amount");

    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_p2p(main_purse, cargo_purse, token_amount);

    BidEscrowContractCaller::at(bid_escrow_address).pick_bid(job_offer_id, bid_id, cargo_purse);
}

fn main() {}
