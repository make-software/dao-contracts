use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::TraitItemMethod;

use crate::contract_interface::ContractTrait;

pub fn generate(contract_trait: &ContractTrait) -> TokenStream {
    let name = contract_trait.ident.to_string();
    let parts: Vec<&str> = name.split("Interface").collect();
    let name = parts.first().unwrap();
    let ident = Ident::new(name, Span::call_site());

    let mut add_entry_points = TokenStream::new();
    add_entry_points.append_all(contract_trait.methods.iter().map(create_entry_point));

    quote! {
        impl #ident {

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

    let add_params = build_params(method);
    let group = build_group(ident);

    quote! {
        let mut params: Vec<casper_types::Parameter> = Vec::new();
        #add_params

        entry_points.add_entry_point(
            casper_types::EntryPoint::new(
                stringify!(#ident),
                params,
                <() as casper_types::CLTyped>::cl_type(),
                #group,
                casper_types::EntryPointType::Contract,
            )
        );
    }
}

fn build_group(method_ident: &Ident) -> TokenStream {
    if method_ident.to_string() == "init" {
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
    let type_paths: Vec<&syn::TypePath> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| -> Option<&syn::TypePath> {
            match arg {
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Type::Path(tp) = &*pat_type.ty {
                        Some(tp)
                    } else {
                        None
                    }
                }
                syn::FnArg::Receiver(_) => None,
            }
        })
        .collect();

    let method_idents: Vec<&Ident> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| -> Option<&Ident> {
            match arg {
                syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                    syn::Pat::Ident(pat_ident) => Some(&pat_ident.ident),
                    _ => None,
                },
                syn::FnArg::Receiver(_) => None,
            }
        })
        .collect();

    let mut push_params = TokenStream::new();
    for i in 0..method_idents.len() {
        let method_ident = method_idents.get(i).unwrap();
        let type_path = type_paths.get(i).unwrap();
        push_params.append_all(quote! {
            params.push(casper_types::Parameter::new(stringify!(#method_ident), <#type_path as casper_types::CLTyped>::cl_type()));
        });
    }

    quote! {
        #push_params
    }
}
