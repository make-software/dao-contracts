//! Useful constants for common strings.
/// The name of `init` entry point.
pub const EP_INIT: &str = "init";
/// The name of `mint` entry point.
pub const EP_MINT: &str = "mint";
/// The name of `burn` entry point.
pub const EP_BURN: &str = "burn";
/// The name of `transfer_from` entry point.
pub const EP_TRANSFER_FROM: &str = "transfer_from";
/// The name of `stake` entry point.
pub const EP_STAKE: &str = "stake";
/// The name of `unstake` entry point.
pub const EP_UNSTAKE: &str = "unstake";
/// The name of `remove_from_whitelist` entry point.
pub const EP_REMOVE_FROM_WHITELIST: &str = "remove_from_whitelist";
/// The name of `add_to_whitelist` entry point.
pub const EP_ADD_TO_WHITELIST: &str = "add_to_whitelist";
/// The name of `change_ownership` entry point.
pub const EP_CHANGE_OWNERSHIP: &str = "change_ownership";
/// The name of `set_or_update` entry point.
pub const EP_SET_OR_UPDATE: &str = "set_or_update";
/// The name of `get` entry point.
pub const EP_GET: &str = "get";

/// The name of `recipient` entry point argument.
pub const ARG_RECIPIENT: &str = "recipient";
/// The name of `amount` entry point argument.
pub const ARG_AMOUNT: &str = "amount";
/// The name of `owner` entry point argument.
pub const ARG_OWNER: &str = "owner";
/// The name of `address` entry point argument.
pub const ARG_ADDRESS: &str = "address";
/// The name of `key` entry point argument.
pub const ARG_KEY: &str = "key";
/// The name of `value` entry point argument.
pub const ARG_VALUE: &str = "value";
/// The name of `to` entry point argument.
pub const ARG_TO: &str = "to";
/// The name of `token_id` entry point argument.
pub const ARG_TOKEN_ID: &str = "token_id";

/// The key the collection length is stored under.
pub const LENGTH_SUFFIX: &str = "_length";
/// The key the contracts' events are stored under.
pub const NAME_EVENTS: &str = "events";

// REPO KEYS
/// A DOS fee that the JobPoster needs to attach to the Post Job query.
/// The value is the minimum amount of Fiat currency to be attached as CSPR using [`FiatConversionRate`](FIAT_CONVERSION_RATE_ADDRESS).
pub const POST_JOB_DOS_FEE: &str = "PostJobDOSFee";
/// The time of the Internal Auction.
pub const INTERNAL_AUCTION_TIME: &str = "InternalAuctionTime";
/// The time of the External Auction.
pub const PUBLIC_AUCTION_TIME: &str = "PublicAuctionTime";
/// Defines how many [`Reputation tokens`]() are given to the `VA’s` for their community audit/vote on a work product.
pub const DEFAULT_POLICING_RATE: &str = "DefaultPolicingRate";
/// Defines how much `Reputation` is minted for each unit of currency paid for `Work`.
pub const REPUTATION_CONVERSION_RATE: &str = "ReputationConversionRate";
/// An address of a contract that will return the conversion rate between Fiat and CSPR.
pub const FIAT_CONVERSION_RATE_ADDRESS: &str = "FiatConversionRateAddress";
/// Defines if KYC is required to post on Forum.
pub const FORUM_KYC_REQUIRED: &str = "ForumKycRequired";
/// How many `VA’s` are needed for an informal voting quorum.
pub const BID_ESCROW_INFORMAL_QUORUM_RATIO: &str = "BidEscrowInformalQuorumRatio";
/// How many `VA’s` are needed for an formal voting quorum.
pub const BID_ESCROW_FORMAL_QUORUM_RATIO: &str = "BidEscrowFormalQuorumRatio";
/// Time for the formal part of the `Bid Escrow` voting.
pub const BID_ESCROW_FORMAL_VOTING_TIME: &str = "BidEscrowFormalVotingTime";
/// Time for the informal part of the `Bid Escrow` voting.
pub const BID_ESCROW_INFORMAL_VOTING_TIME: &str = "BidEscrowInformalVotingTime";
/// Time for the formal part of other votings
pub const FORMAL_VOTING_TIME: &str = "FormalVotingTime";
/// Time for the informal part of other votings
pub const INFORMAL_VOTING_TIME: &str = "InformalVotingTime";
/// How many `VA’s` are needed for an formal voting quorum.
pub const FORMAL_QUORUM_RATIO: &str = "FormalQuorumRatio";
/// How many `VA’s` are needed for an informal voting quorum.
pub const INFORMAL_QUORUM_RATIO: &str = "InformalQuorumRatio";
/// Tells if the Informal Voting should stake the reputation or only simulate it.
pub const INFORMAL_STAKE_REPUTATION: &str = "InformalStakeReputation";
/// Determines if the Payment for the Job should be distributed between all VA’s or only to those who voted.
pub const DISTRIBUTE_PAYMENT_TO_NON_VOTERS: &str = "DistributePaymentToNonVoters";
/// Time between Informal and Formal Votings.
pub const TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING: &str = "TimeBetweenInformalAndFormalVoting";
/// How much time the bid wait for the acceptance. After this time, the bid can be cancelled
pub const VA_BID_ACCEPTANCE_TIMEOUT: &str = "VABidAcceptanceTimeout";
/// Whether or not VA’s can take part in the `Public Auction` part of the `Bidding` process.
pub const VA_CAN_BID_ON_PUBLIC_AUCTION: &str = "VACanBidOnPublicAuction";
/// An address of a multisig wallet (GovernanceWallet) of the DAO.
pub const BID_ESCROW_WALLET_ADDRESS: &str = "BidEscrowWalletAddress";
/// How much CSPR is sent to GovernanceWallet after the Job is finished
pub const BID_ESCROW_PAYMENT_RATIO: &str = "BidEscrowPaymentRatio";
/// If the difference between 50/50 and result of the Informal Voting is bigger than the value, the time between votings should be doubled.
pub const VOTING_CLEARNESS_DELTA: &str = "VotingClearnessDelta";
/// Time between the worker job submission and the internal voting start.
pub const VOTING_START_AFTER_JOB_WORKER_SUBMISSION: &str = "VotingStartAfterJobSubmission";
/// How much reputation of an Internal Worker is slashed after not completing a Job.
pub const DEFAULT_REPUTATION_SLASH: &str = "DefaultReputationSlash";
/// An address of a contract that generates a next voting id.
pub const VOTING_IDS_ADDRESS: &str = "VotingIdsAddress";

// Contract keys.
pub const CONTRACT_MAIN_PURSE: &str = "__contract_main_purse";
