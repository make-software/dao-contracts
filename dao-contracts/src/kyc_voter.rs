// #[casper_contract_interface]
// pub trait KycVoterContractInterface {
//     fn init(&mut self, variable_repo: Address, reputation_token: Address, kyc_token: Address);
//     // Require no voting for a given `address` is on.
//     // Precondition: KycNft.balance_of(address_to_onboard) == 0;
//     // Action: KycNft.mint(address_to_onboard, next_token_id)
//     fn create_voting(&mut self, address_to_onboard: Address, document_hash: String, stake: U256);
//     fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256);
//     fn finish_voting(&mut self, voting_id: VotingId);
//     fn get_dust_amount(&self) -> U256;
//     fn get_variable_repo_address(&self) -> Address;
//     fn get_reputation_token_address(&self) -> Address;
//     fn get_kyc_token_address(&self) -> Address;
//     fn get_voting(&self, voting_id: U256) -> Voting;
//     fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
//     fn get_voter(&self, voting_id: U256, at: u32) -> Address;
// }
