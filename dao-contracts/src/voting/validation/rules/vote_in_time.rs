use casper_dao_utils::{BlockTime, Error};

use crate::{
    rules::validation::VotingValidation,
    voting::voting_state_machine::{VotingState, VotingStateMachine},
};

pub struct VoteInTime {
    pub block_time: BlockTime,
}

impl VotingValidation for VoteInTime {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error> {
        if voting_state_machine.state_in_time(self.block_time) == VotingState::BetweenVotings {
            return Err(Error::VotingDuringTimeBetweenVotingsNotAllowed);
        }

        if voting_state_machine.state_in_time(self.block_time) == VotingState::Finished {
            return Err(Error::VoteOnCompletedVotingNotAllowed);
        }

        Ok(())
    }
}

impl VoteInTime {
    pub fn create(block_time: BlockTime) -> Box<VoteInTime> {
        Box::new(Self { block_time })
    }
}
