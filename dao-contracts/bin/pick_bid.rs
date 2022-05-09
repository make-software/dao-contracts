use casper_dao_contracts::{
    bid::types::Description, voting::ReputationAmount, BidEscrowContractCaller,
    BidEscrowContractInterface,
};
use casper_dao_utils::{
    casper_contract::{
        contract_api::{
            account::get_main_purse,
            runtime::get_named_arg,
            system::{create_purse, transfer_from_purse_to_purse},
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    Address, BlockTime,
};
use casper_types::{URef, U512};

#[no_mangle]
fn call() {
    let token_address: Address = get_named_arg("token_address");
    let token_amount: U512 = get_named_arg("cspr_amount");
    let worker: Address = get_named_arg("worker");
    let description: Description = get_named_arg("description");
    let time: BlockTime = get_named_arg("time");
    let required_stake: Option<ReputationAmount> = get_named_arg("required_stake");

    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, token_amount, None).unwrap_or_revert();

    BidEscrowContractCaller::at(token_address).pick_bid(
        worker,
        description,
        time,
        required_stake,
        cargo_purse,
    );
}

fn main() {}
