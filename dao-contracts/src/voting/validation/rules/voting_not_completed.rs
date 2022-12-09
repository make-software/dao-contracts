use casper_dao_utils::{BlockTime, Error};

use crate::{
    rules::validation::Validation,
    voting::voting::{Voting, VotingState, VotingType},
};

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
