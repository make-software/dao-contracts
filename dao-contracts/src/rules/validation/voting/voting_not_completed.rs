use casper_dao_utils::{Error, casper_dao_macros::Rule};

use crate::{
    rules::validation::VotingValidation,
    voting::voting_state_machine::{VotingState, VotingStateMachine},
};

/// Verifies voting is still in progress. May return [Error::VotingAlreadyFinished].
#[derive(Rule)]
pub struct VotingNotCompleted {}

impl VotingValidation for VotingNotCompleted {
    fn validate(&self, voting_state_machine: &VotingStateMachine) -> Result<(), Error> {
        if voting_state_machine.state() == &VotingState::Finished {
            return Err(Error::VotingAlreadyFinished);
        }

        Ok(())
    }
}
