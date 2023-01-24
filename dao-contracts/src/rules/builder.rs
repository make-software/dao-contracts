use crate::rules::{
    validation::{Validation, VotingValidation},
    Rules,
};

/// A utility to build a set of validation rules.
pub struct RulesBuilder {
    rules: Rules,
}

impl RulesBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self {
            rules: Rules {
                validations: vec![],
                voting_validations: vec![],
            },
        }
    }

    /// Adds a generic validation rule.
    pub fn add_validation(mut self, validation: Box<dyn Validation>) -> RulesBuilder {
        self.rules.validations.push(validation);
        self
    }

    /// Adds a voting validation rule.
    pub fn add_voting_validation(mut self, validation: Box<dyn VotingValidation>) -> RulesBuilder {
        self.rules.voting_validations.push(validation);
        self
    }

    /// Builds a [Rules] struct.
    pub fn build(self) -> Rules {
        self.rules
    }
}

impl Default for RulesBuilder {
    fn default() -> Self {
        Self::new()
    }
}
