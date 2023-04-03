use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{punctuated::Punctuated, token::Comma, FnArg, ReturnType};

use super::{parser::CasperContractItem, utils};

pub fn generate_code(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let contract_test_interface = generate_test_interface(input)?;
    let contract_test_impl = generate_test_implementation(input)?;

    Ok(quote! {
        #contract_test_impl
        #contract_test_interface
    })
}

fn generate_test_implementation(input: &CasperContractItem) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;
    let contract_ident = &input.contract_ident;
    let contract_test_ident = &input.contract_test_ident;
    let constructor = build_constructor(input)?;

    Ok(quote! {
        #[cfg(feature = "test-support")]
        #[doc = "A wrapper around [`"]
        #[doc = stringify!(#contract_ident)]
        #[doc = "`] to simplify testing."]
        #[doc = "Implements [`"]
        #[doc = stringify!(#ident)]
        #[doc = "`] and [`TestContract`](casper_dao_utils::TestContract)."]
        pub struct #contract_test_ident {
            env: casper_dao_utils::TestEnv,
            package_hash: casper_types::ContractPackageHash,
            data: #contract_ident,
        }

        #[cfg(feature = "test-support")]
        impl #contract_test_ident {
            #constructor
        }

        #[cfg(feature = "test-support")]
        impl casper_dao_utils::TestContract for #contract_test_ident {
            fn get_env(&self) -> &casper_dao_utils::TestEnv {
                &self.env
            }

            fn get_package_hash(&self) -> casper_types::ContractPackageHash {
                self.package_hash
            }

            fn address(&self) -> casper_dao_utils::Address {
                casper_dao_utils::Address::from(self.package_hash)
            }

            fn as_account(&mut self, account: casper_dao_utils::Address) -> &mut Self {
                self.env.as_account(account);
                self
            }

            fn as_nth_account(&mut self, account: usize) -> &mut Self {
                self.env.as_account(self.env.get_account(account));
                self
            }

            fn advance_block_time_by(&mut self, seconds: u64) -> &mut Self {
                self.env.advance_block_time_by(core::time::Duration::from_secs(seconds));
                self
            }

            fn events_count(&self) -> u32 {
                self.env.events_count(self.package_hash)
            }

            fn event<T: casper_types::bytesrepr::FromBytes>(&self, index: i32) -> T {
                self.env.event(self.package_hash, index)
            }

            fn assert_event_at<T: casper_types::bytesrepr::FromBytes + std::cmp::PartialEq + std::fmt::Debug>(&self, index: i32, event: T) {
                assert_eq!(self.event::<T>(index), event);
            }

            fn assert_last_event<T: casper_types::bytesrepr::FromBytes + std::cmp::PartialEq + std::fmt::Debug>(&self, event: T) {
                self.assert_event_at(-1, event);
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
            env.deploy_wasm_file(#wasm_file_name, #casper_args).unwrap();
            let package_hash = env.get_contract_package_hash(#package_hash);
            #contract_test_ident {
                env: env.clone(),
                package_hash,
                data: casper_dao_utils::Instanced::instance("contract"),
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
                parse_quote! { fn do_something(&mut self, amount: U512); },
                parse_quote! { fn init(&mut self); },
            ],
            ..utils::tests::mock_valid_item()
        };

        let expected = quote! {
            pub fn new(env: &casper_dao_utils::TestEnv,) -> ContractTest {
                env.deploy_wasm_file("contract_wasm", casper_types::RuntimeArgs::new()).unwrap();
                let package_hash = env.get_contract_package_hash("contract");
                ContractTest {
                    env: env.clone(),
                    package_hash,
                    data: casper_dao_utils::Instanced::instance ("contract"),
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
                ).unwrap();
                let package_hash = env.get_contract_package_hash("contract");
                ContractTest {
                    env: env.clone(),
                    package_hash,
                    data: casper_dao_utils::Instanced::instance("contract"),
                }
            }
        };
        pretty_assertions::assert_eq!(
            expected.to_string(),
            build_constructor(&item).unwrap().to_string(),
        );
    }
}
