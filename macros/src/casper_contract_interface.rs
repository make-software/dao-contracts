use proc_macro2::TokenStream;
use quote::quote;

use crate::casper_contract;
use crate::contract;
use crate::parser::CasperContract;
use crate::{caller, contract_test};

pub fn generate_code(input: CasperContract) -> TokenStream {
    let contract_impl = contract::generate_code(&input);
    let contract_interface_trait = contract::interface::generate_code(&input);
    let caller = caller::generate_code(&input);
    let contract_test = contract_test::generate_code(&input);
    let contract_macro = casper_contract::generate_code(&input);

    quote! {
      #contract_impl

      #contract_interface_trait

      #caller

      #contract_test

      #contract_macro
    }
}
