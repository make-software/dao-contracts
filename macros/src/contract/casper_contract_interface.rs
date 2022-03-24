use proc_macro::TokenStream;
use quote::quote;

use crate::contract::{caller, casper_contract, contract, contract_test};
use syn::parse_macro_input;

use super::parser::CasperContractItem;

pub fn generate_code(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as CasperContractItem);

    let contract_impl = contract::generate_code(&item);
    let contract_interface_trait = contract::interface::generate_code(&item);
    let caller = caller::generate_code(&item);
    let contract_test = contract_test::generate_code(&item);
    let contract_macro = casper_contract::generate_code(&item);

    let result = quote! {
      #contract_impl

      #contract_interface_trait

      #caller

      #contract_test

      #contract_macro
    };

    result.into()
}
