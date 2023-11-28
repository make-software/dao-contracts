use crate::configuration::Configuration;
use crate::rules::validation::VotingValidation;
use crate::utils::Error;
use crate::voting::voting_engine::voting_state_machine::{VotingState, VotingStateMachine};
use macros::Rule;
use odra::types::BlockTime;

/// Verifies if the voting can be cancelled. May return [Error::FinishingCompletedVotingNotAllowed].
#[derive(Rule)]
pub struct FinishedVotingCanBeCancelled {
    block_time: BlockTime,
}

impl VotingValidation for FinishedVotingCanBeCancelled {
    fn validate(
        &self,
        voting_state_machine: &VotingStateMachine,
        configuration: &Configuration,
    ) -> Result<(), Error> {
        // shorthand for checking if block_time > voting_end_time + cancel_finished_voting_timeout
        if voting_state_machine.state_in_time(
            self.block_time - configuration.cancel_finished_voting_timeout(),
            configuration,
        ) == VotingState::Finished
        {
            return Ok(());
        }

        Err(Error::VotingCannotBeCancelledYet)
    }
}
