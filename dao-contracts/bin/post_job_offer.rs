use casper_dao_contracts::bid_escrow::{BidEscrowContractCaller, BidEscrowContractInterface};
use casper_dao_utils::{
    casper_contract::contract_api::{
        account::get_main_purse,
        runtime::get_named_arg,
        system::create_purse,
    },
    cspr::transfer_p2p,
    Address,
    BlockTime,
};
use casper_types::{URef, U512};

#[no_mangle]
fn call() {
    let bid_escrow_address: Address = get_named_arg("bid_escrow_address");
    let token_amount: U512 = get_named_arg("cspr_amount");
    let expected_timeframe: BlockTime = get_named_arg("expected_timeframe");
    let budget: U512 = get_named_arg("budget");

    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_p2p(main_purse, cargo_purse, token_amount);

    BidEscrowContractCaller::at(bid_escrow_address).post_job_offer(
        expected_timeframe,
        budget,
        cargo_purse,
    );
}

fn main() {}
