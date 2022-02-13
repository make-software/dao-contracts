use proc_macro2::TokenStream;
use quote::quote;

use crate::contract::{self};
use crate::parser::ContractTrait;
use crate::{caller, contract_test};

pub fn expand_casper_contract_interface(input: ContractTrait) -> TokenStream {
    let contract_install = contract::generate_install(&input);
    let contract_entry_points = contract::generate_entry_points(&input);
    let interface_trait = contract::interface::generate_trait(&input);
    let caller_struct = caller::generate_struct(&input);
    let caller_impl = caller::generate_interface_impl(&input);

    let contract_test_impl = contract_test::generate_test_implementation(&input);
    let contract_test_interface = contract_test::generate_test_interface(&input);

    quote! {
      #contract_install

      #contract_entry_points

      #interface_trait

      #caller_struct

      #caller_impl

      #contract_test_impl

      #contract_test_interface
    }
}
