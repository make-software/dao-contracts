use dao::core_contracts::TokenId as DaoTokenId;
use odra::{
    test_env,
    types::{Address, Balance, U256},
};

use crate::common::{
    params::{Account, Contract, TokenId},
    DaoWorld,
};

#[odra::external_contract]
pub trait TotalSupply {
    fn total_supply(&self) -> Balance;
}

#[odra::external_contract]
pub trait NftToken {
    fn balance_of(&self, owner: Address) -> U256;
    fn owner_of(&self, token_id: DaoTokenId) -> Address;
    fn mint(&mut self, to: Address);
    fn burn(&mut self, owner: Address);
    fn token_id(&self, address: Address) -> Option<DaoTokenId>;
}

impl DaoWorld {
    pub fn total_supply(&self, contract: Contract) -> Balance {
        let contract = self.contract_address(contract);
        TotalSupplyRef::at(&contract).total_supply()
    }

    pub fn nft_balance_of(&self, contract: Contract, account: &Account) -> u32 {
        let contract = self.contract_address(contract);

        NftTokenRef::at(&contract)
            .balance_of(self.get_address(account))
            .as_u32()
    }

    pub fn nft_owner_of(&self, contract: Contract, token_id: TokenId) -> Address {
        let contract = self.contract_address(contract);

        NftTokenRef::at(&contract).owner_of(*token_id)
    }

    pub fn mint_nft_token(&mut self, contract: Contract, minter: &Account, recipient: &Account) {
        let contract = self.contract_address(contract);
        let minter = self.get_address(minter);
        let recipient = self.get_address(recipient);
        test_env::set_caller(minter);
        NftTokenRef::at(&contract).mint(recipient);
    }

    pub fn burn_nft_token(&mut self, contract: Contract, burner: &Account, holder: &Account) {
        let contract = self.contract_address(contract);
        let burner = self.get_address(burner);
        let holder = self.get_address(holder);

        test_env::set_caller(burner);
        NftTokenRef::at(&contract).burn(holder);
    }

    pub fn has_nft_token(&self, contract: Contract, account: &Account) -> bool {
        let contract = self.contract_address(contract);
        let address = self.get_address(account);
        !NftTokenRef::at(&contract).balance_of(address).is_zero()
    }

    pub fn get_nft_token_id(&self, contract: Contract, holder: &Account) -> TokenId {
        let contract = self.contract_address(contract);
        let holder = self.get_address(holder);
        let id = NftTokenRef::at(&contract)
            .token_id(holder)
            .expect("Holder should own a token");
        TokenId(id)
    }

    fn contract_address(&self, contract: Contract) -> Address {
        let account = Account::Contract(contract);
        self.get_address(&account)
    }
}
