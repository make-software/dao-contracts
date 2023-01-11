use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    ContractCall,
};

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct VotingConfiguration {
    pub is_bid_escrow: bool,
    pub bound_ballot_for_successful_voting: bool,
    pub bound_ballot_address: Option<Address>,
    pub contract_calls: Vec<ContractCall>,
    pub only_va_can_create: bool,
    pub double_time_between_votings: bool,
}
