use casper_dao_utils::{BlockTime, Error};

use crate::{
    rules::validation::Validation,
    voting::voting::{Voting, VotingState},
};

pub struct AfterFormalVoting {
    pub state_in_time: VotingState,
}

impl Validation for AfterFormalVoting {
    fn validate(&self) -> Result<(), Error> {
        if self.state_in_time == VotingState::Finished {
            return Ok(());
        }

        Err(Error::FormalVotingNotCompleted)
    }
}
