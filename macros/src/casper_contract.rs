use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

use crate::{contract::utils, parser::CasperContractItem};

pub fn generate_code(input: &CasperContractItem) -> TokenStream {
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

fn generate_install(contract: &CasperContractItem) -> TokenStream {
    let contract_ident = &contract.contract_ident;

    quote! {
        #[no_mangle]
        fn call() {
            #contract_ident::install();
        }
    }
}

fn generate_interface_methods(contract: &CasperContractItem) -> TokenStream {
    let contract_ident = &contract.contract_ident;
    let contract_interface_ident = &contract.ident;

    let mut methods = TokenStream::new();
    methods.append_all(contract.trait_methods.iter().map(|method| {
        let ident = &method.sig.ident;
        let (casper_args, punctuated_args) = utils::parse_casper_args(method);
        let has_return = matches!(&method.sig.output, syn::ReturnType::Type(_, _));
        if has_return {
            quote! {
                #[no_mangle]
                fn #ident() {
                    use casper_contract::unwrap_or_revert::UnwrapOrRevert;

                    #casper_args
                    let contract = #contract_ident::default();
                    let result = #contract_interface_ident::#ident(&contract, #punctuated_args);
                    let result = casper_types::CLValue::from_t(result).unwrap_or_revert();
                    casper_contract::contract_api::runtime::ret(result);
                }
            }
        } else {
            quote! {
                #[no_mangle]
                fn #ident() {
                    #casper_args
                    let mut contract = #contract_ident::default();
                    #contract_interface_ident::#ident(&mut contract, #punctuated_args);
                }
            }
        }
    }));
    methods
}
