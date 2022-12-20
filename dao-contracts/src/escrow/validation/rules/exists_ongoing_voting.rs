use casper_dao_utils::Error;

use crate::rules::validation::Validation;

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

impl ExistsOngoingVoting {
    pub fn create(is_ongoing_voting: bool) -> Box<ExistsOngoingVoting> {
        Box::new(Self { is_ongoing_voting })
    }
}
