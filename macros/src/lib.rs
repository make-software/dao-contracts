use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod rules;

/// Derives a validation rule boilerplate code on top of any struct.
#[proc_macro_derive(Rule)]
pub fn derive_rule(input: TokenStream) -> TokenStream {
    let derived_input = parse_macro_input!(input as DeriveInput);
    rules::expand_derive_rule(derived_input).into()
}
