use std::{fs::File, io::Write};

use casper_dao_utils::definitions::ContractDef;
use convert_case::{Case, Casing};
use dao_contracts_schemas::all_contracts;

fn main() {
    let contracts = all_contracts();
    contracts.iter().for_each(write_to_file);
}

fn write_to_file(schema: &ContractDef) {
    let file_name = format!("resources/{}_schema.json", schema.name.to_case(Case::Snake));
    println!("Writing {}", file_name);

    let content = serde_json::to_string_pretty(schema).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
