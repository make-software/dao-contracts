use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    ContractCall,
};

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct VotingConfiguration {
    pub is_bid_escrow: bool,
    pub contract_calls: Vec<ContractCall>,
    pub only_va_can_create: bool,
    pub double_time_between_votings: bool,
    pub is_bid_escrow: bool,
}
