use casper_dao_utils::casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_dao_utils::conversions::u512_to_u256;
use casper_dao_utils::{casper_dao_macros::Instance, Address};
use casper_dao_utils::{consts as dao_consts, math};
use casper_types::bytesrepr::FromBytes;
use casper_types::{U256, U512};

use crate::voting::voting::VotingConfiguration;
use crate::VariableRepositoryContractCaller;

#[derive(Instance)]
pub struct VariableRepoContractProxy {}

impl VariableRepoContractProxy {
    // pub fn informal_voting_time(contract_address: Address) -> u64 {
    //     VariableRepoContractProxy::get_variable(contract_address, dao_consts::INFORMAL_VOTING_TIME)
    // }

    // pub fn formal_voting_time(contract_address: Address) -> u64 {
    //     VariableRepoContractProxy::get_variable(contract_address, dao_consts::FORMAL_VOTING_TIME)
    // }

    // pub fn minimum_governance_reputation(contract_address: Address) -> U256 {
    //     VariableRepoContractProxy::get_variable(
    //         contract_address,
    //         dao_consts::MINIMUM_GOVERNANCE_REPUTATION,
    //     )
    // }

    // pub fn informal_voting_quorum(contract_address: Address, total_onboarded: U256) -> U256 {
    //     math::promils_of(
    //         total_onboarded,
    //         VariableRepoContractProxy::get_variable(
    //             contract_address,
    //             dao_consts::INFORMAL_VOTING_QUORUM,
    //         ),
    //     )
    //     .unwrap_or_revert()
    // }

    // pub fn formal_voting_quorum(contract_address: Address, total_onboarded: U256) -> U256 {
    //     math::promils_of(
    //         total_onboarded,
    //         VariableRepoContractProxy::get_variable(
    //             contract_address,
    //             dao_consts::FORMAL_VOTING_QUORUM,
    //         ),
    //     )
    //     .unwrap_or_revert()
    // }

    pub fn reputation_to_mint(contract_address: Address, cspr_amount: U512) -> U256 {
        math::promils_of(
            u512_to_u256(cspr_amount).unwrap_or_revert(),
            VariableRepoContractProxy::get_variable(
                contract_address,
                dao_consts::REPUTATION_CONVERSION_RATE,
            ),
        )
        .unwrap_or_revert()
    }

    pub fn reputation_to_redistribute(contract_address: Address, reputation_amount: U256) -> U256 {
        math::promils_of(
            reputation_amount,
            VariableRepoContractProxy::get_variable(
                contract_address,
                dao_consts::DEFAULT_POLICING_RATE,
            ),
        )
        .unwrap_or_revert()
    }

    fn caller(contract_address: Address) -> VariableRepositoryContractCaller {
        VariableRepositoryContractCaller::at(contract_address)
    }

    fn get_variable<V>(contract_address: Address, key: &str) -> V
    where
        V: FromBytes,
    {
        VariableRepoContractProxy::caller(contract_address).get_variable(key)
    }

    pub fn voting_configuration_defaults(
        contract_address: Address,
        total_onboarded: U256,
    ) -> VotingConfiguration {
        let caller = VariableRepoContractProxy::caller(contract_address);
        VotingConfiguration {
            formal_voting_quorum: caller.formal_voting_quorum(total_onboarded),
            formal_voting_time: caller.formal_voting_time(),
            informal_voting_quorum: Some(caller.informal_voting_quorum(total_onboarded)),
            informal_voting_time: Some(caller.informal_voting_time()),
            cast_first_vote: true,
            create_minimum_reputation: caller.minimum_governance_reputation(),
            cast_minimum_reputation: U256::zero(),
            contract_call: None,
        }
    }
}
