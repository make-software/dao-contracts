#[casper_contract_interface]
pub trait DaoOwnedNftContractInterface {
    fn init(&mut self, name: String, symbol: String, uri: TokenUri);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn get_owner(&self) -> Option<Address>;
    fn is_whitelisted(&self, address: Address) -> bool;

    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn owner_of(&self, token_id: TokenId) -> Address;
    fn balance_of(&self, owner: Address) -> U256;
    fn total_supply(&self) -> U256;
    fn token_uri(&self, token_id: TokenId) -> TokenUri;
    fn base_uri(&self) -> TokenUri;
    fn mint(&mut self, to: Address, token_id: TokenId);
    fn burn(&mut self, token_id: TokenId);
}

// Tokens:
// - VaNftContract,
// - KycNftContract.

// Voters:
// - OnboardingVoter.
// - KycVoter.
#[casper_contract_interface]
pub trait OnboardingVoterContractInterface {
    fn init(
        &mut self,
        variable_repo: Address,
        reputation_token: Address,
        kyc_token: Address,
        va_token: Address,
    );

    // - Require no voting for a given `address` is on.

    // For Adding new VA:
    // - Check if VA is not onboarderd.
    // - Check if `address` is KYCed.
    // - Check if `address` has positive reputation amount.
    
    // For Removing existing VA:
    // - Check if VA is already onboarderd.
    // - Check if `address` has positive reputation amount.
    fn create_voting(
        &mut self,
        action: OnboardingAction, // Add, Remove
        address: Address,
        stake: U256,
    );
    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_kyc_token_address(&self) -> Address;
    fn get_va_token_address(&self) -> Address;

    fn get_voting(&self, voting_id: U256) -> Voting;
    fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
    fn get_voter(&self, voting_id: U256, at: u32) -> Address;
}

#[casper_contract_interface]
pub trait KycVoterContractInterface {
    fn init(
        &mut self, 
        variable_repo: Address, 
        reputation_token: Address,
        kyc_token: Address,
    );
    // Require no voting for a given `address` is on.
    // Precondition: KycNft.balance_of(address_to_onboard) == 0;
    // Action: KycNft.mint(address_to_onboard, next_token_id)
    fn create_voting(
        &mut self,
        address_to_onboard: Address,
        document_hash: String,
        stake: U256,
    );
    fn vote(&mut self, voting_id: VotingId, choice: bool, stake: U256);
    fn finish_voting(&mut self, voting_id: VotingId);
    fn get_dust_amount(&self) -> U256;
    fn get_variable_repo_address(&self) -> Address;
    fn get_reputation_token_address(&self) -> Address;
    fn get_kyc_token_address(&self) -> Address;
    fn get_voting(&self, voting_id: U256) -> Voting;
    fn get_vote(&self, voting_id: U256, address: Address) -> Vote;
    fn get_voter(&self, voting_id: U256, at: u32) -> Address;
}
