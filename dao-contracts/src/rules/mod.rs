use casper_dao_utils::casper_env::revert;

use crate::{
    rules::validation::{Validation, VotingValidation},
    voting::voting_state_machine::VotingStateMachine,
};

pub mod action;
pub mod builder;
pub mod validation;

pub struct Rules {
    pub validations: Vec<Box<dyn Validation>>,
    pub voting_validations: Vec<Box<dyn VotingValidation>>,
}

impl Rules {
    pub fn validate_validations(&self) {
        for validation in &self.validations {
            let result = validation.validate();
            if result.is_err() {
                revert(result.err().unwrap());
            }
        }
    }

    pub fn validate_voting_validations(&self, voting_state_machine: &VotingStateMachine) {
        for validation in &self.voting_validations {
            let result = validation.validate(voting_state_machine);
            if result.is_err() {
                revert(result.err().unwrap());
            }
        }
    }

    pub fn validate(&self, voting_state_machine: &VotingStateMachine) {
        self.validate_validations();
        self.validate_voting_validations(voting_state_machine);
    }
}