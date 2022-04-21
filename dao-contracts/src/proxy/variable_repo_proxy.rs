use casper_dao_utils::casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_dao_utils::{casper_dao_macros::Instance, Address};
use casper_dao_utils::{consts as dao_consts, math};
use casper_types::bytesrepr::FromBytes;
use casper_types::U256;

use crate::VariableRepositoryContractCaller;

#[derive(Instance)]
pub struct VariableRepoContractProxy {}

impl VariableRepoContractProxy {
    pub fn informal_voting_time(contract_address: Address) -> u64 {
        VariableRepoContractProxy::get_variable(contract_address, dao_consts::INFORMAL_VOTING_TIME)
    }

    pub fn formal_voting_time(contract_address: Address) -> u64 {
        VariableRepoContractProxy::get_variable(contract_address, dao_consts::FORMAL_VOTING_TIME)
    }

    pub fn minimum_governance_reputation(contract_address: Address) -> U256 {
        VariableRepoContractProxy::get_variable(
            contract_address,
            dao_consts::MINIMUM_GOVERNANCE_REPUTATION,
        )
    }

    pub fn informal_voting_quorum(contract_address: Address, total_onboarded: U256) -> U256 {
        math::promils_of(
            total_onboarded,
            VariableRepoContractProxy::get_variable(
                contract_address,
                dao_consts::INFORMAL_VOTING_QUORUM,
            ),
        )
        .unwrap_or_revert()
    }

    pub fn formal_voting_quorum(contract_address: Address, total_onboarded: U256) -> U256 {
        math::promils_of(
            total_onboarded,
            VariableRepoContractProxy::get_variable(
                contract_address,
                dao_consts::FORMAL_VOTING_QUORUM,
            ),
        )
        .unwrap_or_revert()
    }

    fn get_variable<V>(contract_address: Address, key: &str) -> V
    where
        V: FromBytes,
    {
        let caller = VariableRepositoryContractCaller::at(contract_address);
        caller.get_variable(key)
    }
}
