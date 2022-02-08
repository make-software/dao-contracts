extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

use crate::contract_interface::ContractTrait;

mod caller;
mod contract_impl;
mod contract_interface;

#[proc_macro_derive(Contract)]
pub fn derive_contract(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;

    let mut methods = TokenStream2::new();
    methods.append_all(contract_impl::generate());

    let expanded = quote! {
        impl #name {
            #methods
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Event)]
pub fn derive_events(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident.clone();
    let fields = match named_fields(input) {
        Ok(fields) => fields,
        Err(error_stream) => return error_stream,
    };

    let mut name_literal = TokenStream2::new();
    name_literal.append_all(quote! {
      stringify!(#struct_ident)
    });

    let mut deserialize_fields = TokenStream2::new();
    deserialize_fields.append_all(fields.iter().map(|ident| {
        quote! {
          let (#ident, bytes) = casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        }
    }));

    let mut construct_struct = TokenStream2::new();
    construct_struct.append_all(fields.iter().map(|ident| quote! { #ident, }));

    let mut sum_serialized_lengths = TokenStream2::new();
    sum_serialized_lengths.append_all(quote! {
      size += #name_literal.serialized_length();
    });
    sum_serialized_lengths.append_all(fields.iter().map(|ident| {
        quote! {
          size += self.#ident.serialized_length();
        }
    }));

    let mut append_bytes = TokenStream2::new();
    append_bytes.append_all(fields.iter().map(|ident| {
        quote! {
          vec.extend(self.#ident.to_bytes()?);
        }
    }));

    let mut type_check = TokenStream2::new();
    type_check.append_all(quote! {
      let (event_name, bytes): (String, _) = casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
      if &event_name != #name_literal {
          return core::result::Result::Err(casper_types::bytesrepr::Error::Formatting)
      }
    });

    TokenStream::from(quote! {
      impl casper_types::bytesrepr::FromBytes for #struct_ident {
        fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
          #type_check
          #deserialize_fields
          let value = #struct_ident {
            #construct_struct
          };
          Ok((value, bytes))
        }
      }

      impl casper_types::bytesrepr::ToBytes for #struct_ident {
        fn serialized_length(&self) -> usize {
          let mut size = 0;
          #sum_serialized_lengths
          return size;
        }

        fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
          let mut vec = Vec::with_capacity(self.serialized_length());
          vec.append(&mut #name_literal.to_bytes()?);
          #append_bytes
          Ok(vec)
        }
      }
    })
}

fn named_fields(input: DeriveInput) -> Result<Vec<proc_macro2::Ident>, TokenStream> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => named_fields
            .named
            .into_iter()
            .map(|x| x.ident.unwrap())
            .collect::<Vec<_>>(),
        _ => {
            return Err(TokenStream::from(
                quote! { compile_error!("Expected a struct with named fields."); },
            ))
        }
    };
    Ok(fields)
}

#[proc_macro]
pub fn generate_contract(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ContractTrait);

    let id = &input.ident;

    let interface_trait = contract_interface::generate(&input);
    let caller = caller::generate(id);

    let expanded = quote! {
      #interface_trait

      #caller
    };

    TokenStream::from(expanded)
}
