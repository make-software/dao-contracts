extern crate proc_macro;

use parser::CasperContractItem;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod caller;
mod casper_contract;
mod casper_contract_interface;
mod contract;
mod contract_test;
mod event;
mod parser;

/// Derive events on top of any struct.
#[proc_macro_derive(Event)]
pub fn derive_events(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    event::expand_derive_events(input).into()
}

#[proc_macro_attribute]
pub fn casper_contract_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as CasperContractItem);
    casper_contract_interface::generate_code(input).into()
}