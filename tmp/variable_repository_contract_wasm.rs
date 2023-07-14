#![no_main]

#[no_mangle]
fn call() {
    let schemas = vec![odra::casper::utils::build_event(
        String::from("ValueUpdated"),
        vec![
            ("key", odra::casper::casper_types::CLType::String),
            (
                "value",
                odra::casper::casper_types::CLType::List(Box::new(
                    odra::casper::casper_types::CLType::U8,
                )),
            ),
            (
                "activation_time",
                odra::casper::casper_types::CLType::Option(Box::new(
                    odra::casper::casper_types::CLType::U64,
                )),
            ),
        ],
    )];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "init",
        vec![
            odra::casper::casper_types::Parameter::new(
                "fiat_conversion",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "bid_escrow_wallet",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "voting_ids",
                odra::casper::casper_types::CLType::Key,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Groups(vec![
            odra::casper::casper_types::Group::new("constructor_group"),
        ]),
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "update_at",
        vec![
            odra::casper::casper_types::Parameter::new(
                "key",
                odra::casper::casper_types::CLType::String,
            ),
            odra::casper::casper_types::Parameter::new(
                "value",
                odra::casper::casper_types::CLType::List(Box::new(
                    odra::casper::casper_types::CLType::U8,
                )),
            ),
            odra::casper::casper_types::Parameter::new(
                "activation_time",
                odra::casper::casper_types::CLType::Option(Box::new(
                    odra::casper::casper_types::CLType::U64,
                )),
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get",
        vec![odra::casper::casper_types::Parameter::new(
            "key",
            odra::casper::casper_types::CLType::String,
        )],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::List(Box::new(
                odra::casper::casper_types::CLType::U8,
            )),
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_full_value",
        vec![odra::casper::casper_types::Parameter::new(
            "key",
            odra::casper::casper_types::CLType::String,
        )],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::Any,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_key_at",
        vec![odra::casper::casper_types::Parameter::new(
            "index",
            odra::casper::casper_types::CLType::U32,
        )],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::String,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "keys_count",
        vec![],
        odra::casper::casper_types::CLType::U32,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "all_variables",
        vec![],
        odra::casper::casper_types::CLType::Map {
            key: Box::new(odra::casper::casper_types::CLType::String),
            value: Box::new(odra::casper::casper_types::CLType::List(Box::new(
                odra::casper::casper_types::CLType::U8,
            ))),
        },
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
            let mut contract_ref =
                dao::core_contracts::VariableRepositoryContractRef::at(&odra_address);
            let fiat_conversion =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg(
                    "fiat_conversion",
                );
            let bid_escrow_wallet =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg(
                    "bid_escrow_wallet",
                );
            let voting_ids =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_ids");
            contract_ref.init(fiat_conversion, bid_escrow_wallet, voting_ids);
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
    let (mut _contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "repository#storage",
            "repository#keys2#values",
            "repository#keys2#index",
        ]);
    let fiat_conversion =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("fiat_conversion");
    let bid_escrow_wallet =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("bid_escrow_wallet");
    let voting_ids =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_ids");
    _contract.init(fiat_conversion, bid_escrow_wallet, voting_ids);
}
#[no_mangle]
fn update_at() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "access_control#whitelist#whitelist",
            "repository#storage",
            "repository#keys2#values",
            "repository#keys2#index",
        ]);
    let key = odra::casper::casper_contract::contract_api::runtime::get_named_arg("key");
    let value = odra::casper::casper_contract::contract_api::runtime::get_named_arg("value");
    let activation_time =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("activation_time");
    _contract.update_at(key, value, activation_time);
}
#[no_mangle]
fn get() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&["", "", "repository#storage", "", ""]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let key = odra::casper::casper_contract::contract_api::runtime::get_named_arg("key");
    let result = _contract.get(key);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_full_value() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "repository#storage",
            "repository#keys2#values",
            "repository#keys2#index",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let key = odra::casper::casper_contract::contract_api::runtime::get_named_arg("key");
    let result = _contract.get_full_value(key);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_key_at() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "repository#storage",
            "repository#keys2#values",
            "repository#keys2#index",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let index = odra::casper::casper_contract::contract_api::runtime::get_named_arg("index");
    let result = _contract.get_key_at(index);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn keys_count() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&["", "", "", "", "repository#keys2#index"]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.keys_count();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn all_variables() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "repository#storage",
            "repository#keys2#values",
            "repository#keys2#index",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.all_variables();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn is_whitelisted() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&["", "access_control#whitelist#whitelist", "", "", ""]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.is_whitelisted(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_owner() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&["access_control#owner#owner", "", "", "", ""]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn change_ownership() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    _contract.change_ownership(owner);
}
#[no_mangle]
fn add_to_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.add_to_whitelist(address);
}
#[no_mangle]
fn remove_from_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VariableRepositoryContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
            "",
            "",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.remove_from_whitelist(address);
}
