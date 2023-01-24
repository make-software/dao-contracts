use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

use super::CasperContractItem;
use crate::contract::utils;

pub fn generate_code(input: &CasperContractItem) -> TokenStream {
    let contract_ident = &input.contract_ident;
    let macro_ident = format_ident!(
        "{}",
        contract_ident
            .to_string()
            .to_case(convert_case::Case::Snake)
    );

    let call = generate_call(input);
    let interface_methods = generate_interface_methods(input);
    let docs = match macro_ident.to_string().contains("mock") {
        true => quote! { #[doc(hidden)]},
        false => quote! {
            #[doc = "Generates a "]
            #[doc = stringify!(#contract_ident)]
            #[doc = " binary with all the required no_mangle functions."]
        },
    };
    quote! {
        #docs
        #[macro_export]
        macro_rules! #macro_ident {
            () => {
                #call

                #interface_methods
            };
        }
    }
}

fn generate_call(contract: &CasperContractItem) -> TokenStream {
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
        let mutability = method.sig.inputs
            .iter()
            .filter_map(|i| match i {
                syn::FnArg::Receiver(r) => r.mutability,
                syn::FnArg::Typed(_) => None,
            }).collect::<Vec<_>>();
        let mutability_token = match mutability.first() {
            Some(m) => quote!(#m),
            None => quote!(),
        };
        if has_return {
            quote! {
                #[no_mangle]
                fn #ident() {
                    use casper_dao_utils::casper_contract::unwrap_or_revert::UnwrapOrRevert;

                    #casper_args
                    let #mutability_token contract: #contract_ident = casper_dao_utils::instance::Instanced::instance("contract");
                    let result = #contract_interface_ident::#ident(&#mutability_token contract, #punctuated_args);
                    let result = casper_types::CLValue::from_t(result).unwrap_or_revert_with(casper_dao_utils::Error::CLValueError);
                    casper_dao_utils::casper_contract::contract_api::runtime::ret(result);
                }
            }
        } else {
            quote! {
                #[no_mangle]
                fn #ident() {
                    #casper_args
                    let #mutability_token contract: #contract_ident = casper_dao_utils::instance::Instanced::instance("contract");
                    #[allow(clippy::unnecessary_mut_passed)]
                    #contract_interface_ident::#ident(&#mutability_token contract, #punctuated_args);
                }
            }
        }
    }));
    methods
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use super::generate_code;
    use crate::contract::utils::tests::mock_valid_item;

    #[test]
    fn generating_no_mangles_works() {
        let item = mock_valid_item();
        let generated = generate_code(&item);

        let expected = quote! {
            #[doc = "Generates a "]
            #[doc = stringify!(Contract)]
            #[doc = " binary with all the required no_mangle functions."]
            #[macro_export]
            macro_rules! contract {
                () => {
                    #[no_mangle]
                    fn call() {
                        Contract::install();
                    }

                    #[no_mangle]
                    fn init() {
                        let mut contract: Contract = casper_dao_utils::instance::Instanced::instance("contract");
                        #[allow(clippy::unnecessary_mut_passed)]
                        ContractTrait::init(&mut contract,);
                    }

                    #[no_mangle]
                    fn do_something() {
                        let amount = casper_dao_utils::casper_contract::contract_api::runtime::get_named_arg(stringify!(amount));
                        let mut contract: Contract = casper_dao_utils::instance::Instanced::instance("contract");
                        #[allow(clippy::unnecessary_mut_passed)]
                        ContractTrait::do_something(&mut contract, amount,);
                    }
                };
            }
        };

        assert_eq!(expected.to_string(), generated.to_string())
    }
}
