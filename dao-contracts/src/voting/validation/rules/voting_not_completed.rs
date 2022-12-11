use casper_dao_utils::Error;

use crate::{rules::validation::Validation, voting::voting_state_machine::VotingState};

pub struct VotingNotCompleted {
    pub voting_state: VotingState,
}

impl Validation for VotingNotCompleted {
    fn validate(&self) -> Result<(), Error> {
        if self.voting_state == VotingState::Finished {
            return Err(Error::VotingAlreadyFinished);
        }

        Ok(())
    }
}
