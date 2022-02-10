use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::TraitItemMethod;

use crate::contract::{self, ContractTrait};

pub fn generate_struct(ident: &Ident) -> TokenStream {
    let caller_ident = generate_ident(ident);

    quote! {
      struct #caller_ident {
        contract_package_hash: casper_types::ContractPackageHash,
      }
    }
}

pub fn generate_interface_impl(input: &ContractTrait) -> TokenStream {
    let ident = &input.ident;
    let caller_ident = generate_ident(ident);
    let methods = build_methods(input);

    quote! {
      impl #ident for #caller_ident {
        #methods
      }
    }
}

fn generate_ident(base_ident: &Ident) -> Ident {
    let caller_name = format!("{}Caller", base_ident);
    Ident::new(&caller_name, Span::call_site())
}

fn build_methods(input: &ContractTrait) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.append_all(input.methods.iter().map(|method| {
        let sig = &method.sig;
        let ident = &sig.ident;
        let args = generate_args(method);
        quote! {
            #sig {
                let _: () = casper_contract::contract_api::runtime::call_versioned_contract(
                    self.contract_package_hash,
                    None,
                    stringify!(#ident),
                    #args,
                );
            }
        }
    }));

    quote! {
        #stream
    }
}

fn generate_args(method: &TraitItemMethod) -> TokenStream {
    let method_idents = contract::utils::collect_arg_idents(method);
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
