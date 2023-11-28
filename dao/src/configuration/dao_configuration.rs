use odra::types::{Address, Balance, BlockTime};
use odra::OdraType;

/// A serializable data structure that represents DAO configuration.
#[derive(OdraType)]
pub struct DaoConfiguration {
    pub post_job_dos_fee: Balance,
    pub internal_auction_time: BlockTime,
    pub public_auction_time: BlockTime,
    pub default_policing_rate: Balance,
    pub reputation_conversion_rate: Balance,
    pub fiat_conversion_rate_address: Address,
    pub forum_kyc_required: bool,
    pub bid_escrow_informal_quorum_ratio: Balance,
    pub bid_escrow_formal_quorum_ratio: Balance,
    pub informal_quorum_ratio: Balance,
    pub formal_quorum_ratio: Balance,
    pub bid_escrow_informal_voting_time: BlockTime,
    pub bid_escrow_formal_voting_time: BlockTime,
    pub informal_voting_time: BlockTime,
    pub formal_voting_time: BlockTime,
    pub informal_stake_reputation: bool,
    pub time_between_informal_and_formal_voting: BlockTime,
    pub va_bid_acceptance_timeout: BlockTime,
    pub va_can_bid_on_public_auction: bool,
    pub distribute_payment_to_non_voters: bool,
    pub bid_escrow_wallet_address: Address,
    pub default_reputation_slash: Balance,
    pub voting_clearness_delta: Balance,
    pub voting_start_after_job_worker_submission: BlockTime,
    pub bid_escrow_payment_ratio: Balance,
    pub voting_ids_address: Address,
    pub cancel_finished_voting_timeout: BlockTime,
}
