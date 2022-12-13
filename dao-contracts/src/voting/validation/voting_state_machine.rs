use casper_dao_utils::BlockTime;

use crate::{
    rules::builder::RulesBuilder,
    voting::{
        validation::rules::{
            after_formal_voting::AfterFormalVoting,
            vote_in_time::VoteInTime,
            voting_not_completed::VotingNotCompleted,
        },
        voting_state_machine::VotingStateMachine,
    },
};

impl VotingStateMachine {
    pub fn guard_vote(&self, block_time: BlockTime) {
        RulesBuilder::new()
            .add_voting_validation(VoteInTime::create(block_time))
            .build()
            .validate(self);
    }

    pub fn guard_finish_formal_voting(&self, block_time: BlockTime) {
        RulesBuilder::new()
            .add_voting_validation(AfterFormalVoting::create(block_time))
            .add_voting_validation(VotingNotCompleted::create(block_time))
            .build()
            .validate(self);
    }
}
