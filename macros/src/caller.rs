use crate::{contract::utils, parser::CasperContract};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

pub fn generate_struct(input: &CasperContract) -> TokenStream {
    let ident = &input.caller_ident;
    quote! {
      pub struct #ident {
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

pub fn generate_interface_impl(input: &CasperContract) -> TokenStream {
    let ident = &input.ident;
    let caller_ident = &input.caller_ident;
    let methods = build_methods(input);

    quote! {
      impl #ident for #caller_ident {
        #methods
      }
    }
}

fn build_methods(input: &CasperContract) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.append_all(input.trait_methods.iter().map(|method| {
        let sig = &method.sig;
        let ident = &sig.ident;
        let args = utils::generate_method_args(method);

        if &ident.to_string() == "init" {
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
        } else {
            quote! {
                #sig {
                    casper_contract::contract_api::runtime::call_versioned_contract(
                        self.contract_package_hash,
                        std::option::Option::None,
                        stringify!(#ident),
                        #args,
                    )
                }
            }
        }
    }));

    quote! {
        #stream
    }
}
