use crate::contract::{utils, ContractTrait};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

pub fn generate_struct(input: &ContractTrait) -> TokenStream {
    let ident = &input.caller_ident;
    quote! {
      struct #ident {
        contract_package_hash: casper_types::ContractPackageHash,
      }

      impl #ident {
        pub fn at(contract_package_hash: casper_types::ContractPackageHash) -> Self {
            #ident {
                contract_package_hash,
            }
        }
      }
    }
}

pub fn generate_interface_impl(input: &ContractTrait) -> TokenStream {
    let ident = &input.ident;
    let caller_ident = &input.caller_ident;
    let methods = build_methods(input);

    quote! {
      impl #ident for #caller_ident {
        #methods
      }
    }
}

fn build_methods(input: &ContractTrait) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.append_all(input.methods.iter().map(|method| {
        let sig = &method.sig;
        let ident = &sig.ident;
        let args = utils::generate_method_args(method);
        quote! {
            #sig {
                let _: () = casper_contract::contract_api::runtime::call_versioned_contract(
                    self.contract_package_hash,
                    std::option::Option::None,
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
