use casper_dao_contracts::{DebtIncreased, DebtPaid, ReputationContractTest};
use casper_dao_erc20::events::Transfer;
use casper_dao_modules::events::{AddedToWhitelist, OwnerChanged, RemovedFromWhitelist};
use casper_dao_utils::{Error, TestContract, TestEnv};
use casper_types::U256;

#[test]
fn test_deploy() {
    let (env, contract) = setup();
    let deployer = env.get_account(0);
    assert_eq!(contract.total_supply(), U256::zero());
    assert_eq!(contract.balance_of(env.get_account(0)), U256::zero());
    assert_eq!(contract.balance_of(env.get_account(1)), U256::zero());
    assert!(contract.is_whitelisted(contract.get_owner().unwrap()));
    contract.assert_event_at(
        0,
        OwnerChanged {
            new_owner: deployer,
        },
    );
    contract.assert_event_at(1, AddedToWhitelist { address: deployer });
}

#[test]
fn test_init_cannot_be_called_twice() {
    let (_, mut contract) = setup();
    let result = contract.init();
    assert_eq!(result.unwrap_err(), Error::InvalidContext);
}

#[test]
fn test_mint_as_owner() {
    let (env, mut contract) = setup();
    let recipient = env.get_account(1);
    let total_supply = 100.into();

    contract.mint(recipient, total_supply).unwrap();

    assert_eq!(contract.balance_of(recipient), total_supply);
    contract.assert_event_at(
        2,
        Transfer {
            from: None,
            to: Some(recipient),
            value: total_supply,
        },
    );
}

#[test]
fn test_mint_as_non_owner() {
    let (env, mut contract) = setup();
    let non_owner = env.get_account(1);

    let result = contract.as_account(non_owner).mint(non_owner, 10.into());

    assert_eq!(result.unwrap_err(), Error::NotWhitelisted);
}

#[test]
fn test_whitelisted_user_burn() {
    let total_supply = 100.into();
    let burn_amount = 10.into();
    let remaining_supply = total_supply - burn_amount;

    let (env, mut contract) = setup_with_initial_supply(total_supply);
    let owner = env.get_account(0);

    contract.burn(owner, burn_amount).unwrap();
    assert_eq!(contract.total_supply(), remaining_supply);
    assert_eq!(contract.balance_of(owner), remaining_supply);
    contract.assert_event_at(
        3,
        Transfer {
            from: Some(owner),
            to: None,
            value: burn_amount,
        },
    );
}

#[test]
fn test_buring_amount_exceeding_balance() {
    let total_supply = 100.into();
    let burn_amount = 101.into();

    let (env, mut contract) = setup_with_initial_supply(total_supply);
    let owner = env.get_account(0);

    contract.burn(owner, burn_amount).unwrap();
    assert_eq!(contract.balance_of(owner), U256::zero());
    assert_eq!(contract.debt(owner), U256::one());
}

#[test]
fn test_buring_amount_exceeding_balance_and_minting_it_back() {
    let total_supply = 100.into();
    let burn_amount = 150.into();
    let mint_amount = 200.into();

    let (env, mut contract) = setup_with_initial_supply(total_supply);
    let owner = env.get_account(0);

    contract.burn(owner, burn_amount).unwrap();

    assert_eq!(contract.balance_of(owner), 0.into());
    contract.assert_event_at(
        -1,
        DebtIncreased {
            owner,
            amount: 50.into(),
            debt: 50.into(),
        },
    );

    contract.mint(owner, mint_amount).unwrap();

    assert_eq!(contract.balance_of(owner), 150.into());
    assert_eq!(contract.debt(owner), U256::zero());
    contract.assert_event_at(
        -1,
        DebtPaid {
            owner,
            amount: 50.into(),
            debt: 0.into(),
        },
    )
}

#[test]
fn test_non_whitelisted_user_burn() {
    let (env, mut contract) = setup_with_initial_supply(100.into());
    let (user1, user2) = (env.get_account(0), env.get_account(1));

    let result = contract.as_account(user2).burn(user1, 10.into());
    assert_eq!(result.unwrap_err(), Error::NotWhitelisted);
}

#[test]
fn test_total_supply_overflow() {
    let (env, mut contract) = setup();

    contract.mint(env.get_account(0), U256::MAX).unwrap();

    let result = contract.mint(env.get_account(0), U256::one());
    assert_eq!(result.unwrap_err(), Error::TotalSupplyOverflow);
}

#[test]
fn test_whitelisting_as_owner() {
    let (env, mut contract) = setup();
    let (owner, user) = (env.get_account(0), env.get_account(1));

    assert!(contract.is_whitelisted(owner));
    assert!(!contract.is_whitelisted(user));

    contract.add_to_whitelist(user).unwrap();
    assert!(contract.is_whitelisted(user));
    contract.assert_event_at(2, AddedToWhitelist { address: user });

    contract.remove_from_whitelist(user).unwrap();
    assert!(!contract.is_whitelisted(user));
    contract.assert_event_at(3, RemovedFromWhitelist { address: user });
}

