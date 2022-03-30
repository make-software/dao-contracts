use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::FnArg;
use syn::ReturnType;

use super::parser::CasperContractItem;
use super::utils;

pub fn generate_code(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_test_interface = generate_test_interface(input)?;
    let contract_test_impl = generate_test_implementation(input)?;

    Ok(quote! {
        #contract_test_impl
        #contract_test_interface
    })
}

fn generate_test_implementation(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_ident = &input.contract_ident;
    let contract_test_ident = &input.contract_test_ident;
    let contrustor = build_constructor(input)?;

    Ok(quote! {
        #[cfg(feature = "test-support")]
        pub struct #contract_test_ident {
            env: casper_dao_utils::TestEnv,
            package_hash: casper_types::ContractPackageHash,
            data: #contract_ident,
        }

        #[cfg(feature = "test-support")]
        impl #contract_test_ident {
            #contrustor

            pub fn get_package_hash(&self) -> casper_types::ContractPackageHash {
                self.package_hash
            }

            pub fn as_account(&mut self, account: casper_dao_utils::Address) -> &mut Self {
                self.env.as_account(account);
                self
            }

            pub fn event<T: casper_types::bytesrepr::FromBytes>(&self, index: u32) -> T {
                let raw_event: std::option::Option<casper_types::bytesrepr::Bytes> = self.env.get_dict_value(self.package_hash, "events", index);
                let raw_event = raw_event.unwrap();
                let (event, bytes) = T::from_bytes(&raw_event).unwrap();
                assert!(bytes.is_empty());
                event
            }

            pub fn assert_event_at<T: casper_types::bytesrepr::FromBytes + std::cmp::PartialEq + std::fmt::Debug>(&self, index: u32, event: T) {
                assert_eq!(self.event::<T>(index), event);
            }
        }
    })
}

fn build_constructor(item: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let init = match utils::find_method(item, "init") {
        Some(method) => method,
        None => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Contract has to define init() method",
            ))
        }
    };
    let casper_args = utils::generate_method_args(init);
    let mut args: Punctuated<FnArg, Comma> = Punctuated::new();
    init.sig
        .inputs
        .clone()
        .into_iter()
        .skip(1)
        .for_each(|arg| args.push(arg));

    let contract_test_ident = &item.contract_test_ident;
    let wasm_file_name = &item.wasm_file_name;
    let package_hash = &item.package_hash;

    Ok(quote! {
        pub fn new(env: &casper_dao_utils::TestEnv, #args) -> #contract_test_ident {
            env.deploy_wasm_file(#wasm_file_name, #casper_args);
            let package_hash = env.get_contract_package_hash(#package_hash);
            #contract_test_ident {
                env: env.clone(),
                package_hash,
                data: Default::default(),
            }
        }
    })
}

fn generate_test_interface(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_test_ident = &input.contract_test_ident;
    let methods = build_methods(input);

    Ok(quote! {
      #[cfg(feature = "test-support")]
      impl #contract_test_ident {
        #methods
      }
    })
}

fn build_methods(input: &CasperContractItem) -> TokenStream {
    let mut stream = TokenStream::new();
    stream.append_all(input.trait_methods.iter().map(|method| {
        let sig = &method.sig;
        let ident = &sig.ident;
        let args = utils::generate_method_args(method);
        let sig_inputs = &sig.inputs;
        match &sig.output {
            ReturnType::Default => quote! {
                pub fn #ident(#sig_inputs) -> Result<(), casper_dao_utils::Error> {
                    let result: Result<Option<()>, casper_dao_utils::Error> = self.env.call(
                        self.package_hash,
                        stringify!(#ident),
                        #args,
                        false
                    );
                    match result {
                        Ok(None) => Ok(()),
                        Ok(Some(_)) => panic!("Unexpected value on return."),
                        Err(err) => Err(err)
                    }
                }
            },
            ReturnType::Type(_, ty) => quote! {
                pub fn #ident(#sig_inputs) -> #ty {
                    let result: Result<Option<#ty>, casper_dao_utils::Error> = self.env.call::<#ty>(
                        self.package_hash,
                        stringify!(#ident),
                        #args,
                        true
                    );
                    result.unwrap().unwrap()
                }
            },
        }
    }));

    quote! {
        #stream
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use crate::contract::{contract_test::build_constructor, utils, CasperContractItem};

    #[test]
    fn generating_test_contract_constructor_works() {
        let item = CasperContractItem {
            trait_methods: vec![
                parse_quote! { fn do_something(&mut self, amount: U256); },
                parse_quote! { fn init(&mut self); },
            ],
            ..utils::tests::mock_valid_item()
        };

        let expected = quote! {
            pub fn new(env: &casper_dao_utils::TestEnv,) -> ContractTest {
                env.deploy_wasm_file("contract_wasm", casper_types::RuntimeArgs::new());
                let package_hash = env.get_contract_package_hash("contract");
                ContractTest {
                    env: env.clone(),
                    package_hash,
                    data: Default::default(),
                }
            }
        };

        pretty_assertions::assert_eq!(
            expected.to_string(),
            build_constructor(&item).unwrap().to_string(),
        );
    }

    #[test]
    fn generating_test_constructor_without_init_method_fails() {
        let invalid_item = utils::tests::mock_item_without_init();
        let result = build_constructor(&invalid_item)
            .map_err(|err| err.to_string())
            .unwrap_err();
        let expected = "Contract has to define init() method".to_string();
        assert_eq!(expected, result);
    }

    #[test]
    fn generating_test_constructor_with_args_works() {
        let item = utils::tests::mock_item_init_with_args();
        let expected = quote! {
            pub fn new(env: &casper_dao_utils::TestEnv, arg1: String, arg2: String) -> ContractTest {
                env.deploy_wasm_file(
                    "contract_wasm",
                    {
                        let mut named_args = casper_types::RuntimeArgs::new();
                        named_args.insert(stringify!(arg1), arg1).unwrap();
                        named_args.insert(stringify!(arg2), arg2).unwrap();
                        named_args
                    }
                );
                let package_hash = env.get_contract_package_hash("contract");
                ContractTest {
                    env: env.clone(),
                    package_hash,
                    data: Default::default(),
                }
            }
        };
        pretty_assertions::assert_eq!(
            expected.to_string(),
            build_constructor(&item).unwrap().to_string(),
        );
    }
}
