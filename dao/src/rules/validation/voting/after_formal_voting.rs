use crate::configuration::Configuration;
use crate::rules::validation::VotingValidation;
use crate::utils::Error;
use crate::voting::voting_engine::voting_state_machine::{VotingState, VotingStateMachine};
use macros::Rule;
use odra::types::BlockTime;

/// Verifies if the `Formal Voting` ended. May return [Error::FormalVotingNotCompleted].
#[derive(Rule)]
pub struct AfterFormalVoting {
    block_time: BlockTime,
}

impl VotingValidation for AfterFormalVoting {
    fn validate(
        &self,
        voting_state_machine: &VotingStateMachine,
        configuration: &Configuration,
    ) -> Result<(), Error> {
        if voting_state_machine.state_in_time(self.block_time, configuration)
            == VotingState::Finished
        {
            return Ok(());
        }

        Err(Error::FormalVotingNotCompleted)
    }
}
