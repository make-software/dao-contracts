use casper_dao_utils::{
    casper_dao_macros::{CLTyped, FromBytes, ToBytes},
    Address,
    ContractCall,
};

#[derive(CLTyped, ToBytes, FromBytes, Debug, Clone)]
pub struct VotingConfiguration {
    pub is_bid_escrow: bool,
    pub bind_ballot_for_successful_voting: bool,
    pub unbound_ballot_address: Option<Address>,
    pub contract_calls: Vec<ContractCall>,
    pub only_va_can_create: bool,
    pub double_time_between_votings: bool,
}

impl VotingConfiguration {
    pub fn set_bind_ballot_for_successful_voting(
        &mut self,
        bind_ballot_for_successful_voting: bool,
    ) {
        self.bind_ballot_for_successful_voting = bind_ballot_for_successful_voting;
    }

    pub fn set_unbound_ballot_address(&mut self, unbound_ballot_address: Option<Address>) {
        self.unbound_ballot_address = unbound_ballot_address;
    }

    pub fn set_is_bid_escrow(&mut self, is_bid_escrow: bool) {
        self.is_bid_escrow = is_bid_escrow;
    }

    pub fn set_only_va_can_create(&mut self, only_va_can_create: bool) {
        self.only_va_can_create = only_va_can_create;
    }

    pub fn set_contract_calls(&mut self, contract_calls: Vec<ContractCall>) {
        self.contract_calls = contract_calls;
    }

    pub fn should_bind_ballot_for_successful_voting(&self) -> bool {
        self.bind_ballot_for_successful_voting
    }

    pub fn get_unbound_ballot_address(&self) -> Option<Address> {
        self.unbound_ballot_address
    }
}
