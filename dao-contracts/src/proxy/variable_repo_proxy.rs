use casper_dao_utils::casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_dao_utils::{casper_dao_macros::Instance, Address};
use casper_dao_utils::{consts as dao_consts, math};
use casper_types::U256;

use crate::VariableRepositoryContractCaller;

#[derive(Instance)]
pub struct VariableRepoContractProxy {}

impl VariableRepoContractProxy {
    pub fn informal_voting_time(contract_address: Address) -> u64 {
        VariableRepoContractProxy::caller(contract_address)
            .get_variable(dao_consts::INFORMAL_VOTING_TIME)
    }

    pub fn formal_voting_time(contract_address: Address) -> u64 {
        VariableRepoContractProxy::caller(contract_address)
            .get_variable(dao_consts::FORMAL_VOTING_TIME)
    }

    pub fn minimum_governance_reputation(contract_address: Address) -> U256 {
        VariableRepoContractProxy::caller(contract_address)
            .get_variable(dao_consts::MINIMUM_GOVERNANCE_REPUTATION)
    }

    pub fn informal_voting_quorum(contract_address: Address) -> U256 {
        math::promils_of(
            4.into(),
            VariableRepoContractProxy::caller(contract_address)
                .get_variable(dao_consts::INFORMAL_VOTING_QUORUM),
        )
        .unwrap_or_revert()
    }

    pub fn formal_voting_quorum(contract_address: Address) -> U256 {
        math::promils_of(
            4.into(),
            VariableRepoContractProxy::caller(contract_address)
                .get_variable(dao_consts::FORMAL_VOTING_QUORUM),
        )
        .unwrap_or_revert()
    }

    fn caller(contract_address: Address) -> VariableRepositoryContractCaller {
        VariableRepositoryContractCaller::at(contract_address)
    }
}
