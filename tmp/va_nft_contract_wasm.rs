#![no_main]

#[no_mangle]
fn call() {
    let schemas = vec![
        odra::casper::utils::build_event(
            String::from("Transfer"),
            vec![
                (
                    "from",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::Key,
                    )),
                ),
                (
                    "to",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::Key,
                    )),
                ),
                ("token_id", odra::casper::casper_types::CLType::U256),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Approval"),
            vec![
                ("owner", odra::casper::casper_types::CLType::Key),
                (
                    "approved",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::Key,
                    )),
                ),
                ("token_id", odra::casper::casper_types::CLType::U256),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("ApprovalForAll"),
            vec![
                ("owner", odra::casper::casper_types::CLType::Key),
                ("operator", odra::casper::casper_types::CLType::Key),
                ("approved", odra::casper::casper_types::CLType::Bool),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("Transfer"),
            vec![
                (
                    "from",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::Key,
                    )),
                ),
                (
                    "to",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::Key,
                    )),
                ),
                ("token_id", odra::casper::casper_types::CLType::U256),
            ],
        ),
    ];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "init",
        vec![
            odra::casper::casper_types::Parameter::new(
                "name",
                odra::casper::casper_types::CLType::String,
            ),
            odra::casper::casper_types::Parameter::new(
                "symbol",
                odra::casper::casper_types::CLType::String,
            ),
            odra::casper::casper_types::Parameter::new(
                "base_uri",
                odra::casper::casper_types::CLType::String,
            ),
        ],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Groups(vec![
            odra::casper::casper_types::Group::new("constructor_group"),
        ]),
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
        "name",
        vec![],
        odra::casper::casper_types::CLType::String,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "symbol",
        vec![],
        odra::casper::casper_types::CLType::String,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "owner_of",
        vec![odra::casper::casper_types::Parameter::new(
            "token_id",
            odra::casper::casper_types::CLType::U256,
        )],
        odra::casper::casper_types::CLType::Key,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "token_id",
        vec![odra::casper::casper_types::Parameter::new(
            "address",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::U256,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "balance_of",
        vec![odra::casper::casper_types::Parameter::new(
            "owner",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::U256,
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
        "token_uri",
        vec![odra::casper::casper_types::Parameter::new(
            "token_id",
            odra::casper::casper_types::CLType::U256,
        )],
        odra::casper::casper_types::CLType::String,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "base_uri",
        vec![],
        odra::casper::casper_types::CLType::String,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "mint",
        vec![odra::casper::casper_types::Parameter::new(
            "to",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "burn",
        vec![odra::casper::casper_types::Parameter::new(
            "owner",
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
            let mut contract_ref = dao::core_contracts::VaNftContractRef::at(&odra_address);
            let name = odra::casper::casper_contract::contract_api::runtime::get_named_arg("name");
            let symbol =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg("symbol");
            let base_uri =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg("base_uri");
            contract_ref.init(name, symbol, base_uri);
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
    let (mut _contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "token#metadata#name",
            "token#metadata#symbol",
            "token#metadata#base_uri",
            "token#access_control#owner#owner",
            "token#access_control#whitelist#whitelist",
            "",
            "",
            "",
        ]);
    let name = odra::casper::casper_contract::contract_api::runtime::get_named_arg("name");
    let symbol = odra::casper::casper_contract::contract_api::runtime::get_named_arg("symbol");
    let base_uri = odra::casper::casper_contract::contract_api::runtime::get_named_arg("base_uri");
    _contract.init(name, symbol, base_uri);
}
#[no_mangle]
fn change_ownership() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "token#access_control#owner#owner",
            "token#access_control#whitelist#whitelist",
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
    let (mut _contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "token#access_control#owner#owner",
            "token#access_control#whitelist#whitelist",
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
    let (mut _contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "token#access_control#owner#owner",
            "token#access_control#whitelist#whitelist",
            "",
            "",
            "",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.remove_from_whitelist(address);
}
#[no_mangle]
fn is_whitelisted() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "token#access_control#whitelist#whitelist",
            "",
            "",
            "",
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
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "token#access_control#owner#owner",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn name() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "token#metadata#name",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.name();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn symbol() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "token#metadata#symbol",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.symbol();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn owner_of() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "token#core#owners",
            "",
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
    let token_id = odra::casper::casper_contract::contract_api::runtime::get_named_arg("token_id");
    let result = _contract.owner_of(&token_id);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn token_id() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
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
            "token#tokens",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.token_id(address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn balance_of() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "token#core#balances",
            "",
            "",
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
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    let result = _contract.balance_of(&owner);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn total_supply() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
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
            "token#total_supply",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.total_supply();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn token_uri() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "token#core#owners",
            "",
            "",
            "",
            "",
            "token#metadata#base_uri",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let token_id = odra::casper::casper_contract::contract_api::runtime::get_named_arg("token_id");
    let result = _contract.token_uri(token_id);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn base_uri() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "token#metadata#base_uri",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.base_uri();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn mint() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "token#core#balances",
            "token#core#owners",
            "token#core#token_approvals",
            "",
            "",
            "",
            "",
            "token#access_control#owner#owner",
            "token#access_control#whitelist#whitelist",
            "token#tokens",
            "token#id_gen#value",
            "token#total_supply",
        ]);
    let to = odra::casper::casper_contract::contract_api::runtime::get_named_arg("to");
    _contract.mint(to);
}
#[no_mangle]
fn burn() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::core_contracts::VaNftContract, _) =
        odra::StaticInstance::instance(&[
            "token#core#balances",
            "token#core#owners",
            "token#core#token_approvals",
            "",
            "",
            "",
            "",
            "token#access_control#owner#owner",
            "token#access_control#whitelist#whitelist",
            "token#tokens",
            "",
            "token#total_supply",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    _contract.burn(owner);
}
