use std::collections::BTreeSet;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_dao_utils::{
    casper_env::{caller, init_events},
    consts,
    owner::Owner,
    staking::TokenWithStaking,
    whitelist::Whitelist,
    Address,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLTyped, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, RuntimeArgs, URef, U256,
};

/// Interface of the Reputation Contract.
///
/// It should be implemented by [`ReputationContract`], [`ReputationContractCaller`]
/// and [`ReputationContractTest`].
pub trait ReputationContractInterface {
    /// Constructor method.
    ///
    /// It initializes contract elements:
    /// * Events dictionary.
    /// * Named keys of [`TokenWithStaking`], [`Owner`] and [`Whitelist`].
    /// * Set [`caller`] as the owner of the contract.
    /// * Add [`caller`] to the whitelist.
    ///
    /// It emits [`OwnerChanged`](casper_dao_utils::owner::events::OwnerChanged),
    /// [`AddedToWhitelist`](casper_dao_utils::whitelist::events::AddedToWhitelist) events.
    fn init(&mut self);

    /// Mint new tokens. Add `amount` of new tokens to the balance of the `recipient` and
    /// increment the total supply. Only whitelisted addresses are permited to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Mint`](casper_dao_utils::token::events::Mint) event.
    fn mint(&mut self, recipient: Address, amount: U256);

    /// Burn existing tokens. Remove `amount` of existing tokens from the balance of the `owner`
    /// and decrement the total supply. Only whitelisted addresses are permited to call this
    /// method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It emits [`Burn`](casper_dao_utils::token::events::Burn) event.
    fn burn(&mut self, owner: Address, amount: U256);

    /// Transfer `amount` of tokens from `owner` to `recipient`. Only whitelisted addresses are
    /// permited to call this method.
    ///
    /// It throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if caller
    /// is not whitelisted.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance)
    /// if `recipient`'s balance is less then `amount`.
    ///
    /// It emits [`Transfer`](casper_dao_utils::token::events::Transfer) event.
    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256);

    /// Change ownership of the contract. Transfer the ownership to the `owner`. Only current owner
    /// is permited to call this method.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It emits [`OwnerChanged`](casper_dao_utils::owner::events::OwnerChanged),
    /// [`AddedToWhitelist`](casper_dao_utils::whitelist::events::AddedToWhitelist) events.
    fn change_ownership(&mut self, owner: Address);

    /// Add new address to the whitelist.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It emits [`AddedToWhitelist`](casper_dao_utils::whitelist::events::AddedToWhitelist) event.
    fn add_to_whitelist(&mut self, address: Address);

    /// Remove address from the whitelist.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It emits [`RemovedFromWhitelist`](casper_dao_utils::whitelist::events::RemovedFromWhitelist)
    /// event.
    fn remove_from_whitelist(&mut self, address: Address);

    /// Stake `amount` of tokens for the `address`. It decrements `address`'s balance by `amount`.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance)
    /// if `address`'s balance is less then `amount`.
    ///
    /// It emits [`TokensStaked`](casper_dao_utils::staking::events::TokensStaked)
    /// event.
    fn stake(&mut self, address: Address, amount: U256);

    /// Unstake `amount` of tokens for the `address`. It increments `address`'s balance by
    /// `amount`.
    ///
    /// It throws [`NotAnOwner`](casper_dao_utils::Error::NotAnOwner) if caller
    /// is not the current owner.
    ///
    /// It throws [`InsufficientBalance`](casper_dao_utils::Error::InsufficientBalance)
    /// if `address`'s staked amount is less then `amount`.
    ///
    /// It emits [`TokensUnstaked`](casper_dao_utils::staking::events::TokensUnstaked)
    /// event.
    fn unstake(&mut self, address: Address, amount: U256);
}

/// Implementation of the Reputation Contract. See [`ReputationContractInterface`].
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

impl ReputationContract {
    pub fn install() {
        // Create a new contract package hash for the contract.
        let (contract_package_hash, _) = storage::create_contract_package_at_hash();
        runtime::put_key(
            "reputation_contract_package_hash",
            contract_package_hash.into(),
        );

        let init_access: URef = storage::create_contract_user_group(
            contract_package_hash,
            "init",
            1,
            Default::default(),
        )
        .unwrap_or_revert()
        .pop()
        .unwrap_or_revert();

        storage::add_contract_version(
            contract_package_hash,
            ReputationContract::entry_points(),
            NamedKeys::new(),
        );

        // Call contrustor method.
        let mut contract_instance = ReputationContractCaller::at(contract_package_hash);
        contract_instance.init();

        // Revoke access to init.
        let mut urefs = BTreeSet::new();
        urefs.insert(init_access);
        storage::remove_contract_user_group_urefs(contract_package_hash, "init", urefs)
            .unwrap_or_revert();
    }

    pub fn entry_points() -> EntryPoints {
        let mut entry_points = EntryPoints::new();
        entry_points.add_entry_point(EntryPoint::new(
            "init",
            vec![],
            <()>::cl_type(),
            EntryPointAccess::Groups(vec![Group::new("init")]),
            EntryPointType::Contract,
        ));

        entry_points.add_entry_point(casper_dao_utils::owner::entry_points::change_ownership());
        entry_points.add_entry_point(casper_dao_utils::whitelist::entry_points::add_to_whitelist());
        entry_points
            .add_entry_point(casper_dao_utils::whitelist::entry_points::remove_from_whitelist());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::mint());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::burn());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::transfer_from());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::stake());
        entry_points.add_entry_point(casper_dao_utils::staking::entry_points::unstake());

        entry_points
    }
}

