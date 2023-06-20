use crate::bid_escrow::events::{CSPRTransfer, TransferReason};
use odra::contract_env::{self_address, transfer_tokens};
use odra::types::{event::OdraEvent, Address, Balance};

/// Withdraws CSPR from the contract and emits a corresponding event.
pub fn withdraw(to: &Address, amount: Balance, reason: TransferReason) {
    transfer_tokens(to, amount);

    CSPRTransfer {
        from: self_address(),
        to: *to,
        amount,
        reason: reason.to_string(),
    }
    .emit();
}
