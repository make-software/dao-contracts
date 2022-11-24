use casper_dao_utils::{Address, ContractCall};

use crate::{voting::{GovernanceVoting}, KycNftContractCaller, KycNftContractInterface, VariableRepositoryContractCaller, DaoConfiguration, VariableRepositoryContractInterface, VaNftContractCaller, VaNftContractInterface};

pub struct VotingConfigurationBuilder {
    configuration: DaoConfiguration,
}

impl VotingConfigurationBuilder {
    pub fn defaults(variable_repo_address: Address, va_token_address: Address) -> VotingConfigurationBuilder {
        let total_onboarded =
            VaNftContractCaller::at(va_token_address).total_supply();
        let variables = VariableRepositoryContractCaller::at(
            variable_repo_address,
        ).all_variables();

        VotingConfigurationBuilder {
            configuration: DaoConfiguration {
                contract_call: None,
                only_va_can_create: true,
                unbounded_tokens_for_creator: false,
                onboard_creator: false,
            }
        }
    }

    pub fn contract_call(mut self, contract_call: ContractCall) -> VotingConfigurationBuilder {
        self.configuration.contract_call = Some(contract_call);
        self
    }

    pub fn only_va_can_create(mut self, only_va_can_create: bool) -> VotingConfigurationBuilder {
        self.configuration.only_va_can_create = only_va_can_create;
        self
    }

    pub fn unbounded_tokens_for_creator(
        mut self,
        unbounded_tokens_for_creator: bool,
    ) -> VotingConfigurationBuilder {
        self.configuration.unbounded_tokens_for_creator = unbounded_tokens_for_creator;
        self
    }

    pub fn onboard(mut self, onboard: bool) -> VotingConfigurationBuilder {
        self.configuration.onboard_creator = onboard;
        self
    }

    pub fn build(self) -> DaoConfiguration {
        self.configuration
    }
}
