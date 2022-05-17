use casper_dao_utils::ContractCall;
use casper_types::U256;

use crate::{
    proxy::reputation_proxy::ReputationContractProxy,
    voting::{voting::VotingConfiguration, GovernanceVoting},
    VariableRepositoryContractCaller,
};

pub struct VotingConfigurationBuilder {
    voting_configuration: VotingConfiguration,
}

impl VotingConfigurationBuilder {
    pub fn with_defaults(voting: &GovernanceVoting) -> VotingConfigurationBuilder {
        let total_onboarded =
            ReputationContractProxy::total_onboarded(voting.get_reputation_token_address());
        VotingConfigurationBuilder {
            voting_configuration: VariableRepositoryContractCaller::at(
                voting.get_variable_repo_address(),
            )
            .voting_configuration_defaults(total_onboarded),
        }
    }

    pub fn with_contract_call(mut self, contract_call: ContractCall) -> VotingConfigurationBuilder {
        self.voting_configuration.contract_call = Some(contract_call);
        self
    }

    pub fn with_cast_first_vote(mut self, cast: bool) -> VotingConfigurationBuilder {
        self.voting_configuration.cast_first_vote = cast;
        self
    }

    pub fn with_create_minimum_reputation(
        mut self,
        minimum_reputation: U256,
    ) -> VotingConfigurationBuilder {
        self.voting_configuration.create_minimum_reputation = minimum_reputation;
        self
    }

    pub fn build(self) -> VotingConfiguration {
        self.voting_configuration
    }
}
