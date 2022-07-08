use casper_dao_erc20::{ERC20Caller, ERC20Interface};
use casper_dao_utils::{
    casper_contract::{
        contract_api::{
            account::get_main_purse,
            runtime,
            system::{create_purse, transfer_from_purse_to_purse},
        },
        unwrap_or_revert::UnwrapOrRevert,
    },
    Address,
};
use casper_types::{URef, U512};

#[no_mangle]
fn call() {
    let token_address: Address = runtime::get_named_arg("token_address");
    let token_amount: U512 = runtime::get_named_arg("cspr_amount");

    let main_purse: URef = get_main_purse();
    let cargo_purse: URef = create_purse();
    transfer_from_purse_to_purse(main_purse, cargo_purse, token_amount, None).unwrap_or_revert();

    ERC20Caller::at(token_address).deposit(cargo_purse);
}

fn main() {}
