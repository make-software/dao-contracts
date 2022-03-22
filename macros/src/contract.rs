use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::TraitItemMethod;

use crate::parser::ContractTrait;

pub fn generate_install(input: &ContractTrait) -> TokenStream {
    let ident = &input.contract_ident;
    let caller_ident = &input.caller_ident;
    let package_hash = &input.package_hash;

    let init_method = utils::find_method(input, "init").unwrap();
    let (args_stream, punctuated_args) = utils::parse_casper_args(init_method);

    quote! {
        impl #ident {
            pub fn install() {
                casper_dao_utils::casper_env::install_contract(
                    #package_hash,
                    #ident::entry_points(),
                    |contract_package_hash| {
                        let mut contract_instance = #caller_ident::at(contract_package_hash);
                        #args_stream
                        contract_instance.init( #punctuated_args );
                    }
                );
            }
        }
    }
}

pub fn generate_entry_points(contract_trait: &ContractTrait) -> TokenStream {
    let mut add_entry_points = TokenStream::new();
    add_entry_points.append_all(contract_trait.methods.iter().map(create_entry_point));
    let contract_ident = &contract_trait.contract_ident;

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

fn create_entry_point(method: &TraitItemMethod) -> TokenStream {
    let ident = &method.sig.ident;
    let params = build_params(method);
    let group = build_group(ident);

    quote! {
        entry_points.add_entry_point(
            casper_types::EntryPoint::new(
                stringify!(#ident),
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

fn build_params(method: &TraitItemMethod) -> TokenStream {
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
    use super::ContractTrait;
    use proc_macro2::TokenStream;
    use quote::{quote, TokenStreamExt};

    pub fn generate_trait(model: &ContractTrait) -> TokenStream {
        let id = &model.ident;

        let mut methods = TokenStream::new();
        methods.append_all(&model.methods);

        quote! {
            pub trait #id {
                #methods
            }
        }
    }
}

pub mod utils {
    use proc_macro2::{Ident, Span, TokenStream};
    use quote::{format_ident, quote, TokenStreamExt};
    use syn::{punctuated::Punctuated, token, FnArg, Pat, Token, TraitItemMethod, Type, TypePath};

    use crate::parser::ContractTrait;

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

    pub fn generate_empty_args() -> TokenStream {
        quote! {
            {
                let mut named_args = casper_types::RuntimeArgs::new();
                named_args
            }
        }
    }

    pub fn find_method<'a>(
        input: &'a ContractTrait,
        method_name: &'static str,
    ) -> Option<&'a TraitItemMethod> {
        input
            .methods
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
                    let #ident = casper_contract::contract_api::runtime::get_named_arg(stringify!(#ident));
                });
            });

        (casper_args, punctuated_args)
    }
}
