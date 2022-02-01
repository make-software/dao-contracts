use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, TokenStream as TokenStream2};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

fn named_fields(input: DeriveInput) -> Result<Vec<Ident2>, TokenStream> {
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

#[proc_macro_derive(EventFromBytes)]
pub fn derive_from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident.clone();
    let fields = match named_fields(input) {
        Ok(fields) => fields,
        Err(error_stream) => return error_stream,
    };

    let mut deserialize_fields = TokenStream2::new();
    deserialize_fields.append_all(fields.iter().map(|ident| {
        quote! {
          let (#ident, bytes) = casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        }
    }));

    let mut construct_struct = TokenStream2::new();
    construct_struct.append_all(fields.iter().map(|ident| quote! { #ident, }));

    let expanded = quote! {
      impl casper_types::bytesrepr::FromBytes for #struct_ident {
        fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
          #deserialize_fields
          let value = #struct_ident {
            #construct_struct
          };
          Ok((value, bytes))
        }
      }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(EventToBytes)]
pub fn derive_to_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = input.ident.clone();
    let fields = match named_fields(input) {
        Ok(fields) => fields,
        Err(error_stream) => return error_stream,
    };

    let mut sum_serialized_lengths = TokenStream2::new();
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

    let expanded = quote! {
      impl casper_types::bytesrepr::ToBytes for #struct_ident {
        fn serialized_length(&self) -> usize {
          let mut size = 0;
          #sum_serialized_lengths
          return size;
        }

        fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
          let mut vec = Vec::with_capacity(self.serialized_length());
          #append_bytes
          Ok(vec)
        }
      }
    };

    TokenStream::from(expanded)
}
