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
    DocumentHash,
};
use casper_types::{URef, U512};

#[no_mangle]
fn call() {
    let bid_escrow_address: Address = get_named_arg("bid_escrow_address");
    let job_id: u32 = get_named_arg("job_id");
    let proof: DocumentHash = get_named_arg("proof");
    let reputation_stake: U512 = get_named_arg("reputation_stake");
    let onboard: bool = get_named_arg("onboard");
    let cspr_amount: U512 = get_named_arg("cspr_amount");
    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, cspr_amount, None).unwrap_or_revert();

    BidEscrowContractCaller::at(bid_escrow_address).submit_job_proof_during_grace_period(
        job_id,
        proof,
        reputation_stake,
        onboard,
        Some(cargo_purse),
    );
}

fn main() {}
