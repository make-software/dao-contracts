use crate::configuration::Configuration;
use crate::rules::validation::VotingValidation;
use crate::utils::Error;
use crate::voting::voting_engine::voting_state_machine::{VotingState, VotingStateMachine};
use macros::Rule;

/// Verifies voting is still in progress. May return [Error::VotingAlreadyFinished].
#[derive(Rule)]
pub struct VotingNotCompleted {}

impl VotingValidation for VotingNotCompleted {
    fn validate(
        &self,
        voting_state_machine: &VotingStateMachine,
        _configuration: &Configuration,
    ) -> Result<(), Error> {
        if voting_state_machine.state() == &VotingState::Finished {
            return Err(Error::VotingAlreadyFinished);
        }

        Ok(())
    }
}
