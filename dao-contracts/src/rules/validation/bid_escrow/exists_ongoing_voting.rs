use casper_dao_utils::{casper_dao_macros::Rule, Error};

use crate::rules::validation::Validation;

/// Verifies if exists conflicting ongoing voting. May return [Error::VotingNotCompleted].
#[derive(Rule)]
pub struct ExistsOngoingVoting {
    is_ongoing_voting: bool,
}

impl Validation for ExistsOngoingVoting {
    fn validate(&self) -> Result<(), Error> {
        if self.is_ongoing_voting {
            return Err(Error::VotingNotCompleted);
        };

        Ok(())
    }
}
