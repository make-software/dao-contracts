use casper_dao_utils::{casper_dao_macros::Instance, Address};
use casper_types::U256;

use crate::{ReputationContractCaller, ReputationContractInterface};

#[derive(Instance)]
pub struct ReputationContractProxy {}

impl ReputationContractProxy {
    // pub fn balance_of(contract_address: Address, address: &Address) -> U256 {
    //     ReputationContractProxy::caller(contract_address).balance_of(*address)
    // }

    // pub fn has_reputation(contract_address: Address, address: &Address) -> bool {
    //     ReputationContractProxy::balance_of(contract_address, address) > U256::zero()
    // }

    pub fn total_onboarded(contract_address: Address) -> U256 {
        ReputationContractProxy::caller(contract_address).total_onboarded()
    }

    fn caller(contract_address: Address) -> ReputationContractCaller {
        ReputationContractCaller::at(contract_address)
    }

    pub fn transfer(contract_address: Address, from: Address, to: Address, amount: U256) {
        ReputationContractProxy::caller(contract_address).transfer_from(from, to, amount)
    }

    pub fn burn(contract_address: Address, owner: Address, amount: U256) {
        ReputationContractProxy::caller(contract_address).burn(owner, amount);
    }

    pub fn mint(contract_address: Address, recipient: Address, amount: U256) {
        ReputationContractProxy::caller(contract_address).mint(recipient, amount);
    }
}
