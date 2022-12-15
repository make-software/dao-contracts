use crate::rules::{
    validation::{Validation, VotingValidation},
    Rules,
};

pub struct RulesBuilder {
    pub rules: Rules,
}

impl RulesBuilder {
    pub fn new() -> RulesBuilder {
        Self {
            rules: Rules {
                validations: vec![],
                voting_validations: vec![],
            },
        }
    }

    pub fn add_validation(mut self, validation: Box<dyn Validation>) -> RulesBuilder {
        self.rules.validations.push(validation);
        self
    }

    pub fn add_voting_validation(mut self, validation: Box<dyn VotingValidation>) -> RulesBuilder {
        self.rules.voting_validations.push(validation);
        self
    }

    pub fn build(self) -> Rules {
        self.rules
    }

    pub fn validate(self) {
        self.rules.validate_generic_validations();
    }
}

impl Default for RulesBuilder {
    fn default() -> Self {
        Self::new()
    }
}
