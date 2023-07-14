#![no_main]

#[no_mangle]
fn call() {
    let schemas = vec![
        odra::casper::utils::build_event(
            String::from("SimpleVotingCreated"),
            vec![
                ("document_hash", odra::casper::casper_types::CLType::String),
                ("creator", odra::casper::casper_types::CLType::Key),
                (
                    "stake",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::U512,
                    )),
                ),
                ("voting_id", odra::casper::casper_types::CLType::U32),
                (
                    "config_informal_quorum",
                    odra::casper::casper_types::CLType::U32,
                ),
                (
                    "config_informal_voting_time",
                    odra::casper::casper_types::CLType::U64,
                ),
                (
                    "config_formal_quorum",
                    odra::casper::casper_types::CLType::U32,
                ),
                (
                    "config_formal_voting_time",
                    odra::casper::casper_types::CLType::U64,
                ),
                (
                    "config_total_onboarded",
                    odra::casper::casper_types::CLType::U512,
                ),
                (
                    "config_double_time_between_votings",
                    odra::casper::casper_types::CLType::Bool,
                ),
                (
                    "config_voting_clearness_delta",
                    odra::casper::casper_types::CLType::U512,
                ),
                (
                    "config_time_between_informal_and_formal_voting",
                    odra::casper::casper_types::CLType::U64,
                ),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("VotingCreatedInfo"),
            vec![
                ("creator", odra::casper::casper_types::CLType::Key),
                (
                    "stake",
                    odra::casper::casper_types::CLType::Option(Box::new(
                        odra::casper::casper_types::CLType::U512,
                    )),
                ),
                ("voting_id", odra::casper::casper_types::CLType::U32),
                (
                    "config_informal_quorum",
                    odra::casper::casper_types::CLType::U32,
                ),
                (
                    "config_informal_voting_time",
                    odra::casper::casper_types::CLType::U64,
                ),
                (
                    "config_formal_quorum",
                    odra::casper::casper_types::CLType::U32,
                ),
                (
                    "config_formal_voting_time",
                    odra::casper::casper_types::CLType::U64,
                ),
                (
                    "config_total_onboarded",
                    odra::casper::casper_types::CLType::U512,
                ),
                (
                    "config_double_time_between_votings",
                    odra::casper::casper_types::CLType::Bool,
                ),
                (
                    "config_voting_clearness_delta",
                    odra::casper::casper_types::CLType::U512,
                ),
                (
                    "config_time_between_informal_and_formal_voting",
                    odra::casper::casper_types::CLType::U64,
                ),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("BallotCast"),
            vec![
                ("voter", odra::casper::casper_types::CLType::Key),
                ("voting_id", odra::casper::casper_types::CLType::U32),
                ("voting_type", odra::casper::casper_types::CLType::U32),
                ("choice", odra::casper::casper_types::CLType::U32),
                ("stake", odra::casper::casper_types::CLType::U512),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("VotingEnded"),
            vec![
                ("voting_id", odra::casper::casper_types::CLType::U32),
                ("voting_type", odra::casper::casper_types::CLType::U32),
                ("voting_result", odra::casper::casper_types::CLType::U32),
                ("stake_in_favor", odra::casper::casper_types::CLType::U512),
                ("stake_against", odra::casper::casper_types::CLType::U512),
                (
                    "unbound_stake_in_favor",
                    odra::casper::casper_types::CLType::U512,
                ),
                (
                    "unbound_stake_against",
                    odra::casper::casper_types::CLType::U512,
                ),
                ("votes_in_favor", odra::casper::casper_types::CLType::U32),
                ("votes_against", odra::casper::casper_types::CLType::U32),
                (
                    "unstakes",
                    odra::casper::casper_types::CLType::Map {
                        key: Box::new(odra::casper::casper_types::CLType::Tuple2([
                            Box::new(odra::casper::casper_types::CLType::Key),
                            Box::new(odra::casper::casper_types::CLType::U32),
                        ])),
                        value: Box::new(odra::casper::casper_types::CLType::U512),
                    },
                ),
                (
                    "stakes",
                    odra::casper::casper_types::CLType::Map {
                        key: Box::new(odra::casper::casper_types::CLType::Tuple2([
                            Box::new(odra::casper::casper_types::CLType::Key),
                            Box::new(odra::casper::casper_types::CLType::U32),
                        ])),
                        value: Box::new(odra::casper::casper_types::CLType::U512),
                    },
                ),
                (
                    "burns",
                    odra::casper::casper_types::CLType::Map {
                        key: Box::new(odra::casper::casper_types::CLType::Tuple2([
                            Box::new(odra::casper::casper_types::CLType::Key),
                            Box::new(odra::casper::casper_types::CLType::U32),
                        ])),
                        value: Box::new(odra::casper::casper_types::CLType::U512),
                    },
                ),
                (
                    "mints",
                    odra::casper::casper_types::CLType::Map {
                        key: Box::new(odra::casper::casper_types::CLType::Tuple2([
                            Box::new(odra::casper::casper_types::CLType::Key),
                            Box::new(odra::casper::casper_types::CLType::U32),
                        ])),
                        value: Box::new(odra::casper::casper_types::CLType::U512),
                    },
                ),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("VotingCanceled"),
            vec![
                ("voting_id", odra::casper::casper_types::CLType::U32),
                ("voting_type", odra::casper::casper_types::CLType::U32),
                (
                    "unstakes",
                    odra::casper::casper_types::CLType::Map {
                        key: Box::new(odra::casper::casper_types::CLType::Key),
                        value: Box::new(odra::casper::casper_types::CLType::U512),
                    },
                ),
            ],
        ),
        odra::casper::utils::build_event(
            String::from("BallotCanceled"),
            vec![
                ("voter", odra::casper::casper_types::CLType::Key),
                ("voting_id", odra::casper::casper_types::CLType::U32),
                ("voting_type", odra::casper::casper_types::CLType::U32),
                ("choice", odra::casper::casper_types::CLType::U32),
                ("stake", odra::casper::casper_types::CLType::U512),
            ],
        ),
    ];
    let mut entry_points = odra::casper::casper_types::EntryPoints::new();
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "init",
        vec![
            odra::casper::casper_types::Parameter::new(
                "variable_repository",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "reputation_token",
                odra::casper::casper_types::CLType::Key,
            ),
            odra::casper::casper_types::Parameter::new(
                "va_token",
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
        "create_voting",
        vec![
            odra::casper::casper_types::Parameter::new(
                "document_hash",
                odra::casper::casper_types::CLType::String,
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
        "finish_voting",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voting_id",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "voting_type",
                odra::casper::casper_types::CLType::U32,
            ),
        ],
        odra::casper::casper_types::CLType::Any,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_document_hash",
        vec![odra::casper::casper_types::Parameter::new(
            "voting_id",
            odra::casper::casper_types::CLType::U32,
        )],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::String,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "vote",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voting_id",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "voting_type",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "choice",
                odra::casper::casper_types::CLType::U32,
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
        "slash_voter",
        vec![odra::casper::casper_types::Parameter::new(
            "voter",
            odra::casper::casper_types::CLType::Key,
        )],
        odra::casper::casper_types::CLType::Unit,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "voting_exists",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voting_id",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "voting_type",
                odra::casper::casper_types::CLType::U32,
            ),
        ],
        odra::casper::casper_types::CLType::Bool,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_voting",
        vec![odra::casper::casper_types::Parameter::new(
            "voting_id",
            odra::casper::casper_types::CLType::U32,
        )],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::Any,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_ballot",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voting_id",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "voting_type",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "address",
                odra::casper::casper_types::CLType::Key,
            ),
        ],
        odra::casper::casper_types::CLType::Option(Box::new(
            odra::casper::casper_types::CLType::Any,
        )),
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "get_voter",
        vec![
            odra::casper::casper_types::Parameter::new(
                "voting_id",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "voting_type",
                odra::casper::casper_types::CLType::U32,
            ),
            odra::casper::casper_types::Parameter::new(
                "at",
                odra::casper::casper_types::CLType::U32,
            ),
        ],
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
        "variable_repository_address",
        vec![],
        odra::casper::casper_types::CLType::Key,
        odra::casper::casper_types::EntryPointAccess::Public,
        odra::casper::casper_types::EntryPointType::Contract,
    ));
    entry_points.add_entry_point(odra::casper::casper_types::EntryPoint::new(
        "reputation_token_address",
        vec![],
        odra::casper::casper_types::CLType::Key,
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
            let mut contract_ref = dao::voting_contracts::SimpleVoterContractRef::at(&odra_address);
            let variable_repository =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg(
                    "variable_repository",
                );
            let reputation_token =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg(
                    "reputation_token",
                );
            let va_token =
                odra::casper::casper_contract::contract_api::runtime::get_named_arg("va_token");
            contract_ref.init(variable_repository, reputation_token, va_token);
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
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "refs#variable_repository",
            "refs#reputation_token",
            "refs#va_token",
            "refs#kyc_token",
            "",
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
    let variable_repository =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("variable_repository");
    let reputation_token =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("reputation_token");
    let va_token = odra::casper::casper_contract::contract_api::runtime::get_named_arg("va_token");
    _contract.init(variable_repository, reputation_token, va_token);
}
#[no_mangle]
fn create_voting() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "refs#variable_repository",
            "",
            "refs#va_token",
            "",
            "refs#variable_repository",
            "refs#reputation_token",
            "refs#va_token",
            "refs#kyc_token",
            "voting_engine#voting_states",
            "voting_engine#ballots",
            "voting_engine#voters",
            "voting_engine#configurations",
            "voting_engine#active_votings",
            "simple_votings",
            "",
            "",
        ]);
    let document_hash =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("document_hash");
    let stake = odra::casper::casper_contract::contract_api::runtime::get_named_arg("stake");
    _contract.create_voting(document_hash, stake);
}
#[no_mangle]
fn finish_voting() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "refs#variable_repository",
            "refs#reputation_token",
            "refs#va_token",
            "refs#kyc_token",
            "voting_engine#voting_states",
            "voting_engine#ballots",
            "voting_engine#voters",
            "voting_engine#configurations",
            "voting_engine#active_votings",
            "simple_votings",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let voting_type =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_type");
    let result = _contract.finish_voting(voting_id, voting_type);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_document_hash() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "simple_votings",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let result = _contract.get_document_hash(voting_id);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn vote() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "refs#variable_repository",
            "refs#reputation_token",
            "refs#va_token",
            "refs#kyc_token",
            "voting_engine#voting_states",
            "voting_engine#ballots",
            "voting_engine#voters",
            "voting_engine#configurations",
            "voting_engine#active_votings",
            "",
            "",
            "",
        ]);
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let voting_type =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_type");
    let choice = odra::casper::casper_contract::contract_api::runtime::get_named_arg("choice");
    let stake = odra::casper::casper_contract::contract_api::runtime::get_named_arg("stake");
    _contract.vote(voting_id, voting_type, choice, stake);
}
#[no_mangle]
fn slash_voter() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "refs#variable_repository",
            "refs#reputation_token",
            "refs#va_token",
            "refs#kyc_token",
            "voting_engine#voting_states",
            "voting_engine#ballots",
            "voting_engine#voters",
            "voting_engine#configurations",
            "voting_engine#active_votings",
            "",
            "",
            "access_control#whitelist#whitelist",
        ]);
    let voter = odra::casper::casper_contract::contract_api::runtime::get_named_arg("voter");
    _contract.slash_voter(voter);
}
#[no_mangle]
fn voting_exists() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "voting_engine#voting_states",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let voting_type =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_type");
    let result = _contract.voting_exists(voting_id, voting_type);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_voting() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "voting_engine#voting_states",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let result = _contract.get_voting(voting_id);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_ballot() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "voting_engine#ballots",
            "",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let voting_type =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_type");
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    let result = _contract.get_ballot(voting_id, voting_type, address);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn get_voter() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "voting_engine#voters",
            "",
            "",
            "",
            "",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let voting_id =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_id");
    let voting_type =
        odra::casper::casper_contract::contract_api::runtime::get_named_arg("voting_type");
    let at = odra::casper::casper_contract::contract_api::runtime::get_named_arg("at");
    let result = _contract.get_voter(voting_id, voting_type, at);
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn change_ownership() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    let owner = odra::casper::casper_contract::contract_api::runtime::get_named_arg("owner");
    _contract.change_ownership(owner);
}
#[no_mangle]
fn add_to_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.add_to_whitelist(address);
}
#[no_mangle]
fn remove_from_whitelist() {
    odra::casper::utils::assert_no_attached_value();
    let (mut _contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "access_control#owner#owner",
            "access_control#whitelist#whitelist",
        ]);
    let address = odra::casper::casper_contract::contract_api::runtime::get_named_arg("address");
    _contract.remove_from_whitelist(address);
}
#[no_mangle]
fn is_whitelisted() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
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
            "access_control#owner#owner",
            "",
        ]);
    use odra::casper::casper_contract::unwrap_or_revert::UnwrapOrRevert;
    let result = _contract.get_owner();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn variable_repository_address() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "refs#variable_repository",
            "",
            "",
            "",
            "",
            "",
            "",
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
    let result = _contract.variable_repository_address();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
#[no_mangle]
fn reputation_token_address() {
    odra::casper::utils::assert_no_attached_value();
    let (_contract, _): (dao::voting_contracts::SimpleVoterContract, _) =
        odra::StaticInstance::instance(&[
            "",
            "refs#reputation_token",
            "",
            "",
            "",
            "",
            "",
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
    let result = _contract.reputation_token_address();
    let result = odra::casper::casper_types::CLValue::from_t(result).unwrap_or_revert();
    odra::casper::casper_contract::contract_api::runtime::ret(result);
}