/// Implementation of the Reputation Contract Caller. See [`ReputationContractInterface`].
pub struct ReputationContractCaller {
    contract_package_hash: ContractPackageHash,
}

impl ReputationContractCaller {
    pub fn at(contract_package_hash: ContractPackageHash) -> Self {
        ReputationContractCaller {
            contract_package_hash,
        }
    }
}

impl ReputationContractInterface for ReputationContractCaller {
    fn init(&mut self) {
        let _: () = runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_INIT,
            runtime_args! {},
        );
    }

    fn mint(&mut self, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_MINT,
            runtime_args! {
                consts::PARAM_RECIPIENT => recipient,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn burn(&mut self, owner: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_BURN,
            runtime_args! {
                consts::PARAM_OWNER => owner,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_TRANSFER_FROM,
            runtime_args! {
                consts::PARAM_OWNER => owner,
                consts::PARAM_RECIPIENT => recipient,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn change_ownership(&mut self, owner: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_CHANGE_OWNERSHIP,
            runtime_args! {
                consts::PARAM_OWNER => owner,
            },
        )
    }

    fn add_to_whitelist(&mut self, address: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_ADD_TO_WHITELIST,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
            },
        )
    }

    fn remove_from_whitelist(&mut self, address: Address) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_REMOVE_FROM_WHITELIST,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
            },
        )
    }

    fn stake(&mut self, address: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_STAKE,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
                consts::PARAM_AMOUNT => amount
            },
        )
    }

    fn unstake(&mut self, address: Address, amount: U256) {
        runtime::call_versioned_contract(
            self.contract_package_hash,
            None,
            consts::EP_UNSTAKE,
            runtime_args! {
                consts::PARAM_ADDRESS => address,
                consts::PARAM_AMOUNT => amount
            },
        )
    }
}

#[cfg(feature = "test-support")]
mod tests {
    use std::fmt::Debug;

    use casper_dao_utils::{consts, Address, TestEnv};
    use casper_types::bytesrepr::{Bytes, FromBytes};
    use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};

    use crate::{ReputationContract, ReputationContractInterface};

    /// Implementation of the Reputation Contract Test. See [`ReputationContractInterface`].
    pub struct ReputationContractTest {
        env: TestEnv,
        package_hash: ContractPackageHash,
        data: ReputationContract,
    }

    impl ReputationContractTest {
        pub fn new(env: &TestEnv) -> ReputationContractTest {
            env.deploy_wasm_file("reputation_contract.wasm", runtime_args! {});
            let package_hash = env.get_contract_package_hash("reputation_contract_package_hash");
            ReputationContractTest {
                env: env.clone(),
                package_hash,
                data: ReputationContract::default(),
            }
        }

        pub fn as_account(&mut self, account: Address) -> &mut Self {
            self.env.as_account(account);
            self
        }

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

        pub fn event<T: FromBytes>(&self, index: u32) -> T {
            let raw_event: Bytes = self.env.get_dict_value(self.package_hash, "events", index);
            let (event, bytes) = T::from_bytes(&raw_event).unwrap();
            assert!(bytes.is_empty());
            event
        }

        pub fn assert_event_at<T: FromBytes + PartialEq + Debug>(&self, index: u32, event: T) {
            assert_eq!(self.event::<T>(index), event);
        }
    }

    impl ReputationContractInterface for ReputationContractTest {
        fn init(&mut self) {
            self.env
                .call_contract_package(self.package_hash, "init", runtime_args! {})
        }

        fn mint(&mut self, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_MINT,
                runtime_args! {
                    consts::PARAM_RECIPIENT => recipient,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn burn(&mut self, owner: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_BURN,
                runtime_args! {
                    consts::PARAM_OWNER => owner,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn transfer_from(&mut self, owner: Address, recipient: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_TRANSFER_FROM,
                runtime_args! {
                    consts::PARAM_OWNER => owner,
                    consts::PARAM_RECIPIENT => recipient,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn change_ownership(&mut self, owner: Address) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_CHANGE_OWNERSHIP,
                runtime_args! {
                    consts::PARAM_OWNER => owner
                },
            )
        }

        fn add_to_whitelist(&mut self, address: Address) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_ADD_TO_WHITELIST,
                runtime_args! {
                    consts::PARAM_ADDRESS => address
                },
            )
        }

        fn remove_from_whitelist(&mut self, address: Address) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_REMOVE_FROM_WHITELIST,
                runtime_args! {
                    consts::PARAM_ADDRESS => address
                },
            )
        }

        fn stake(&mut self, address: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_STAKE,
                runtime_args! {
                    consts::PARAM_ADDRESS => address,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }

        fn unstake(&mut self, address: Address, amount: U256) {
            self.env.call_contract_package(
                self.package_hash,
                consts::EP_UNSTAKE,
                runtime_args! {
                    consts::PARAM_ADDRESS => address,
                    consts::PARAM_AMOUNT => amount
                },
            )
        }
    }
}

#[cfg(feature = "test-support")]
pub use tests::ReputationContractTest;
