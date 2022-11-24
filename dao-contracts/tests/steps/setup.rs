use cucumber::{gherkin::Step, given};

use crate::common::{
    config::UserConfiguration,
    params::{common::Contract, nft::Account},
    DaoWorld,
};

#[given(expr = "users in {contract} contract")]
fn users_setup(world: &mut DaoWorld, step: &Step, contract: Contract) {
    let users_iter = step.table.as_ref().unwrap().rows.iter().skip(1);

    for row in users_iter {
        let config: UserConfiguration = row.into();

        if config.is_whitelisted() {
            world
                .whitelist(&contract, &Account::Owner, config.account())
                .unwrap();
        }

        // TODO: world should accept an Account.
        let user_address = config.account().get_address(world);
        if config.is_kyced() {
            world.kyc(user_address);
        }

        if config.is_va() {
            world.make_va(user_address);
        }
    }
}
