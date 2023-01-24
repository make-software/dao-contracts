use casper_dao_contracts::bid_escrow::{BidEscrowContractCaller, BidEscrowContractInterface};
use casper_dao_utils::{
    casper_contract::{
        contract_api::{
            account::get_main_purse,
            runtime::get_named_arg,
            system::{create_purse, transfer_from_purse_to_purse},
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    Address,
    BlockTime,
    Error,
};
use casper_types::{URef, U512};

#[no_mangle]
fn call() {
    let bid_escrow_address: Address = get_named_arg("bid_escrow_address");
    let job_offer_id: u32 = get_named_arg("job_offer_id");
    let time: BlockTime = get_named_arg("time");
    let payment: U512 = get_named_arg("payment");
    let reputation_stake: U512 = get_named_arg("reputation_stake");
    let onboard: bool = get_named_arg("onboard");
    let cspr_amount: U512 = get_named_arg("cspr_amount");
    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, cspr_amount, None)
        .unwrap_or_revert_with(Error::TransferError);

    BidEscrowContractCaller::at(bid_escrow_address).submit_bid(
        job_offer_id,
        time,
        payment,
        reputation_stake,
        onboard,
        Some(cargo_purse),
    );
}

fn main() {}
