#![no_main]

#[no_mangle]
fn call() {
    let schemas = vec![];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "init",
        vec![odra::casper::casper_types::Parameter::new(
            "rate",
            odra::casper::casper_types::CLType::U512,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Groups(vec![
            odra::casper::casper_types::Group::new("constructor_group"),
        ]),
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_rate",
        vec![],
        odra::casper::casper_types::CLType::U512,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "set_rate",
        vec![odra::casper::casper_types::Parameter::new(
            "rate",
            odra::casper::casper_types::CLType::U512,
        )],
        odra::casper::casper_types::CLType::Unit,
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
            let mut contract_ref =
                dao::utils_contracts::CSPRRateProviderContractRef::at(&odra_address);
            let rate = odra::casper::casper_contract::contract_api::runtime::get_named_arg("rate");
            contract_ref.init(rate);
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
    let (mut _contract, _): (dao::utils_contracts::CSPRRateProviderContract, _) =
        odra::StaticInstance::instance(&["owner#owner", "rate"]);
    let rate = odra::casper::casper_contract::contract_api::runtime::get_named_arg("rate");
    _contract.init(rate);
}
#[no_mangle]
fn get_rate() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::utils_contracts::CSPRRateProviderContract, _) =
        odra::StaticInstance::instance(&["", "rate"]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_rate();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn set_rate() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::utils_contracts::CSPRRateProviderContract, _) =
        odra::StaticInstance::instance(&["owner#owner", "rate"]);
    let rate = odra::casper::casper_contract::contract_api::runtime::get_named_arg("rate");
    _contract.set_rate(rate);
}
#[no_mangle]
fn get_owner() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::utils_contracts::CSPRRateProviderContract, _) =
        odra::StaticInstance::instance(&["owner#owner", ""]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
