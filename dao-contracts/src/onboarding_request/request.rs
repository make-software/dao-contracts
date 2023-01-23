use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    DocumentHash,
};
use casper_types::U512;

use crate::{
    rules::validation::bid_escrow::{ExistsOngoingVoting, IsNotVa},
    rules::{RulesBuilder, validation::IsUserKyced},
};

pub struct OnboardingRequest {
    pub requestor: Address,
    pub reason: DocumentHash,
    pub rep_stake: U512,
    pub cspr_deposit: U512,
    pub is_va: bool,
    pub exists_ongoing_voting: bool,
    pub is_kyced: bool,
}

#[derive(CLTyped, ToBytes, FromBytes, Debug)]
pub struct Request {
    creator: Address,
    reason: DocumentHash,
    rep_stake: U512,
    cspr_deposit: U512,
}

impl Request {
    pub fn new(request: OnboardingRequest) -> Self {
        RulesBuilder::new()
            .add_validation(IsUserKyced::create(request.is_kyced))
            .add_validation(IsNotVa::create(request.is_va))
            .add_validation(ExistsOngoingVoting::create(request.exists_ongoing_voting))
            .validate();

        Request {
            creator: request.requestor,
            reason: request.reason,
            rep_stake: request.rep_stake,
            cspr_deposit: request.cspr_deposit,
        }
    }

    pub fn creator(&self) -> Address {
        self.creator
    }

    pub fn reason(&self) -> &DocumentHash {
        &self.reason
    }

    pub fn rep_stake(&self) -> U512 {
        self.rep_stake
    }

    pub fn cspr_deposit(&self) -> U512 {
        self.cspr_deposit
    }
}
