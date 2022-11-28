use cucumber::{gherkin::Step, given};

use crate::common::{
    config::UserConfiguration,
    params::{Account, Contract, U256},
    DaoWorld,
};

#[given(expr = "users in {contract} contract")]
fn users_setup(world: &mut DaoWorld, step: &Step, contract: Contract) {
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
        let reputation_balance = config.reputation_balance();

        if config.is_whitelisted() {
            world
                .whitelist(&contract, &Account::Owner, account)
                .unwrap();
        }

        // TODO: world should accept an Account.
        let user_address = world.get_address(account);
        if config.is_kyced() {
            world.kyc(user_address);
        }

        if config.is_va() {
            world.make_va(user_address);
        }

        if reputation_balance > U256::zero() {
            world.mint_reputation(&Account::Owner, account, reputation_balance);
        }
    }
}
