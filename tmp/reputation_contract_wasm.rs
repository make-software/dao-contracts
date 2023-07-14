#![no_main]

#[no_mangle]
fn call() {
    let schemas = vec![
        odra::casper::utils::build_event(
            String::from("Mint"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Burn"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Mint"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Burn"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Mint"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Burn"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Mint"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Burn"),
            vec![
                ("address", odra::casper::casper_types::CLType::Key),
                ("amount", odra::casper::casper_types::CLType::U512),
            ],
        ),
    ];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "init",
        vec![],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Groups(vec![
            odra::casper::casper_types::Group::new("constructor_group"),
        ]),
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "mint_passive",
        vec![
            odra::casper::casper_types::Parameter::new(
                "recipient",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "amount",
                odra::casper::casper_types::CLType::U512,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "burn_passive",
        vec![
            odra::casper::casper_types::Parameter::new(
                "owner",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "amount",
                odra::casper::casper_types::CLType::U512,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "passive_balance_of",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::U512,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "change_ownership",
        vec![odra::casper::casper_types::Parameter::new(
            "owner",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "add_to_whitelist",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "remove_from_whitelist",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "is_whitelisted",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Bool,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_owner",
        vec![],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::Key,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "mint",
        vec![
            odra::casper::casper_types::Parameter::new(
                "recipient",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "amount",
                odra::casper::casper_types::CLType::U512,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "burn",
        vec![
            odra::casper::casper_types::Parameter::new(
                "owner",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "amount",
                odra::casper::casper_types::CLType::U512,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "total_supply",
        vec![],
        odra::casper::casper_types::CLType::U512,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "balance_of",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::U512,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "bulk_mint_burn",
        vec![
            odra::casper::casper_types::Parameter::new(
                "mints",
                odra::casper::casper_types::CLType::Map {
                    key: Box::new(odra::casper::casper_types::CLType::Key),
                    value: Box::new(odra::casper::casper_types::CLType::U512),
                },
            ),
            odra::casper::casper_types::Parameter::new(
                "burns",
                odra::casper::casper_types::CLType::Map {
                    key: Box::new(odra::casper::casper_types::CLType::Key),
                    value: Box::new(odra::casper::casper_types::CLType::U512),
                },
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "burn_all",
        vec![odra::casper::casper_types::Parameter::new(
            "owner",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "stake",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voter",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "stake",
                odra::casper::casper_types::CLType::U512,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "unstake",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voter",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "stake",
                odra::casper::casper_types::CLType::U512,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "bulk_unstake",
        vec![odra::casper::casper_types::Parameter::new(
            "stakes",
            odra::casper::casper_types::CLType::List(Box::new(
                odra::casper::casper_types::CLType::Tuple2([
                    Box::new(odra::casper::casper_types::CLType::Key),
                    Box::new(odra::casper::casper_types::CLType::U512),
                ]),
            )),
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_stake",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::U512,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "all_balances",
        vec![],
        odra::casper::casper_types::CLType::Any,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "partial_balances",
        vec![odra::casper::casper_types::Parameter::new(
            "addresses",
            odra::casper::casper_types::CLType::List(Box::new(
                odra::casper::casper_types::CLType::Key,
            )),
        )],
        odra::casper::casper_types::CLType::Any,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    #[allow(unused_variables)]
    let contract_package_hash = odra::casper::utils::install_contract(entry_points, schemas);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let constructor_access = odra::casper::utils::create_constructor_group(contract_package_hash);
    let constructor_name = odra::casper::utils::load_constructor_name_arg();
    match constructor_name.as_str() {
        "init" => {
            let odra_address = odra::types::Address::try_from(contract_package_hash)
                .map_err(|err| {
                    let code = odra::types::ExecutionError::from(err).code();
                    odra::casper::casper_types::ApiError::User(code)
                })
                .unwrap_or_revert();
            let mut contract_ref = dao::core_contracts::ReputationContractRef::at(&odra_address);
            contract_ref.init();
        }
        _ => odra::casper::utils::revert_on_unknown_constructor(),
    };
    odra::casper::utils::revoke_access_to_constructor_group(
        contract_package_hash,
        constructor_access,
    );
}
#[no_mangle]
fn init() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    _contract.init();
}
#[no_mangle]
fn mint_passive() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "passive_reputation_storage#balances",
            "passive_reputation_storage#holders#values",
            "passive_reputation_storage#holders#index",
            "passive_reputation_storage#total_supply#total_supply",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let recipient =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("recipient");
    let amount = odra::casper::casper_contract::contract_api::runtime::get_named_arg("amount");
    _contract.mint_passive(recipient, amount);
}
#[no_mangle]
fn burn_passive() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "passive_reputation_storage#balances",
            "passive_reputation_storage#holders#values",
            "passive_reputation_storage#holders#index",
            "passive_reputation_storage#total_supply#total_supply",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    let amount = odra::casper::casper_contract::contract_api::runtime::get_named_arg("amount");
    _contract.burn_passive(owner, amount);
}
#[no_mangle]
fn passive_balance_of() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "passive_reputation_storage#balances",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.passive_balance_of(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn change_ownership() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    _contract.change_ownership(owner);
}
#[no_mangle]
fn add_to_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.add_to_whitelist(address);
}
#[no_mangle]
fn remove_from_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.remove_from_whitelist(address);
}
#[no_mangle]
fn is_whitelisted() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "access_control#whitelist#whitelist",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.is_whitelisted(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_owner() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "access_control#owner#owner",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn mint() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let recipient =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("recipient");
    let amount = odra::casper::casper_contract::contract_api::runtime::get_named_arg("amount");
    _contract.mint(recipient, amount);
}
#[no_mangle]
fn burn() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    let amount = odra::casper::casper_contract::contract_api::runtime::get_named_arg("amount");
    _contract.burn(owner, amount);
}
#[no_mangle]
fn total_supply() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "reputation_storage#total_supply#total_supply",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.total_supply();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn balance_of() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "reputation_storage#balances",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.balance_of(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn bulk_mint_burn() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let mints = odra::casper::casper_contract::contract_api::runtime::get_named_arg("mints");
    let burns = odra::casper::casper_contract::contract_api::runtime::get_named_arg("burns");
    _contract.bulk_mint_burn(mints, burns);
}
#[no_mangle]
fn burn_all() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    _contract.burn_all(owner);
}
#[no_mangle]
fn stake() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "stakes_storage#stake",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "reputation_storage#access_control#owner#owner",
            "reputation_storage#access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let voter = odra::casper::casper_contract::contract_api::runtime::get_named_arg("voter");
    let stake = odra::casper::casper_contract::contract_api::runtime::get_named_arg("stake");
    _contract.stake(voter, stake);
}
#[no_mangle]
fn unstake() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "stakes_storage#stake",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "reputation_storage#access_control#owner#owner",
            "reputation_storage#access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let voter = odra::casper::casper_contract::contract_api::runtime::get_named_arg("voter");
    let stake = odra::casper::casper_contract::contract_api::runtime::get_named_arg("stake");
    _contract.unstake(voter, stake);
}
#[no_mangle]
fn bulk_unstake() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "stakes_storage#stake",
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "reputation_storage#access_control#owner#owner",
            "reputation_storage#access_control#whitelist#whitelist",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    let stakes = odra::casper::casper_contract::contract_api::runtime::get_named_arg("stakes");
    _contract.bulk_unstake(stakes);
}
#[no_mangle]
fn get_stake() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "stakes_storage#stake",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.get_stake(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn all_balances() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "reputation_storage#access_control#owner#owner",
            "reputation_storage#access_control#whitelist#whitelist",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.all_balances();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn partial_balances() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::ReputationContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "reputation_storage#balances",
            "reputation_storage#holders#values",
            "reputation_storage#holders#index",
            "reputation_storage#total_supply#total_supply",
            "reputation_storage#access_control#owner#owner",
            "reputation_storage#access_control#whitelist#whitelist",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let addresses =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("addresses");
    let result = _contract.partial_balances(addresses);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
