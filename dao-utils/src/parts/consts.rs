//! Useful constants for common strings.

pub const EP_INIT: &str = "init";
pub const EP_MINT: &str = "mint";
pub const EP_BURN: &str = "burn";
pub const EP_TRANSFER_FROM: &str = "transfer_from";
pub const EP_STAKE: &str = "stake";
pub const EP_UNSTAKE: &str = "unstake";
pub const EP_REMOVE_FROM_WHITELIST: &str = "remove_from_whitelist";
pub const EP_ADD_TO_WHITELIST: &str = "add_to_whitelist";
pub const EP_CHANGE_OWNERSHIP: &str = "change_ownership";
pub const EP_SET_OR_UPDATE: &str = "set_or_update";
pub const EP_GET: &str = "get";

pub const ARG_RECIPIENT: &str = "recipient";
pub const ARG_AMOUNT: &str = "amount";
pub const ARG_OWNER: &str = "owner";
pub const ARG_ADDRESS: &str = "address";
pub const ARG_KEY: &str = "key";
pub const ARG_VALUE: &str = "value";
pub const ARG_TO: &str = "to";
pub const ARG_TOKEN_ID: &str = "token_id";

pub const NAME_OWNER: &str = "owner";
pub const NAME_STAKES: &str = "stakes";
pub const NAME_TOTAL_SUPPLY: &str = "total_supply";
pub const NAME_BALANCES: &str = "balances";
pub const NAME_WHITELIST: &str = "whitelist";
pub const NAME_STORAGE: &str = "storage";
pub const NAME_KEYS: &str = "keys";
pub const LENGTH_SUFFIX: &str = "_length";
pub const NAME_EVENTS: &str = "events";

//REPO KEYS
pub const POST_JOB_DOS_FEE: &str = "PostJobDOSFee";
pub const INTERNAL_AUCTION_TIME: &str = "InternalAuctionTime";
pub const PUBLIC_AUCTION_TIME: &str = "PublicAuctionTime";
pub const DEFAULT_POLICING_RATE: &str = "DefaultPolicingRate";
pub const REPUTATION_CONVERSION_RATE: &str = "ReputationConversionRate";
pub const FIAT_CONVERSION_RATE_ADDRESS: &str = "FiatConversionRateAddress";
pub const FORUM_KYC_REQUIRED: &str = "ForumKycRequired";
pub const FORMAL_VOTING_QUORUM: &str = "FormalVotingQuorum";
pub const INFORMAL_VOTING_QUORUM: &str = "InformalVotingQuorum";
pub const VOTING_QUORUM: &str = "VotingQuorum";
pub const FORMAL_VOTING_TIME: &str = "FormalVotingTime";
pub const INFORMAL_VOTING_TIME: &str = "InformalVotingTime";
pub const VOTING_TIME: &str = "VotingTime";
pub const TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING: &str = "TimeBetweenInformalAndFormalVoting";
pub const VA_BID_ACCEPTANCE_TIMEOUT: &str = "VABidAcceptanceTimeout";
pub const VA_CAN_BID_ON_PUBLIC_AUCTION: &str = "VACanBidOnPublicAuction";
pub const GOVERNANCE_WALLET_ADDRESS: &str = "GovernanceWalletAddress";
pub const GOVERNANCE_PAYMENT_RATIO: &str = "GovernancePaymentRatio";
pub const JOB_SUBMIT_GRACE_PERIOD: &str = "JobSubmitGracePeriod";
pub const DEFAULT_REPUTATION_SLASH: &str = "DefaultReputationSlash";

// TODO: Remove
pub const MINIMUM_GOVERNANCE_REPUTATION: &str = "minimum_governance_reputation";
// TODO: Remove
pub const MINIMUM_VOTING_REPUTATION: &str = "minimum_voting_reputation";

// Contract keys.
pub const CONTRACT_MAIN_PURSE: &str = "__contract_main_purse";
