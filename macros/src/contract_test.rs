use crate::contract::utils;
use crate::parser::CasperContractItem;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use syn::ReturnType;

pub fn generate_code(input: &CasperContractItem) -> TokenStream {
    let contract_test_interface = generate_test_interface(input);
    let contract_test_impl = generate_test_implementation(input);

    quote! {
        #contract_test_impl
        #contract_test_interface
    }
}

fn generate_test_implementation(input: &CasperContractItem) -> TokenStream {
    let contract_ident = &input.contract_ident;
    let contract_test_ident = &input.contract_test_ident;
    let args = utils::generate_empty_args();
    let wasm_file_name = &input.wasm_file_name;
    let package_hash = &input.package_hash;

    quote! {
        #[cfg(feature = "test-support")]
        pub struct #contract_test_ident {
            env: casper_dao_utils::TestEnv,
            package_hash: casper_types::ContractPackageHash,
            data: #contract_ident,
        }

        #[cfg(feature = "test-support")]
        impl #contract_test_ident {
            pub fn new(env: &casper_dao_utils::TestEnv) -> #contract_test_ident {
                env.deploy_wasm_file(#wasm_file_name, #args);
                let package_hash = env.get_contract_package_hash(#package_hash);
                #contract_test_ident {
                    env: env.clone(),
                    package_hash,
                    data: #contract_ident::default(),
                }
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
    }
}

fn generate_test_interface(input: &CasperContractItem) -> TokenStream {
    let contract_test_ident = &input.contract_test_ident;
    let methods = build_methods(input);

    quote! {
      #[cfg(feature = "test-support")]
      impl #contract_test_ident {
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
