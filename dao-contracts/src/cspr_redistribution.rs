use casper_dao_utils::{cspr, Address};
use casper_types::U512;

use crate::{config::Configuration, reputation::ReputationContractInterface, voting::refs::ContractRefs};

/// TODO: docs
pub fn redistribute_cspr_to_all_vas(to_redistribute: U512, refs: &dyn ContractRefs) {
    let all_balances = refs.reputation_token().all_balances();
    let total_supply = all_balances.total_supply();

    for (address, balance) in all_balances.balances() {
        let amount = to_redistribute * balance / total_supply;
        cspr::withdraw(*address, amount);
    }
}

/// TODO: docs
pub fn redistribute_to_governance(amount: U512, configuration: &Configuration) -> U512 {
    let governance_wallet: Address = configuration.bid_escrow_wallet_address();
    let governance_wallet_payment = configuration.apply_bid_escrow_payment_ratio_to(amount);
    cspr::withdraw(governance_wallet, governance_wallet_payment);

    amount - governance_wallet_payment
}
