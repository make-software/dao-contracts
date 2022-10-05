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
    let token_address: Address = get_named_arg("token_address");
    let token_amount: U512 = get_named_arg("cspr_amount");
    let worker: Address = get_named_arg("worker");
    let document_hash: DocumentHash = get_named_arg("document_hash");
    let time: BlockTime = get_named_arg("time");
    let required_stake: Option<U256> = get_named_arg("required_stake");

    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, token_amount, None).unwrap_or_revert();

    BidEscrowContractCaller::at(token_address).pick_bid(
        worker,
        document_hash,
        time,
        required_stake,
        cargo_purse,
    );
}

fn main() {}
