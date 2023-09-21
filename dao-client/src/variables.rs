use dao::utils::*;

pub enum VariableType {
    Balance,
    BlockTime,
    Address,
    Bool,
    Unknown,
}

impl VariableType {
    pub fn from_str(name: &str) -> VariableType {
        match name {
            POST_JOB_DOS_FEE
            | DEFAULT_POLICING_RATE
            | REPUTATION_CONVERSION_RATE
            | BID_ESCROW_INFORMAL_QUORUM_RATIO
            | BID_ESCROW_FORMAL_QUORUM_RATIO
            | INFORMAL_QUORUM_RATIO
            | FORMAL_QUORUM_RATIO
            | DEFAULT_REPUTATION_SLASH
            | VOTING_CLEARNESS_DELTA
            | BID_ESCROW_PAYMENT_RATIO => VariableType::Balance,
            INTERNAL_AUCTION_TIME
            | PUBLIC_AUCTION_TIME
            | BID_ESCROW_INFORMAL_VOTING_TIME
            | BID_ESCROW_FORMAL_VOTING_TIME
            | INFORMAL_VOTING_TIME
            | FORMAL_VOTING_TIME
            | TIME_BETWEEN_INFORMAL_AND_FORMAL_VOTING
            | VA_BID_ACCEPTANCE_TIMEOUT
            | VOTING_START_AFTER_JOB_WORKER_SUBMISSION => VariableType::BlockTime,
            FIAT_CONVERSION_RATE_ADDRESS | BID_ESCROW_WALLET_ADDRESS | VOTING_IDS_ADDRESS => {
                VariableType::Address
            }
            FORUM_KYC_REQUIRED
            | INFORMAL_STAKE_REPUTATION
            | VA_CAN_BID_ON_PUBLIC_AUCTION
            | DISTRIBUTE_PAYMENT_TO_NON_VOTERS => VariableType::Bool,
            _ => VariableType::Unknown,
        }
    }
}
