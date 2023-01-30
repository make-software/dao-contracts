use casper_dao_utils::{casper_dao_macros::Rule, BlockTime, Error};

use crate::{
    rules::validation::VotingValidation,
    voting::voting_state_machine::{VotingState, VotingStateMachine},
};

/// Verifies if a ballot is cast in the right time. May return [Error::InformalVotingNotStarted],
/// [Error::VotingDuringTimeBetweenVotingsNotAllowed] or [Error::VoteOnCompletedVotingNotAllowed].
#[derive(Rule)]
pub struct VoteInTime {
    block_time: BlockTime,
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
