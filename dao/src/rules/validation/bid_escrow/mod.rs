//! Groups [Bid Escrow](crate::bid_escrow)-related validations.

mod can_be_onboarded;
mod can_bid_be_cancelled;
mod can_bid_on_auction_state;
mod can_bid_on_own_job;
mod can_job_offer_be_cancelled;
mod can_pick_bid;
mod can_progress_job_offer;
mod does_proposed_payment_exceed_budget;
mod does_proposed_payment_match_transferred;
mod exists_ongoing_voting;
mod has_permissions_to_cancel_bid;
mod has_permissions_to_cancel_job_offer;
mod is_dos_fee_enough;
mod is_grace_period;
mod is_not_va;
mod is_stake_non_zero;

pub use can_be_onboarded::CanBeOnboarded;
pub use can_bid_be_cancelled::CanBidBeCancelled;
pub use can_bid_on_auction_state::CanBidOnAuctionState;
pub use can_bid_on_own_job::CanBidOnOwnJob;
pub use can_job_offer_be_cancelled::CanJobOfferBeCancelled;
pub use can_pick_bid::CanPickBid;
pub use can_progress_job_offer::CanProgressJobOffer;
pub use does_proposed_payment_exceed_budget::DoesProposedPaymentExceedBudget;
pub use does_proposed_payment_match_transferred::DoesProposedPaymentMatchTransferred;
pub use exists_ongoing_voting::ExistsOngoingVoting;
pub use has_permissions_to_cancel_bid::HasPermissionsToCancelBid;
pub use has_permissions_to_cancel_job_offer::HasPermissionsToCancelJobOffer;
pub use is_dos_fee_enough::IsDosFeeEnough;
pub use is_grace_period::IsGracePeriod;
pub use is_not_va::IsNotVa;
pub use is_stake_non_zero::IsStakeNonZero;
