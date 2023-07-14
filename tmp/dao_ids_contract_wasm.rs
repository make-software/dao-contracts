#![no_main]

#[no_mangle]
fn call() {
    let schemas = vec![];
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
        "next_voting_id",
        vec![],
        odra::casper::casper_types::CLType::U32,
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
            let mut contract_ref = dao::utils_contracts::DaoIdsContractRef::at(&odra_address);
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
    let (mut _contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
        ]);
    _contract.init();
}
#[no_mangle]
fn next_voting_id() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "access_control#whitelist#whitelist",
            "voting_id_seq#value",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.next_voting_id();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn change_ownership() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    _contract.change_ownership(owner);
}
#[no_mangle]
fn add_to_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.add_to_whitelist(address);
}
#[no_mangle]
fn remove_from_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&[
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
            "",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.remove_from_whitelist(address);
}
#[no_mangle]
fn is_whitelisted() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&["", "access_control#whitelist#whitelist", ""]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.is_whitelisted(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_owner() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::utils_contracts::DaoIdsContract, _) =
        odra::StaticInstance::instance(&["access_control#owner#owner", "", ""]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
