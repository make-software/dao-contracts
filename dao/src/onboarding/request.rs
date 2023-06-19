//! Onboarding request related structs
use crate::rules::validation::bid_escrow::{ExistsOngoingVoting, IsNotVa};
use crate::rules::validation::IsUserKyced;
use crate::rules::RulesBuilder;
use crate::utils::types::DocumentHash;
use odra::types::{Address, Balance};
use odra::OdraType;

/// Onboarding request
pub struct OnboardingRequest {
    pub requestor: Address,
    pub reason: DocumentHash,
    pub rep_stake: Balance,
    pub cspr_deposit: Balance,
    pub is_va: bool,
    pub exists_ongoing_voting: bool,
    pub is_kyced: bool,
}

/// Onboarding request saved in the state
#[derive(OdraType, Debug)]
pub struct Request {
    creator: Address,
    reason: DocumentHash,
    rep_stake: Balance,
    cspr_deposit: Balance,
}

impl Request {
    /// Creates new onboarding request
    pub fn new(request: OnboardingRequest) -> Self {
        RulesBuilder::new()
            .add_validation(IsUserKyced::create(request.is_kyced))
            .add_validation(IsNotVa::create(request.is_va))
            .add_validation(ExistsOngoingVoting::create(request.exists_ongoing_voting))
            .build()
            .validate_generic_validations();

        Request {
            creator: request.requestor,
            reason: request.reason,
            rep_stake: request.rep_stake,
            cspr_deposit: request.cspr_deposit,
        }
    }

    /// Returns creator of the request
    pub fn creator(&self) -> Address {
        self.creator
    }

    /// Returns reason of the request
    pub fn reason(&self) -> &DocumentHash {
        &self.reason
    }

    /// Returns reputation stake of the request
    pub fn rep_stake(&self) -> Balance {
        self.rep_stake
    }

    /// Returns CSPR deposit of the request
    pub fn cspr_deposit(&self) -> Balance {
        self.cspr_deposit
    }
}
