//! Abstractions facilitating error handling.

use crate::rules::validation::{Validation, VotingValidation};
use odra::contract_env::revert;

mod builder;
pub mod validation;

use crate::configuration::Configuration;
use crate::voting::voting_engine::voting_state_machine::VotingStateMachine;
pub use builder::RulesBuilder;

/// A set of validation rules.
///
/// If any rule fail, the current contract execution stops, and reverts
/// if the error raised by the first falling rule.
///
/// Rules related to voting must be given voting state.
pub struct Rules {
    validations: Vec<Box<dyn Validation>>,
    voting_validations: Vec<Box<dyn VotingValidation>>,
}

impl Rules {
    /// Validates only the rules that don't need voting state.
    pub fn validate_generic_validations(&self) {
        for validation in &self.validations {
            if let Err(error) = validation.validate() {
                revert(error)
            }
        }
    }

    /// Validates only the rules that require voting state.
    pub fn validate_voting_validations(
        &self,
        voting_state_machine: &VotingStateMachine,
        configuration: &Configuration,
    ) {
        for validation in &self.voting_validations {
            if let Err(error) = validation.validate(voting_state_machine, configuration) {
                revert(error)
            }
        }
    }

    /// Validates all the rules.
    pub fn validate(
        &self,
        voting_state_machine: &VotingStateMachine,
        configuration: &Configuration,
    ) {
        self.validate_generic_validations();
        self.validate_voting_validations(voting_state_machine, configuration);
    }
}
