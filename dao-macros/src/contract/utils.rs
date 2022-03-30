use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};
use syn::{punctuated::Punctuated, token, FnArg, Pat, Token, TraitItemMethod, Type, TypePath};

use crate::contract::parser::CasperContractItem;

pub fn collect_type_paths(method: &TraitItemMethod) -> Vec<&TypePath> {
    method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| -> Option<&TypePath> {
            match arg {
                FnArg::Typed(pat_type) => {
                    if let Type::Path(tp) = &*pat_type.ty {
                        Some(tp)
                    } else {
                        None
                    }
                }
                FnArg::Receiver(_) => None,
            }
        })
        .collect()
}

pub fn collect_arg_idents(method: &TraitItemMethod) -> Vec<&Ident> {
    method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| -> Option<&Ident> {
            match arg {
                FnArg::Typed(pat_type) => match &*pat_type.pat {
                    Pat::Ident(pat_ident) => Some(&pat_ident.ident),
                    _ => None,
                },
                FnArg::Receiver(_) => None,
            }
        })
        .collect()
}

pub fn generate_method_args(method: &TraitItemMethod) -> TokenStream {
    let method_idents = collect_arg_idents(method);
    if method_idents.is_empty() {
        quote! {
            casper_types::RuntimeArgs::new()
        }
    } else {
        let mut args = TokenStream::new();
        args.append_all(method_idents.iter().map(|ident| {
            quote! {
                named_args.insert(stringify!(#ident), #ident).unwrap();
            }
        }));
        quote! {
            {
                let mut named_args = casper_types::RuntimeArgs::new();
                #args
                named_args
            }
        }
    }
}

pub fn find_method<'a>(
    input: &'a CasperContractItem,
    method_name: &str,
) -> Option<&'a TraitItemMethod> {
    input
        .trait_methods
        .iter()
        .find(|method| method.sig.ident == *method_name)
}

pub fn parse_casper_args(method: &TraitItemMethod) -> (TokenStream, Punctuated<Ident, Token![,]>) {
    let comma = token::Comma([Span::call_site()]);

    let mut punctuated_args: Punctuated<Ident, Token![,]> = Punctuated::new();
    let mut casper_args = TokenStream::new();

    collect_arg_idents(method)
        .iter()
        .for_each(|ident| {
            punctuated_args.push_value(format_ident!("{}", ident));
            punctuated_args.push_punct(comma);

            casper_args.append_all(quote! {
                let #ident = casper_dao_utils::casper_contract::contract_api::runtime::get_named_arg(stringify!(#ident));
            });
        });

    (casper_args, punctuated_args)
}

#[cfg(test)]
pub mod tests {
    use quote::format_ident;
    use syn::parse_quote;

    use crate::contract::parser::CasperContractItem;

    pub fn mock_valid_item() -> CasperContractItem {
        CasperContractItem {
            trait_token: Default::default(),
            ident: format_ident!("{}", "ContractTrait"),
            trait_methods: vec![
                parse_quote! { fn init(&mut self); },
                parse_quote! { fn do_something(&mut self, amount: U256); },
            ],
            caller_ident: format_ident!("{}", "ContractCaller"),
            contract_ident: format_ident!("{}", "Contract"),
            contract_test_ident: format_ident!("{}", "ContractTest"),
            package_hash: "contract".to_string(),
            wasm_file_name: "contract_wasm".to_string(),
        }
    }

    pub fn mock_item_without_init() -> CasperContractItem {
        CasperContractItem {
            trait_methods: vec![
                parse_quote! { fn contrustor(&mut self); },
                parse_quote! { fn do_something(&mut self, amount: U256); },
            ],
            ..mock_valid_item()
        }
    }

    pub fn mock_item_init_with_args() -> CasperContractItem {
        CasperContractItem {
            trait_methods: vec![
                parse_quote! { fn init(&mut self, arg1: String, arg2: String); },
                parse_quote! { fn do_something(&mut self, amount: U256); },
            ],
            ..mock_valid_item()
        }
    }
}
