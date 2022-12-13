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
        match voting_state_machine.state_in_time(self.block_time) {
            VotingState::Created => Err(Error::InformalVotingNotStarted),
            VotingState::BetweenVotings => Err(Error::VotingDuringTimeBetweenVotingsNotAllowed),
            VotingState::Finished => Err(Error::VoteOnCompletedVotingNotAllowed),
            _ => Ok(()),
        }
    }
}

impl VoteInTime {
    pub fn create(block_time: BlockTime) -> Box<VoteInTime> {
        Box::new(Self { block_time })
    }
}
