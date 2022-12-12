use casper_dao_utils::BlockTime;

use crate::{
    rules::{builder::RulesBuilder, Rules},
    voting::{
        validation::rules::{
            after_formal_voting::AfterFormalVoting,
            vote_in_time::VoteInTime,
            voting_not_completed::VotingNotCompleted,
        },
        voting_state_machine::VotingStateMachine,
    },
};

pub mod rules;

pub fn vote_validator(voting: &VotingStateMachine, block_time: BlockTime) -> Rules {
    let mut rules_builder = RulesBuilder::new();
    rules_builder.add_validation(Box::new(VoteInTime {
        voting_state: voting.state_in_time(block_time),
    }));
    rules_builder.build()
}

pub fn finish_formal_voting_validator(voting: &VotingStateMachine, block_time: BlockTime) -> Rules {
    let mut rules_builder = RulesBuilder::new();
    rules_builder.add_validation(Box::new(AfterFormalVoting {
        state_in_time: voting.state_in_time(block_time),
    }));
    rules_builder.add_validation(Box::new(VotingNotCompleted {
        voting_state: voting.state().clone(),
    }));
    rules_builder.build()
}
