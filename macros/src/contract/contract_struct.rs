use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::TraitItemMethod;

use super::{parser::CasperContractItem, utils};

pub fn generate_code(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_install = generate_install(input)?;
    let contract_entry_points = generate_entry_points(input);

    Ok(quote! {
        #contract_install
        #contract_entry_points
    })
}

fn generate_install(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_ident = &input.contract_ident;
    let caller_ident = &input.caller_ident;
    let package_hash = &input.package_hash;

    let init_method = match utils::find_method(input, "init") {
        Some(method) => method,
        None => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Contract has to define init() method",
            ))
        }
    };

    let (args_stream, punctuated_args) = utils::parse_casper_args(init_method);

    Ok(quote! {
        impl #contract_ident {
            pub fn install() {
                casper_dao_utils::casper_env::install_contract(
                    #package_hash,
                    #contract_ident::entry_points(),
                    |contract_package_hash| {
                        let mut contract_instance = #caller_ident::at(contract_package_hash);
                        #args_stream
                        contract_instance.init( #punctuated_args );
                    }
                );
            }
        }
    })
}

fn generate_entry_points(contract_trait: &CasperContractItem) -> TokenStream {
    let contract_ident = &contract_trait.contract_ident;
    let mut add_entry_points = TokenStream::new();
    add_entry_points.append_all(contract_trait.trait_methods.iter().map(build_entry_point));

    quote! {
        impl #contract_ident {
            pub fn entry_points() -> casper_types::EntryPoints {
                let mut entry_points = casper_types::EntryPoints::new();
                #add_entry_points
                entry_points
            }
        }
    }
}

fn build_entry_point(method: &TraitItemMethod) -> TokenStream {
    let method_ident = &method.sig.ident;
    let params = build_entry_point_params(method);
    let group = build_group(method_ident);

    quote! {
        entry_points.add_entry_point(
            casper_types::EntryPoint::new(
                stringify!(#method_ident),
                #params,
                <() as casper_types::CLTyped>::cl_type(),
                #group,
                casper_types::EntryPointType::Contract,
            )
        );
    }
}

fn build_group(method_ident: &Ident) -> TokenStream {
    if &*method_ident.to_string() == "init" {
        quote! {
            casper_types::EntryPointAccess::Groups(vec![casper_types::Group::new("init")])
        }
    } else {
        quote! {
            casper_types::EntryPointAccess::Public
        }
    }
}

fn build_entry_point_params(method: &TraitItemMethod) -> TokenStream {
    let type_paths = utils::collect_type_paths(method);
    let method_idents = utils::collect_arg_idents(method);

    let mut stream = TokenStream::new();
    for i in 0..method_idents.len() {
        let method_ident = method_idents.get(i).unwrap();
        let type_path = type_paths.get(i).unwrap();
        stream.append_all(quote! {
            params.push(casper_types::Parameter::new(stringify!(#method_ident), <#type_path as casper_types::CLTyped>::cl_type()));
        });
    }

    quote! {
        {
            let mut params: Vec<casper_types::Parameter> = Vec::new();
            #stream
            params
        }
    }
}

pub mod interface {
    use super::CasperContractItem;
    use proc_macro2::TokenStream;
    use quote::{quote, TokenStreamExt};

    pub fn generate_code(model: &CasperContractItem) -> TokenStream {
        let id = &model.ident;

        let mut methods = TokenStream::new();
        methods.append_all(&model.trait_methods);

        quote! {
            pub trait #id {
                #methods
            }
        }
    }
}

pub mod utils {
    use crate::parser::CasperContractItem;
    use proc_macro2::{Ident, Span, TokenStream};
    use quote::{format_ident, quote, TokenStreamExt};
    use syn::{punctuated::Punctuated, token, FnArg, Pat, Token, TraitItemMethod, Type, TypePath};

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

    pub fn find_method<'a>(
        input: &'a CasperContractItem,
        method_name: &str,
    ) -> Option<&'a TraitItemMethod> {
        input
            .trait_methods
            .iter()
            .find(|method| method.sig.ident == *method_name)
    }

    pub fn parse_casper_args(
        method: &TraitItemMethod,
    ) -> (TokenStream, Punctuated<Ident, Token![,]>) {
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
}

#[cfg(test)]
mod tests {
    use crate::contract::{
        contract_struct::{generate_entry_points, generate_install},
        utils,
    };
    use pretty_assertions::assert_eq;
    use quote::quote;

    #[test]
    fn generating_install_without_init_method_fails() {
        let valid_item = utils::tests::mock_item_without_init();
        let result = generate_install(&valid_item)
            .map_err(|err| err.to_string())
            .unwrap_err();
        let expected = "Contract has to define init() method".to_string();
        assert_eq!(expected, result);
    }

    #[test]
    fn generating_install_no_args() {
        let valid_item = utils::tests::mock_valid_item();
        let result = generate_install(&valid_item).unwrap().to_string();
        let expected = quote! {
            impl Contract {
                pub fn install() {
                    casper_dao_utils::casper_env::install_contract(
                        "contract",
                        Contract::entry_points(),
                        |contract_package_hash| {
                            let mut contract_instance = ContractCaller::at(contract_package_hash);
                            contract_instance.init();
                        }
                    );
                }
            }
        }
        .to_string();
        assert_eq!(expected, result);
    }

    #[test]
    fn generating_install_with_args() {
        let valid_item = utils::tests::mock_item_init_with_args();
        let result = generate_install(&valid_item).unwrap().to_string();
        let expected = quote! {
            impl Contract {
                pub fn install() {
                    casper_dao_utils::casper_env::install_contract(
                        "contract",
                        Contract::entry_points(),
                        |contract_package_hash| {
                            let mut contract_instance = ContractCaller::at(contract_package_hash);
                            let arg1 = casper_contract::contract_api::runtime::get_named_arg(stringify!(arg1));
                            let arg2 = casper_contract::contract_api::runtime::get_named_arg(stringify!(arg2));
                            contract_instance.init(arg1, arg2,);
                        }
                    );
                }
            }
        }
        .to_string();
        assert_eq!(expected, result);
    }
    
    #[test]
    fn generating_entry_points_works() {
        let valid_item = utils::tests::mock_valid_item();
        let result = generate_entry_points(&valid_item).to_string();
        let expected = quote! {
            impl Contract {
                pub fn entry_points() -> casper_types::EntryPoints {
                    let mut entry_points = casper_types::EntryPoints::new();
                    entry_points.add_entry_point(
                        casper_types::EntryPoint::new(
                            stringify!(init),
                            {
                                let mut params: Vec<casper_types::Parameter> = Vec::new();
                                params
                            },
                            <() as casper_types::CLTyped>::cl_type(),
                            casper_types::EntryPointAccess::Groups(vec![casper_types::Group::new("init")]),
                            casper_types::EntryPointType::Contract,
                        )
                    );
                    entry_points.add_entry_point(
                        casper_types::EntryPoint::new(
                            stringify!(do_something),
                            {
                                let mut params: Vec<casper_types::Parameter> = Vec::new();
                                params.push(casper_types::Parameter::new(stringify!(amount), <U256 as casper_types::CLTyped>::cl_type()));
                                params
                            },
                            <() as casper_types::CLTyped>::cl_type(),
                            casper_types::EntryPointAccess::Public,
                            casper_types::EntryPointType::Contract,
                        )
                    );
                    entry_points
                }
            }
        }.to_string();
        assert_eq!(expected, result);
    }
}