#[test]
fn test_not_whitelisted_user_removal_has_no_effect() {
    let (env, mut contract) = setup();
    let user = env.get_account(1);

    assert!(!contract.is_whitelisted(user));

    contract.remove_from_whitelist(user).unwrap();
    assert!(!contract.is_whitelisted(user));
}

#[test]
fn test_duplicated_whitelisting() {
    let (env, mut contract) = setup();
    let user = env.get_account(1);

    contract.add_to_whitelist(user).unwrap();
    contract.add_to_whitelist(user).unwrap();
    assert!(contract.is_whitelisted(user));
    contract.assert_event_at(2, AddedToWhitelist { address: user });
    contract.assert_event_at(3, AddedToWhitelist { address: user });

    contract.remove_from_whitelist(user).unwrap();
    assert!(!contract.is_whitelisted(user));
    contract.assert_event_at(4, RemovedFromWhitelist { address: user });
}

#[test]
fn test_whitelisting_as_non_owner() {
    let (env, mut contract) = setup();
    let (user1, user2) = (env.get_account(1), env.get_account(2));

    contract.add_to_whitelist(user1).unwrap();

    let result = contract.as_account(user1).add_to_whitelist(user2);
    assert_eq!(result.unwrap_err(), Error::NotAnOwner);

    let result = contract.as_account(user1).remove_from_whitelist(user2);
    assert_eq!(result.unwrap_err(), Error::NotAnOwner);
}

#[test]
fn test_new_owner_whitelisting() {
    let (env, mut contract) = setup();
    let (owner, new_owner) = (env.get_account(0), env.get_account(1));

    assert!(contract.is_whitelisted(owner));

    contract.change_ownership(new_owner).unwrap();
    assert!(contract.is_whitelisted(new_owner));
    contract.assert_event_at(2, OwnerChanged { new_owner });
    contract.assert_event_at(3, AddedToWhitelist { address: new_owner });
}

#[test]
fn test_transfer_from() {
    let total_supply = 10.into();
    let transfer_amount = 4.into();

    let (env, mut contract) = setup_with_initial_supply(total_supply);
    let (owner, first_recipient) = (env.get_account(0), env.get_account(1));

    contract
        .transfer_from(owner, first_recipient, transfer_amount)
        .unwrap();

    assert_eq!(contract.balance_of(owner), total_supply - transfer_amount);
    assert_eq!(contract.balance_of(first_recipient), transfer_amount);
    contract.assert_event_at(
        3,
        Transfer {
            from: Some(owner),
            to: Some(first_recipient),
            value: transfer_amount,
        },
    );
}

#[test]
fn test_transfer_from_not_whitelisted_user() {
    let (env, mut contract) = setup();
    let (sender, recipient) = (env.get_account(1), env.get_account(2));

    contract.mint(sender, 10.into()).unwrap();

    let result = contract
        .as_account(sender)
        .transfer_from(sender, recipient, 1.into());
    assert_eq!(result.unwrap_err(), Error::NotWhitelisted);
}

#[test]
fn test_transfer_amount_higher_than_balance() {
    let total_supply = 10.into();
    let transfer_amount = 11.into();

    let (env, mut contract) = setup_with_initial_supply(total_supply);
    let (owner, first_recipient) = (env.get_account(0), env.get_account(1));

    let result = contract.transfer_from(owner, first_recipient, transfer_amount);
    assert_eq!(result.unwrap_err(), Error::InsufficientBalance);
}

#[test]
fn test_ownership() {
    let (env, mut contract) = setup();
    let (owner, new_owner) = (env.get_account(0), env.get_account(1));
    assert_eq!(contract.get_owner(), Some(owner));

    contract.change_ownership(new_owner).unwrap();
    assert_eq!(contract.get_owner().unwrap(), new_owner);

    let result = contract.change_ownership(new_owner);
    assert_eq!(result.unwrap_err(), Error::NotAnOwner);
}

#[test]
fn test_that_contract_have_different_hashes() {
    let env = TestEnv::new();
    let contract1 = ReputationContractTest::new(&env);
    let contract2 = ReputationContractTest::new(&env);

    assert_ne!(contract1.get_package_hash(), contract2.get_package_hash());
}

fn setup() -> (TestEnv, ReputationContractTest) {
    let env = TestEnv::new();
    let contract = ReputationContractTest::new(&env);

    (env, contract)
}

fn setup_with_initial_supply(amount: U256) -> (TestEnv, ReputationContractTest) {
    let (env, mut contract) = setup();
    contract.mint(env.get_account(0), amount).unwrap();

    (env, contract)
}
