use std::{fs::File, io::Write};

use casper_dao_utils::definitions::ContractDef;
use convert_case::{Case, Casing};
use dao_contracts_schemas::{all_contracts, all_proxy_wasms, ProxyWasmDef};
use serde::Serialize;

fn main() {
    all_contracts().iter().for_each(write_schema_to_file);
    all_proxy_wasms().iter().for_each(write_proxy_wasm_to_file);
}

fn write_schema_to_file(schema: &ContractDef) {
    let file_name = format!(
        "resources/contract_{}_schema.json",
        schema.name.to_case(Case::Snake)
    );
    write_to_file(file_name, schema);
}

fn write_proxy_wasm_to_file(schema: &ProxyWasmDef) {
    let file_name = format!(
        "resources/proxy_{}_{}_schema.json",
        schema.contract.to_case(Case::Snake),
        schema.method
    );
    write_to_file(file_name, schema);
}

fn write_to_file<T: Serialize>(file_name: String, content: &T) {
    println!("Writing {}", file_name);
    let content = serde_json::to_string_pretty(content).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
