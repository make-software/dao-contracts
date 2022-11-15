use proc_macro2::TokenStream;
use quote::quote;

use super::parser::CasperContractItem;
use crate::contract::{caller, contract_bin, contract_struct, contract_test};

pub fn generate_code(item: CasperContractItem) -> TokenStream {
    match generate_or_err(item) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn generate_or_err(item: CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_impl = contract_struct::generate_code(&item)?;
    let contract_interface_trait = contract_struct::interface::generate_code(&item);
    let caller = caller::generate_code(&item);
    let contract_test = contract_test::generate_code(&item)?;
    let contract_macro = contract_bin::generate_code(&item);

    Ok(quote! {
      #contract_impl

      #contract_interface_trait

      #caller

      #contract_test

      #contract_macro
    })
}
