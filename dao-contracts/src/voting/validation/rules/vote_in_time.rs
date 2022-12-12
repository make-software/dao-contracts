use casper_dao_utils::Error;

use crate::{rules::validation::Validation, voting::voting_state_machine::VotingState};

pub struct VoteInTime {
    pub voting_state: VotingState,
}

impl Validation for VoteInTime {
    fn validate(&self) -> Result<(), Error> {
        if self.voting_state == VotingState::BetweenVotings {
            return Err(Error::VotingDuringTimeBetweenVotingsNotAllowed);
        }

        if self.voting_state == VotingState::Finished {
            return Err(Error::VoteOnCompletedVotingNotAllowed);
        }

        Ok(())
    }
}
