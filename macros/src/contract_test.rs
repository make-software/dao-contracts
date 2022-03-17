use crate::contract::utils;
use crate::parser::ContractTrait;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use syn::ReturnType;

pub fn generate_test_implementation(input: &ContractTrait) -> TokenStream {
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

pub fn generate_test_interface(input: &ContractTrait) -> TokenStream {
    let ident = &input.ident;
    let contract_test_ident = &input.contract_test_ident;
    let methods = build_methods(input);

    quote! {
      #[cfg(feature = "test-support")]
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

        let mut return_value = TokenStream::new();
        //solves only the simplest but the most common case, consider handling arrys, tuples, generics
        return_value.append_all(match &sig.output {
            ReturnType::Default => quote! {},
            ReturnType::Type(_, ty) => {
                let unboxed_ty = &**ty;
                match unboxed_ty {
                    syn::Type::Path(path) => quote! { #path::default() },
                    _ => quote! {},
                }
            }
        });

        quote! {
            #sig {
                self.env.call_contract_package(
                    self.package_hash,
                    stringify!(#ident),
                    #args,
                );
                #return_value
            }
        }
    }));

    quote! {
        #stream
    }
}
