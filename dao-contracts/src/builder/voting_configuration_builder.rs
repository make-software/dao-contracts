use casper_dao_utils::ContractCall;
use casper_types::U256;

use crate::{
    voting::{voting::VotingConfiguration, GovernanceVoting},
    KycOwnedNftContractCaller, KycOwnedNftContractInterface, VariableRepositoryContractCaller,
};

pub struct VotingConfigurationBuilder {
    voting_configuration: VotingConfiguration,
}

impl VotingConfigurationBuilder {
    pub fn defaults(voting: &GovernanceVoting) -> VotingConfigurationBuilder {
        let total_onboarded =
            KycOwnedNftContractCaller::at(voting.get_va_token_address()).total_supply();
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

    pub fn cast_first_vote(mut self, cast: bool) -> VotingConfigurationBuilder {
        self.voting_configuration.cast_first_vote = cast;
        self
    }

    pub fn create_minimum_reputation(
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
