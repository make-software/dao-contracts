use convert_case::Casing;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{punctuated::Punctuated, token, Token, TraitItemMethod};

use crate::{contract::utils::collect_arg_idents, parser::ContractTrait};

pub fn generate_macro(input: &ContractTrait) -> TokenStream {
    let macro_ident = format_ident!(
        "{}",
        &input
            .contract_ident
            .to_string()
            .to_case(convert_case::Case::Snake)
    );

    let install = generate_install(&input);
    let interface_methods = generate_interface_methods(&input);

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

fn generate_install(contract: &ContractTrait) -> TokenStream {
    let contract_ident = &contract.contract_ident;

    quote! {
        #[no_mangle]
        fn call() {
            casper_dao_contracts::#contract_ident::install();
        }
    }
}

fn generate_interface_methods(contract: &ContractTrait) -> TokenStream {
    let contract_ident = &contract.contract_ident;
    let contract_interface_ident = &contract.ident;

    let mut methods = TokenStream::new();
    methods.append_all(contract.methods.iter().map(|method| {
        let ident = &method.sig.ident;
        let (casper_args, punctuated_args) = parse_args(method);
    
        quote! {
            #[no_mangle]
            fn #ident() {
                #casper_args
                let mut contract = casper_dao_contracts::#contract_ident::default();
                casper_dao_contracts::#contract_interface_ident::#ident(&mut contract, #punctuated_args);
            }
        }
    }));
    methods
}

fn parse_args(method: &TraitItemMethod) -> (TokenStream, Punctuated<Ident, Token![,]>) {
    let comma = token::Comma([Span::call_site()]);

    let mut punctuated_args: Punctuated<Ident, Token![,]> = Punctuated::new();
    let mut casper_args = TokenStream::new();

    collect_arg_idents(method).iter().for_each(|ident| {
        punctuated_args.push_value(format_ident!("{}", ident));
        punctuated_args.push_punct(comma);

        casper_args.append_all(quote! {
            let #ident = casper_contract::contract_api::runtime::get_named_arg(stringify!(#ident));
        });
    });

    (casper_args, punctuated_args)
}
