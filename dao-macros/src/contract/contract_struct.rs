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
                        let mut contract_instance = #caller_ident::at(casper_dao_utils::Address::from(contract_package_hash));
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
        stream.extend(quote! {
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
    use proc_macro2::TokenStream;
    use quote::{quote, TokenStreamExt};

    use super::CasperContractItem;

    pub fn generate_code(model: &CasperContractItem) -> TokenStream {
        let id = &model.ident;
        let contract_name = &model.contract_name();

        let mut methods = TokenStream::new();
        methods.append_all(&model.trait_methods);

        quote! {
            #[doc = "Defines the "]
            #[doc = #contract_name]
            #[doc = " contract's public interface."]
            pub trait #id {
                #methods
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use crate::contract::{
        contract_struct::{generate_entry_points, generate_install},
        utils,
    };

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
                            let mut contract_instance = ContractCaller::at(casper_dao_utils::Address::from(contract_package_hash));
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
                            let mut contract_instance = ContractCaller::at(casper_dao_utils::Address::from(contract_package_hash));
                            let arg1 = casper_dao_utils::casper_contract::contract_api::runtime::get_named_arg(stringify!(arg1));
                            let arg2 = casper_dao_utils::casper_contract::contract_api::runtime::get_named_arg(stringify!(arg2));
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
