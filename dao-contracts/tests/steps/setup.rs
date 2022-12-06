use cucumber::{gherkin::Step, given};

use crate::common::{
    config::UserConfiguration,
    params::{Account, U256, U512},
    DaoWorld,
};

#[given(expr = "users")]
#[given(expr = "accounts")]
#[given(expr = "following balances")]
fn users_setup(world: &mut DaoWorld, step: &Step) {
    let labels = step
        .table
        .as_ref()
        .unwrap()
        .rows
        .first()
        .expect("User configuration is missing");
    let users_iter = step.table.as_ref().unwrap().rows.iter().skip(1);

    for row in users_iter {
        let config = UserConfiguration::from_labeled_data(labels, row);

        let account = config.account();
        let owner = Account::Owner;
        let reputation_balance = config.reputation_balance();
        let cspr_balance = config.cspr_balance();

        for contract in config.get_contracts_to_be_whitelisted_in() {
            world.whitelist_account(contract, &owner, account).unwrap();
        }

        let user_address = world.get_address(account);
        if config.is_kyced() {
            world.mint_kyc_token(&owner, account).unwrap();
        }

        // TODO: world should accept an Account.
        if config.is_va() {
            world.make_va(user_address);
        }

        if reputation_balance > U256::zero() {
            world.mint_reputation(&Account::Owner, account, reputation_balance);
        }

        let address = world.get_address(account);
        world.set_cspr_balance(address, cspr_balance.0);
    }
}
