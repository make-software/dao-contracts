use casper_types::U256;

use crate::{token::Token, Address, Mapping};

pub struct TokenWithStaking {
    pub stakes: Mapping<Address, U256>,
    pub token: Token,
}

impl Default for TokenWithStaking {
    fn default() -> Self {
        Self {
            stakes: Mapping::new(format!("stakes")),
            token: Token::default(),
        }
    }
}

impl TokenWithStaking {
    pub fn init(&mut self) {
        self.token.init();
    }

    pub fn mint(&mut self, recipient: Address, amount: U256) {
        self.token.mint(recipient, amount);
    }

    pub fn burn(&mut self, owner: Address, amount: U256) {
        self.token.burn(owner, amount);
    }

    pub fn raw_transfer(&mut self, sender: Address, recipient: Address, amount: U256) {
        self.token.raw_transfer(sender, recipient, amount);
    }

    fn stake(&mut self, address: Address, amount: U256) {}

    fn unstake(&mut self, address: Address, amount: U256) {}
}

pub mod entry_points {
    use casper_types::{CLTyped, EntryPoint, EntryPointAccess, EntryPointType, Parameter, U256};

    use crate::Address;

    const EP_MINT: &str = "mint";
    const EP_BURN: &str = "burn";
    const EP_TRANSFER_FROM: &str = "transfer_from";
    const EP_STAKE: &str = "stake";
    const EP_UNSTAKE: &str = "unstake";

    const PARAM_RECIPIENT: &str = "recipient";
    const PARAM_AMOUNT: &str = "amount";
    const PARAM_OWNER: &str = "owner";

    pub fn mint() -> EntryPoint {
        EntryPoint::new(
            EP_MINT,
            vec![
                Parameter::new(PARAM_RECIPIENT, Address::cl_type()),
                Parameter::new(PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn burn() -> EntryPoint {
        EntryPoint::new(
            EP_BURN,
            vec![
                Parameter::new(PARAM_OWNER, Address::cl_type()),
                Parameter::new(PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn transfer_from() -> EntryPoint {
        EntryPoint::new(
            EP_TRANSFER_FROM,
            vec![
                Parameter::new(PARAM_OWNER, Address::cl_type()),
                Parameter::new(PARAM_RECIPIENT, Address::cl_type()),
                Parameter::new(PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn stake() -> EntryPoint {
        EntryPoint::new(
            EP_STAKE,
            vec![
                Parameter::new(PARAM_OWNER, Address::cl_type()),
                Parameter::new(PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }

    pub fn unstake() -> EntryPoint {
        EntryPoint::new(
            EP_UNSTAKE,
            vec![
                Parameter::new(PARAM_OWNER, Address::cl_type()),
                Parameter::new(PARAM_AMOUNT, U256::cl_type()),
            ],
            <()>::cl_type(),
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    }
}
