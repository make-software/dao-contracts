use casper_dao_utils::TestContract;

use super::{DaoWorld, params::Contract};

mod account;
mod bid_escrow;
mod cspr;
mod events;
mod kyc;
mod ownership;
mod reputation;
mod va;
mod voting;

#[macro_export]
macro_rules! on_contract {
    ($world:ident, $contract:ident, $call:ident($($arg:expr),*)) => {
        match &$contract {
            Contract::KycToken => $world.kyc_token.$call( $($arg),* ),
            Contract::KycVoter => $world.kyc_voter.$call( $($arg),* ),
            Contract::VaToken => $world.va_token.$call( $($arg),* ),
            Contract::ReputationToken => $world.reputation_token.$call( $($arg),* ),
            Contract::BidEscrow => $world.bid_escrow.$call( $($arg),* ),
            Contract::VariableRepository => $world.variable_repository.$call( $($arg),* ),
            Contract::SlashingVoter => $world.slashing_voter.$call( $($arg),* ),
            Contract::Admin => $world.admin.$call( $($arg),* ),
            Contract::RepoVoter => $world.repo_voter.$call( $($arg),* ),
            Contract::SimpleVoter => $world.simple_voter.$call( $($arg),* ),
            Contract::ReputationVoter => $world.reputation_voter.$call( $($arg),* ),
            Contract::Onboarding => $world.onboarding.$call( $($arg),* ),
        }
    };
    ($world:ident, $caller:ident, $contract:ident, $call:ident($($arg:expr),*)) => {
        match &$contract {
            Contract::KycToken => $world.kyc_token.as_account($caller).$call( $($arg),* ),
            Contract::KycVoter => $world.kyc_voter.as_account($caller).$call( $($arg),* ),
            Contract::VaToken => $world.va_token.as_account($caller).$call( $($arg),* ),
            Contract::ReputationToken => $world.reputation_token.as_account($caller).$call( $($arg),* ),
            Contract::BidEscrow => $world.bid_escrow.as_account($caller).$call( $($arg),* ),
            Contract::VariableRepository => $world.variable_repository.as_account($caller).$call( $($arg),* ),
            Contract::SlashingVoter => $world.slashing_voter.as_account($caller).$call( $($arg),* ),
            Contract::Admin => $world.admin.as_account($caller).$call( $($arg),* ),
            Contract::RepoVoter => $world.repo_voter.as_account($caller).$call( $($arg),* ),
            Contract::SimpleVoter => $world.simple_voter.as_account($caller).$call( $($arg),* ),
            Contract::ReputationVoter => $world.reputation_voter.as_account($caller).$call( $($arg),* ),
            Contract::Onboarding => $world.onboarding.as_account($caller).$call( $($arg),* ),
        }
    }
}

#[macro_export]
macro_rules! on_voting_contract {
    ($world:ident, $contract:ident, $call:ident($($arg:expr),*)) => {
        match &$contract {
            Contract::KycVoter => $world.kyc_voter.$call( $($arg),* ),
            Contract::BidEscrow => $world.bid_escrow.$call( $($arg),* ),
            Contract::SlashingVoter => $world.slashing_voter.$call( $($arg),* ),
            Contract::Admin => $world.admin.$call( $($arg),* ),
            Contract::RepoVoter => $world.repo_voter.$call( $($arg),* ),
            Contract::SimpleVoter => $world.simple_voter.$call( $($arg),* ),
            Contract::ReputationVoter => $world.reputation_voter.$call( $($arg),* ),
            Contract::Onboarding => $world.onboarding.$call( $($arg),* ),
            invalid => panic!("{:?} is not a voting contract", invalid),
        }
    };
    ($world:ident, $caller:ident, $contract:ident, $call:ident($($arg:expr),*)) => {
        match &$contract {
            Contract::KycVoter => $world.kyc_voter.as_account($caller).$call( $($arg),* ),
            Contract::BidEscrow => $world.bid_escrow.as_account($caller).$call( $($arg),* ),
            Contract::SlashingVoter => $world.slashing_voter.as_account($caller).$call( $($arg),* ),
            Contract::Admin => $world.admin.as_account($caller).$call( $($arg),* ),
            Contract::RepoVoter => $world.repo_voter.as_account($caller).$call( $($arg),* ),
            Contract::SimpleVoter => $world.simple_voter.as_account($caller).$call( $($arg),* ),
            Contract::ReputationVoter => $world.reputation_voter.as_account($caller).$call( $($arg),* ),
            Contract::Onboarding => $world.onboarding.as_account($caller).$call( $($arg),* ),
            invalid => panic!("{:?} is not a voting contract", invalid),
        }
    }
}
