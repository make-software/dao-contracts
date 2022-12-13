use casper_dao_utils::{BlockTime, Error};

use crate::{
    rules::validation::VotingValidation,
    voting::voting_state_machine::{VotingState, VotingStateMachine},
};

pub struct AfterFormalVoting {
    pub block_time: BlockTime,
}

impl VotingValidation for AfterFormalVoting {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error> {
        if voting_state_machine.state_in_time(self.block_time) == VotingState::Finished {
            return Ok(());
        }

        Err(Error::FormalVotingNotCompleted)
    }
}

impl AfterFormalVoting {
    pub fn create(block_time: BlockTime) -> Box<AfterFormalVoting> {
        Box::new(Self { block_time })
    }
}
