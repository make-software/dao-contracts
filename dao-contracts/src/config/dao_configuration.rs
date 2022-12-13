use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    BlockTime,
};
use casper_types::U512;

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct DaoConfiguration {
    pub post_job_dos_fee: U512,
    pub internal_auction_time: BlockTime,
    pub public_auction_time: BlockTime,
    pub default_policing_rate: U512,
    pub reputation_conversion_rate: U512,
    pub fiat_conversion_rate_address: Address,
    pub forum_kyc_required: bool,
    pub bid_escrow_informal_quorum_ratio: U512,
    pub bid_escrow_formal_quorum_ratio: U512,
    pub informal_quorum_ratio: U512,
    pub formal_quorum_ratio: U512,
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
    pub default_reputation_slash: U512,
    pub voting_clearness_delta: U512,
    pub voting_start_after_job_worker_submission: BlockTime,
    pub bid_escrow_payment_ratio: U512,
}
