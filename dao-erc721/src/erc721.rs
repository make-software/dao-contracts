use casper_dao_utils::{casper_dao_macros::casper_contract_interface, Address, Mapping, Variable};
use casper_types::{bytesrepr::Bytes, U256};

pub type TokenId = U256;
pub type TokenUri = String;

#[casper_contract_interface]
pub trait ERC721Interface {
    fn init(&mut self, name: String, symbol: String);
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn owner_of(&self, token_id: TokenId) -> Address;
    fn balance_of(&self, owner: Address) -> U256;
    fn total_supply(&self) -> U256;
    fn token_uri(&self, token_id: TokenId) -> TokenUri;
    fn base_uri(&self) -> TokenUri;
    fn token_of_owner_by_index(&self, owner: Address, index: U256) -> TokenId;
    fn token_by_index(&self, index: U256) -> TokenId;
    fn transfer(&mut self, recipient: Address, amount: U256);
    fn approve(&mut self, spender: Address, token_id: TokenId);
    fn get_approved(&self, token_id: TokenId) -> Address;
    fn set_approval_for_all(&self, operator: Address, approved: bool);
    fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool;
    fn transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId);
    fn safe_transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId);
    fn safe_transfer_from_with_data(
        &mut self,
        owner: Address,
        recipient: Address,
        token_id: TokenId,
        data: Bytes,
    );
}

pub struct ERC721 {
    name: Variable<String>,
    symbol: Variable<String>,
    total_supply: Variable<U256>,
    // Mapping owner address to token count
    balances: Mapping<Address, U256>,
    // Mapping from token ID to owner address
    owners: Mapping<U256, Option<Address>>,
    // Mapping from token ID to approved address
    token_approvals: Mapping<U256, Option<Address>>,
    // Mapping from owner to operator approvals
    operator_approvals: Mapping<(Address, Address), bool>,
}

impl Default for ERC721 {
    fn default() -> Self {
        Self {
            name: Variable::from("name"),
            symbol: Variable::from("symbol"),
            total_supply: Variable::from("total_supply"),
            balances: Mapping::from("balances"),
            owners: Mapping::from("owners"),
            token_approvals: Mapping::from("token_approvals"),
            operator_approvals: Mapping::from("operator_approvals"),
        }
    }
}

impl ERC721Interface for ERC721 {
    fn init(&mut self, name: String, symbol: String) {
        self.name.set(name);
        self.symbol.set(symbol);
    }

    fn name(&self) -> String {
        self.name.get()
    }

    fn symbol(&self) -> String {
        self.symbol.get()
    }

    fn owner_of(&self, token_id: TokenId) -> Address {
        todo!()
    }

    fn balance_of(&self, owner: Address) -> U256 {
        todo!()
    }

    fn total_supply(&self) -> U256 {
        todo!()
    }

    fn token_uri(&self, token_id: TokenId) -> TokenUri {
        todo!()
    }

    fn base_uri(&self) -> TokenUri {
        todo!()
    }

    fn token_of_owner_by_index(&self, owner: Address, index: U256) -> TokenId {
        todo!()
    }

    fn token_by_index(&self, index: U256) -> TokenId {
        todo!()
    }

    fn transfer(&mut self, recipient: Address, amount: U256) {
        todo!()
    }

    fn approve(&mut self, spender: Address, token_id: TokenId) {
        todo!()
    }

    fn get_approved(&self, token_id: TokenId) -> Address {
        todo!()
    }

    fn set_approval_for_all(&self, operator: Address, approved: bool) {
        todo!()
    }

    fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        todo!()
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId) {
        todo!()
    }

    fn safe_transfer_from(&mut self, owner: Address, recipient: Address, token_id: TokenId) {
        todo!()
    }

    fn safe_transfer_from_with_data(
        &mut self,
        owner: Address,
        recipient: Address,
        token_id: TokenId,
        data: Bytes,
    ) {
        todo!()
    }
}
