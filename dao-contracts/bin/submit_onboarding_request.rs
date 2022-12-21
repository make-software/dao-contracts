use casper_dao_contracts::{OnboardingRequestContractCaller, OnboardingRequestContractInterface};
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
    let onboarding_address: Address = get_named_arg("onboarding_address");
    let cspr_amount: U512 = get_named_arg("cspr_amount");
    let reason: DocumentHash = get_named_arg("reason");
    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, cspr_amount, None).unwrap_or_revert();

    OnboardingRequestContractCaller::at(onboarding_address)
        .submit_onboarding_request(reason, cargo_purse);
}

fn main() {}
