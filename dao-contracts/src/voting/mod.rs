//! Voting utilities.
//!
//! # Choice
//! Voting is a binary choice. Each voter may vote:
//! * In favor - `yes` vote
//! * Against - `no` vote.
//!
//! # Calculating Quorum
//! During both types of `Voting` a specific amount of votes is required to be cast - the `Quorum`.
//! The proportional amount of votes is defined in the configuration:
//! * [Informal Quorum Ratio](crate::config::Configuration::informal_voting_quorum()),
//! * [Formal Quorum Ratio](crate::config::Configuration::formal_voting_quorum()).
//!
//! To calculate the amount of votes required to achieve quorum we use the following formula:
//!
//! `quorum = total amount of VAs' * QuorumRatio`
//!
//! For example, with 11 VA’s and `QuorumRatio` set to 0.5, the quorum is 5.5, which means that
//! there need to be at least 6 votes cast for the Voting.
//!
//! # Informal Voting
//! The Informal Voting is the first phase of the Voting process. Its parameters are configurable:
//! * [Informal Voting Time](crate::config::Configuration::informal_voting_time()) - how long the voting lasts,
//! * [Informal Quorum Ratio](crate::config::Configuration::informal_voting_quorum()) - how many VA votes are needed,
//! * [Informal Stake Reputation](crate::config::Configuration::informal_stake_reputation()) - if the `Reputation`
//! used for `Informal Voting` should be staked or not.
//!
//! The first vote in the `Informal Voting` may be casted automatically at creation time
//! as a `yes` depending on [configuration](crate::config::Configuration::should_cast_first_vote()).
//!
//! ## Voting passed
//! When `Informal Voting` passes, following things happen:
//! * The [time between Votes](crate::config::Configuration::time_between_informal_and_formal_voting()) starts to count,
//! * Creator's stake is used as a `yes` vote for the first vote in the `Formal Voting`,
//! * `VAs'` stakes are returned to them.
//!
//! ## Voting failed
//! When Informal Voting fails, following things happen:
//! * The [time between Votes](crate::config::Configuration::time_between_informal_and_formal_voting()) starts to count,
//! * Creator's stake is used as a `yes` vote for the first vote in the `Formal Voting`,
//! * `VAs'` stakes are returned to them.
//!
//! ## Quorum not reached
//! When the Quorum is not reached during the `Informal Voting`, following things happen:
//! * The process ends here,
//! * `VAs'` stakes are returned to them,
//!
//! # Time between Votes
//! After passing or failing `Informal Voting`, there is a [certain amount of time](crate::config::Configuration::time_between_informal_and_formal_voting())
//! before the `Formal Voting` starts.
//! However, depending on the vote difference in the `Informal Voting` this time can be [doubled](crate::config::Configuration::double_time_between_votings()).
//!
//! # Formal Voting
//! The Formal Voting is the final step in the Voting process. Its parameters are configured using governance variables:
//! * [Voting Time](crate::config::Configuration::formal_voting_time()) - how long the voting lasts,
//! * [Quorum Ratio](crate::config::Configuration::formal_voting_quorum()) - how many VA votes are needed,
//!
//! Different actions are performed by the contract depending on the result.
//!
//! ## Voting passed
//! Besides yielding a positive result, the `Voting` passed means that the Reputation staked by the losing side is
//! redistributed between the winning side proportionally to the amount of reputation staked in the voting.
//!
//! ## Voting failed
//! Besides yielding a negative result, the `Voting` passed means that the Reputation staked by the losing side is
//! redistributed between the winning side proportionally to the amount of reputation staked in the voting.
//!
//! ## Quorum not reached
//! When the Quorum is not reached during the `Formal Voting`, following things happen:
//! * The process ends here.
//! * `VAs'` stakes are returned to them.
mod ballot;
mod cspr_redistribution;
mod ids;
mod kyc_info;
mod onboarding_info;
pub mod refs;
mod types;
mod voting_engine;

pub use ballot::{Ballot, Choice, ShortenedBallot};
pub use cspr_redistribution::{redistribute_cspr_to_all_vas, redistribute_to_governance};
pub use types::VotingId;
pub use voting_engine::{events, voting_state_machine, VotingEngine};

/// Voting utility submodules.
pub mod submodules {
    pub use super::{kyc_info::KycInfo, onboarding_info::OnboardingInfo};
}
