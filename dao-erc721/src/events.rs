//! Event definitions the module emits.
use casper_dao_utils::{casper_dao_macros::Event, Address};

use crate::TokenId;

/// Informs a token has been transferred.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct Transfer {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub token_id: TokenId,
}

/// Informs the approval status has changed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct Approval {
    pub owner: Option<Address>,
    pub approved: Option<Address>,
    pub token_id: TokenId,
}

/// Informs the approval for all status has changed.
#[derive(Debug, PartialEq, Eq, Event)]
pub struct ApprovalForAll {
    pub owner: Address,
    pub operator: Address,
    pub approved: bool,
}
