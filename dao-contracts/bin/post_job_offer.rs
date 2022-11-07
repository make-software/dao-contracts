use casper_dao_contracts::{BidEscrowContractCaller, BidEscrowContractInterface};
use casper_dao_utils::{
    casper_contract::{
        contract_api::{
            account::get_main_purse,
            runtime::get_named_arg,
            system::{create_purse, transfer_from_purse_to_purse},
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    Address, BlockTime, DocumentHash,
};
use casper_types::{URef, U256, U512};

#[no_mangle]
fn call() {
    let bid_escrow_address: Address = get_named_arg("bid_escrow_address");
    let token_amount: U512 = get_named_arg("cspr_amount");
    let expected_timeframe: BlockTime = get_named_arg("expected_timeframe");
    let budget: U512 = get_named_arg("budget");

    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, token_amount, None).unwrap_or_revert();

    BidEscrowContractCaller::at(bid_escrow_address).post_job_offer(
        expected_timeframe,
        budget,
        cargo_purse,
    );
}

fn main() {}
