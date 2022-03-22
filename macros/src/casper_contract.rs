use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

use crate::{contract::utils, parser::CasperContract};

pub fn generate_macro(input: &CasperContract) -> TokenStream {
    let macro_ident = format_ident!(
        "{}",
        &input
            .contract_ident
            .to_string()
            .to_case(convert_case::Case::Snake)
    );

    let install = generate_install(input);
    let interface_methods = generate_interface_methods(input);

    quote! {
        #[macro_export]
        macro_rules! #macro_ident {
            () => {
                #install

                #interface_methods
            };
        }
    }
}

fn generate_install(contract: &CasperContract) -> TokenStream {
    let contract_ident = &contract.contract_ident;

    quote! {
        #[no_mangle]
        fn call() {
            #contract_ident::install();
        }
    }
}

fn generate_interface_methods(contract: &CasperContract) -> TokenStream {
    let contract_ident = &contract.contract_ident;
    let contract_interface_ident = &contract.ident;

    let mut methods = TokenStream::new();
    methods.append_all(contract.trait_methods.iter().map(|method| {
        let ident = &method.sig.ident;
        let (casper_args, punctuated_args) = utils::parse_casper_args(method);

        quote! {
            #[no_mangle]
            fn #ident() {
                #casper_args
                let mut contract = #contract_ident::default();
                #contract_interface_ident::#ident(&mut contract, #punctuated_args);
            }
        }
    }));
    methods
}
