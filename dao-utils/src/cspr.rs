//! Functions for interacting with purses.
use casper_contract::{
    contract_api::system::{
        get_purse_balance,
        transfer_from_purse_to_account,
        transfer_from_purse_to_purse,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{URef, U512};

use crate::{
    casper_env::{self, revert},
    Address,
    Error,
};

/// Gets the balance of the currently executing contract main purse.
pub fn main_purse_balance() -> U512 {
    get_purse_balance(casper_env::contract_main_purse()).unwrap_or_default()
}

/// Transfers all the funds from the given `cargo_purse` to the currently executing contract main purse.
///
/// Reverts if the `cargo_purse` is empty or transfer from purse to purse fails.
pub fn deposit(cargo_purse: URef) -> U512 {
    let main_purse = casper_env::contract_main_purse();
    let amount = get_purse_balance(cargo_purse).unwrap_or_revert_with(Error::PurseError);

    if amount.is_zero() {
        revert(Error::CannotDepositZeroAmount);
    }

    transfer_p2p(cargo_purse, main_purse, amount);
    amount
}

/// Withdraws funds from the currently executing contract main purse to the given [`Address`].
///
/// Reverts if the `address` is invalid or transfer from purse to account fails.
pub fn withdraw(address: Address, amount: U512) {
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

/// Transfers all the funds from the given `from` purse to the `to` purse.
///
/// Reverts if the transfer from purse to purse fails.
pub fn transfer_p2p(from: URef, to: URef, amount: U512) {
    transfer_from_purse_to_purse(from, to, amount, None)
        .unwrap_or_revert_with(Error::TransferError);
}
