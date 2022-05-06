extern crate proc_macro;

use contract::CasperContractItem;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod contract;
mod event;
mod instance;
mod serialization;

/// Derive events on top of any struct.
#[proc_macro_derive(Event)]
pub fn derive_events(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    event::expand_derive_events(input).into()
}

#[proc_macro_derive(Instance)]
pub fn derive_instance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    instance::generate_code(input).into()
}

#[proc_macro_attribute]
pub fn casper_contract_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as CasperContractItem);
    contract::generate_code(item).into()
}

#[proc_macro_derive(CLTyped)]
pub fn derive_cl_typed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    serialization::derive_cl_typed(input)
}

// TODO: return compile error if enum is not flat

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let derived_input = parse_macro_input!(input as DeriveInput);
    match derived_input.data {
        syn::Data::Struct(_) => serialization::derive_from_bytes(derived_input),
        syn::Data::Enum(_) => serialization::derive_from_bytes_enum(derived_input),
        syn::Data::Union(_) => {
            TokenStream::from(quote! { compile_error!("Union types are not supported."); })
        }
    }
}

#[proc_macro_derive(ToBytes)]
pub fn derive_to_bytes(input: TokenStream) -> TokenStream {
    let derived_input = parse_macro_input!(input as DeriveInput);
    match derived_input.data {
        syn::Data::Struct(_) => serialization::derive_to_bytes(derived_input),
        syn::Data::Enum(_) => serialization::derive_to_bytes_enum(derived_input),
        syn::Data::Union(_) => {
            TokenStream::from(quote! { compile_error!("Union types are not supported."); })
        }
    }
}
