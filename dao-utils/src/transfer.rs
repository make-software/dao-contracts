use casper_contract::{contract_api::system::{get_purse_balance, transfer_from_purse_to_purse, transfer_from_purse_to_account}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U512};

use crate::{casper_env::{self, revert}, Address, Error};

pub fn deposit_cspr(cargo_purse: URef) -> U512 {
    let main_purse = casper_env::contract_main_purse();
    let amount = get_purse_balance(cargo_purse).unwrap_or_revert_with(Error::PurseError);

    if amount.is_zero() {
        revert(Error::CannotDepositZeroAmount);
    }

    transfer_from_purse_to_purse(cargo_purse, main_purse, amount, None)
        .unwrap_or_revert_with(Error::TransferError);
    amount
}

pub fn withdraw_cspr(address: Address, amount: U512) {
    let main_purse = casper_env::contract_main_purse();
    transfer_from_purse_to_account(
        main_purse,
        *address
            .as_account_hash()
            .unwrap_or_revert_with(Error::InvalidAddress),
        amount,
        None,
    )
    .unwrap_or_revert_with(Error::TransferError);
}
