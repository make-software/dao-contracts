extern crate proc_macro;
use contract::ContractTrait;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod caller;
mod contract;
mod contract_test;
mod event;

#[proc_macro_derive(Event)]
pub fn derive_events(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(event::generate(input))
}

#[proc_macro_attribute]
pub fn casper_contract_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ContractTrait);

    let contract_install = contract::generate_install(&input);
    let contract_entry_points = contract::generate_entry_points(&input);
    let interface_trait = contract::interface::generate_trait(&input);
    let caller_struct = caller::generate_struct(&input);
    let caller_impl = caller::generate_interface_impl(&input);

    let contract_test_impl = contract_test::generate_test_implementation(&input);
    let contract_test_interface = contract_test::generate_test_interface(&input);

    let expanded = quote! {
      #contract_install

      #contract_entry_points

      #interface_trait

      #caller_struct

      #caller_impl

      #contract_test_impl

      #contract_test_interface
    };

    TokenStream::from(expanded)
}
