use casper_dao_utils::{casper_dao_macros::Rule, BlockTime, Error};

use crate::{
    rules::validation::VotingValidation,
    voting::voting_state_machine::{VotingState, VotingStateMachine},
};

/// Verifies if the `Formal Voting` ended. May return [Error::FormalVotingNotCompleted].
#[derive(Rule)]
pub struct AfterFormalVoting {
    block_time: BlockTime,
}

impl VotingValidation for AfterFormalVoting {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error> {
        if voting_state_machine.state_in_time(self.block_time) == VotingState::Finished {
            return Ok(());
        }

        Err(Error::FormalVotingNotCompleted)
    }
}
