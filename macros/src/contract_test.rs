use crate::contract::utils;
use crate::contract::ContractTrait;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;

pub fn generate_test_implementation(input: &ContractTrait) -> TokenStream {
    let contract_ident = &input.contract_ident;
    let contract_test_ident = &input.contract_test_ident;

    let args = utils::generate_empty_args();

    quote! {

        pub struct #contract_test_ident {
            env: casper_dao_utils::TestEnv,
            package_hash: casper_types::ContractPackageHash,
            data: #contract_ident,
        }

        impl #contract_test_ident {
            pub fn new(env: &casper_dao_utils::TestEnv) -> #contract_test_ident {
                env.deploy_wasm_file("reputation_contract.wasm", #args);
                let package_hash = env.get_contract_package_hash("reputation_contract_package_hash");
                #contract_test_ident {
                    env: env.clone(),
                    package_hash,
                    data: #contract_ident::default(),
                }
            }

            pub fn event<T: casper_types::bytesrepr::FromBytes>(&self, index: u32) -> T {
                let raw_event: casper_types::bytesrepr::Bytes = self.env.get_dict_value(self.package_hash, "events", index);
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

pub fn generate_test_interface(input: &ContractTrait) -> TokenStream {
    let ident = &input.ident;
    let contract_test_ident = &input.contract_test_ident;
    let methods = build_methods(input);

    quote! {
      impl #ident for #contract_test_ident {
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
                self.env.call_contract_package(
                    self.package_hash,
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
