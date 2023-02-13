#![feature(iterator_try_collect)]
extern crate proc_macro;

use contract::CasperContractItem;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod contract;
mod instance;
mod rule;
mod serialization;

/// Derives [Instanced](../casper_dao_utils/trait.Instanced.html) boilerplate code on top of any struct.
#[proc_macro_derive(Instance, attributes(scoped))]
pub fn derive_instance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    instance::generate_code(input).into()
}

/// Generates contracts' `no_mangle` functions.
#[proc_macro_attribute]
pub fn casper_contract_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as CasperContractItem);
    contract::generate_code(item).into()
}

/// Derives [CLType](https://docs.rs/casper-types/1.5.0/casper_types/trait.CLTyped.html) boilerplate code on top of any struct.
#[proc_macro_derive(CLTyped)]
pub fn derive_cl_typed(input: TokenStream) -> TokenStream {
    let derived_input = parse_macro_input!(input as DeriveInput);
    match derived_input.data {
        syn::Data::Struct(_) => serialization::derive_cl_typed(derived_input),
        syn::Data::Enum(_) => serialization::derive_cl_typed_enum(derived_input),
        syn::Data::Union(_) => {
            TokenStream::from(quote! { compile_error!("Union types are not supported."); })
        }
    }
}

// TODO: return compile error if enum is not flat
/// Derives [FromBytes](https://docs.rs/casper-types/1.5.0/casper_types/bytesrepr/trait.FromBytes.html) boilerplate code on top of any struct.
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

/// Derives [ToBytes](https://docs.rs/casper-types/1.5.0/casper_types/bytesrepr/trait.ToBytes.html) boilerplate code on top of any struct.
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

/// Derives a validation rule boilerplate code on top of any struct.
#[proc_macro_derive(Rule)]
pub fn derive_rule(input: TokenStream) -> TokenStream {
    let derived_input = parse_macro_input!(input as DeriveInput);
    rule::expand_derive_rule(derived_input).into()
}
