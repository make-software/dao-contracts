use proc_macro2::TokenStream;
use quote::quote;

use crate::contract::{caller, casper_contract, contract, contract_test};

use super::parser::CasperContractItem;

pub fn generate_code(item: CasperContractItem) -> TokenStream {
    match generate_or_err(item) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn generate_or_err(item: CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_impl = contract::generate_code(&item)?;
    let contract_interface_trait = contract::interface::generate_code(&item);
    let caller = caller::generate_code(&item);
    let contract_test = contract_test::generate_code(&item)?;
    let contract_macro = casper_contract::generate_code(&item);

    Ok(quote! {
      #contract_impl

      #contract_interface_trait

      #caller

      #contract_test

      #contract_macro
    })
}
