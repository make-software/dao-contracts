use casper_dao_utils::ContractCall;

use crate::{
    voting::{voting::VotingConfiguration, GovernanceVoting},
    KycNftContractCaller,
    KycNftContractInterface,
    VariableRepositoryContractCaller,
};

pub struct VotingConfigurationBuilder {
    voting_configuration: VotingConfiguration,
}

impl VotingConfigurationBuilder {
    pub fn defaults(voting: &GovernanceVoting) -> VotingConfigurationBuilder {
        let total_onboarded =
            KycNftContractCaller::at(voting.get_va_token_address()).total_supply();
        VotingConfigurationBuilder {
            voting_configuration: VariableRepositoryContractCaller::at(
                voting.get_variable_repo_address(),
            )
            .voting_configuration_defaults(total_onboarded),
        }
    }

    pub fn contract_call(mut self, contract_call: ContractCall) -> VotingConfigurationBuilder {
        self.voting_configuration.contract_call = Some(contract_call);
        self
    }

    pub fn only_va_can_create(mut self, only_va_can_create: bool) -> VotingConfigurationBuilder {
        self.voting_configuration.only_va_can_create = only_va_can_create;
        self
    }

    pub fn unbounded_tokens_for_creator(
        mut self,
        unbounded_tokens_for_creator: bool,
    ) -> VotingConfigurationBuilder {
        self.voting_configuration.unbounded_tokens_for_creator = unbounded_tokens_for_creator;
        self
    }

    pub fn onboard(mut self, onboard: bool) -> VotingConfigurationBuilder {
        self.voting_configuration.onboard_creator = onboard;
        self
    }

    pub fn build(self) -> VotingConfiguration {
        self.voting_configuration
    }
}
