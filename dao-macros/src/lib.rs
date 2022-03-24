extern crate proc_macro;

use proc_macro::TokenStream;

mod contract;
mod event;

/// Derive events on top of any struct.
#[proc_macro_derive(Event)]
pub fn derive_events(input: TokenStream) -> TokenStream {
    event::expand_derive_events(input)
}

#[proc_macro_attribute]
pub fn casper_contract_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate_code(item).into()
}
