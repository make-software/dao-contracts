use crate::rules::{validation::Validation, Rules};

pub struct RulesBuilder {
    pub rules: Rules,
}

impl RulesBuilder {
    pub fn new() -> Self {
        Self {
            rules: Rules {
                validations: vec![],
            },
        }
    }

    pub fn add_validation<'a>(&mut self, validation: Box<dyn Validation>) -> &Self {
        self.rules.validations.push(validation);
        self
    }

    pub fn build(self) -> Rules {
        self.rules
    }
}
