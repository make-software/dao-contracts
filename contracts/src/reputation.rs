use casper_dao_utils::{
    casper_dao_macros::casper_contract_interface,
    casper_env::{caller, init_events},
    owner::Owner,
    staking::TokenWithStaking,
    whitelist::Whitelist,
    Address,
};
use casper_types::U256;

#[casper_contract_interface]
pub trait ReputationContractInterface {
    fn init(&mut self);
    fn mint(&mut self, recipient: Address, amount: U256);
    fn burn(&mut self, owner: Address, amount: U256);
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);
    fn change_ownership(&mut self, owner: Address);
    fn add_to_whitelist(&mut self, address: Address);
    fn remove_from_whitelist(&mut self, address: Address);
    fn stake(&mut self, address: Address, amount: U256);
    fn unstake(&mut self, address: Address, amount: U256);
}

#[derive(Default)]
pub struct ReputationContract {
    pub token: TokenWithStaking,
    pub owner: Owner,
    pub whitelist: Whitelist,
}

impl ReputationContractInterface for ReputationContract {
    fn init(&mut self) {
        init_events();
        let deployer = caller();
        self.owner.init(deployer);
        self.whitelist.init();
        self.whitelist.add_to_whitelist(deployer);
        self.token.init();
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.mint(recipient, amount);
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.burn(owner, amount);
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.raw_transfer(owner, recipient, amount);
    }

    fn change_ownership(&mut self, owner: Address) {
        self.owner.ensure_owner();
        self.owner.change_ownership(owner);
        self.whitelist.add_to_whitelist(owner);
    }

    fn add_to_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.add_to_whitelist(address);
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        self.owner.ensure_owner();
        self.whitelist.remove_from_whitelist(address);
    }

    fn stake(&mut self, address: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.stake(address, amount);
    }

    fn unstake(&mut self, address: Address, amount: U256) {
        self.whitelist.ensure_whitelisted();
        self.token.unstake(address, amount);
    }
}

#[cfg(feature = "test-support")]
impl ReputationContractTest {
    pub fn get_owner(&self) -> Option<Address> {
        self.env
            .get_value(self.package_hash, self.data.owner.owner.path())
    }

    pub fn total_supply(&self) -> U256 {
        self.env
            .get_value(self.package_hash, self.data.token.token.total_supply.path())
    }

    pub fn balance_of(&self, address: Address) -> U256 {
        self.env.get_dict_value(
            self.package_hash,
            self.data.token.token.balances.path(),
            address,
        )
    }

    pub fn is_whitelisted(&self, address: Address) -> bool {
        self.env.get_dict_value(
            self.package_hash,
            self.data.whitelist.whitelist.path(),
            address,
        )
    }

    pub fn get_staked_balance_of(&self, address: Address) -> U256 {
        self.env
            .get_dict_value(self.package_hash, self.data.token.stakes.path(), address)
    }
}
