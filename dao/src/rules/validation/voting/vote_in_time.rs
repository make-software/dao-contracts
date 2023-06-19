use crate::configuration::Configuration;
use crate::rules::validation::VotingValidation;
use crate::utils::Error;
use crate::voting::voting_engine::voting_state_machine::{VotingState, VotingStateMachine};
use macros::Rule;
use odra::types::BlockTime;

/// Verifies if a ballot is cast in the right time. May return [Error::InformalVotingNotStarted],
/// [Error::VotingDuringTimeBetweenVotingsNotAllowed] or [Error::VoteOnCompletedVotingNotAllowed].
#[derive(Rule)]
pub struct VoteInTime {
    block_time: BlockTime,
}

impl VotingValidation for VoteInTime {
    fn validate(
        &self,
        voting_state_machine: &VotingStateMachine,
        configuration: &Configuration,
    ) -> Result<(), Error> {
        match voting_state_machine.state_in_time(self.block_time, configuration) {
            VotingState::Created => Err(Error::InformalVotingNotStarted),
            VotingState::BetweenVotings => Err(Error::VotingDuringTimeBetweenVotingsNotAllowed),
            VotingState::Finished => Err(Error::VoteOnCompletedVotingNotAllowed),
            _ => Ok(()),
        }
    }
}
