use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::{parser::CasperContractItem, utils};

pub fn generate_code(input: &CasperContractItem) -> TokenStream {
    let struct_stream = generate_struct(input);
    let struct_impl_stream = generate_interface_impl(input);
    quote! {
      #struct_stream

      #struct_impl_stream
    }
}

fn generate_struct(input: &CasperContractItem) -> TokenStream {
    let contract_name = &input.contract_name();
    let ident = &input.caller_ident;
    quote! {
      #[doc = "Provides a reference to a deployed "]
      #[doc = #contract_name]
      #[doc = "."]
      pub struct #ident {
        contract_package_hash: casper_types::ContractPackageHash,
      }

      impl #ident {
        /// Creates a new caller instance from the given address.
        pub fn at(address: casper_dao_utils::Address) -> Self {
          Self {
              contract_package_hash: *address.as_contract_package_hash().unwrap(),
          }
        }
      }
    }
}

fn generate_interface_impl(input: &CasperContractItem) -> TokenStream {
    let ident = &input.ident;
    let caller_ident = &input.caller_ident;
    let methods = build_methods(input);

    quote! {
      impl #ident for #caller_ident {
        #methods
      }
    }
}

fn build_methods(input: &CasperContractItem) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.append_all(input.trait_methods.iter().map(|method| {
        let sig = &method.sig;
        let ident = &sig.ident;
        let args = utils::generate_method_args(method);

        if &ident.to_string() == "init" {
            quote! {
              #sig {
                let _: () = casper_dao_utils::casper_contract::contract_api::runtime::call_versioned_contract(
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
                    casper_dao_utils::casper_contract::contract_api::runtime::call_versioned_contract(
                        self.contract_package_hash,
                        std::option::Option::None,
                        stringify!(#ident),
                        #args,
                    )
                }
            }
        }
    }));
    stream
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quote::quote;

    use super::{generate_interface_impl, generate_struct};
    use crate::contract::utils;

    #[test]
    fn generating_caller_struct_works() {
        let item = utils::tests::mock_valid_item();
        let struct_stream = generate_struct(&item);

        let expected = quote! {
          #[doc = "Provides a reference to a deployed "]
          #[doc = "Contract"]
          #[doc = "."]
          pub struct ContractCaller {
            contract_package_hash: casper_types::ContractPackageHash,
          }

          impl ContractCaller {
            #[doc = r" Creates a new caller instance from the given address."]
            pub fn at(address: casper_dao_utils::Address) -> Self {
              Self {
                  contract_package_hash: *address.as_contract_package_hash().unwrap(),
              }
            }
          }
        };

        assert_eq!(struct_stream.to_string(), expected.to_string());
    }

    #[test]
    fn generating_caller_impl_works() {
        let item = utils::tests::mock_valid_item();
        let impl_stream = generate_interface_impl(&item);

        let expected = quote! {
          impl ContractTrait for ContractCaller {
            fn init(&mut self) {
              let _: () = casper_dao_utils::casper_contract::contract_api::runtime::call_versioned_contract(
                self.contract_package_hash,
                std::option::Option::None,
                stringify!(init),
                casper_types::RuntimeArgs::new(),
              );
            }

            fn do_something(&mut self, amount: U256) {
              casper_dao_utils::casper_contract::contract_api::runtime::call_versioned_contract(
                self.contract_package_hash,
                std::option::Option::None,
                stringify!(do_something),
                {
                  let mut named_args = casper_types::RuntimeArgs::new();
                  named_args.insert(stringify!(amount), amount).unwrap();
                  named_args
                },
              )
            }
          }
        };
        assert_eq!(impl_stream.to_string(), expected.to_string());
    }
}
