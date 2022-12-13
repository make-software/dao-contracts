use casper_dao_utils::{BlockTime, Error};

use crate::{
    rules::validation::VotingValidation,
    voting::voting_state_machine::{VotingState, VotingStateMachine},
};

pub struct VotingNotCompleted {
    pub block_time: BlockTime,
}

impl VotingValidation for VotingNotCompleted {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error> {
        if voting_state_machine.state() == &VotingState::Finished {
            return Err(Error::VotingAlreadyFinished);
        }

        Ok(())
    }
}

impl VotingNotCompleted {
    pub fn create(block_time: BlockTime) -> Box<VotingNotCompleted> {
        Box::new(Self { block_time })
    }
}
