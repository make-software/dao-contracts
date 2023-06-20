//! CSPR redistribution helper functions.
use crate::configuration::Configuration;
use crate::modules::refs::ContractRefs;
use odra::contract_env::transfer_tokens;
use odra::types::{Address, Balance};

/// Transfers CSPRs to all VAs'. Each VA gets the amount of CSPR proportionally to their reputation.
///
/// Interacts with [`Reputation Token Contract`](crate::core_contracts::ReputationContract) to get balances information.
pub fn redistribute_cspr_to_all_vas(to_redistribute: Balance, refs: &ContractRefs) {
    let all_balances = refs.reputation_token().all_balances();
    let total_supply = all_balances.total_supply();
    for (address, balance) in all_balances.balances() {
        let amount = to_redistribute * *balance / total_supply;
        transfer_tokens(address, amount);
    }
}

/// Transfers some part of a given amount to `Bid Escrow Wallet` and returns the remaining amount.
///
/// See [`Configuration::bid_escrow_wallet_address()`](Configuration::bid_escrow_wallet_address()).
pub fn redistribute_to_governance(amount: Balance, configuration: &Configuration) -> Balance {
    let governance_wallet: Address = configuration.bid_escrow_wallet_address();
    let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(amount);
    transfer_tokens(&governance_wallet, governance_wallet_payment);
    amount - governance_wallet_payment
}
