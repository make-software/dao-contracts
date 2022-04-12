use proc_macro::TokenStream;
use proc_macro2::{Ident as Ident2, TokenStream as TokenStream2};
use quote::{quote, TokenStreamExt};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields};

pub fn derive_cl_typed(input: DeriveInput) -> TokenStream {
    let ident = input.ident;

    let expanded = quote! {
      impl casper_types::CLTyped for #ident {
        fn cl_type() -> casper_types::CLType {
          casper_types::CLType::Any
        }
      }
    };

    TokenStream::from(expanded)
}

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

pub fn derive_from_bytes(input: DeriveInput) -> TokenStream {
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
        fn from_bytes(bytes: &[u8]) -> std::result::Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
          #deserialize_fields
          let value = #struct_ident {
            #construct_struct
          };
          std::result::Result::Ok((value, bytes))
        }
      }
    };

    TokenStream::from(expanded)
}

pub fn derive_to_bytes(input: DeriveInput) -> TokenStream {
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

        fn to_bytes(&self) -> std::result::Result<std::vec::Vec<u8>, casper_types::bytesrepr::Error> {
          let mut vec = Vec::with_capacity(self.serialized_length());
          #append_bytes
          std::result::Result::Ok(vec)
        }
      }
    };

    TokenStream::from(expanded)
}

fn variants(input: DeriveInput) -> Result<Vec<Ident2>, TokenStream> {
    let fields = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            variants.into_iter().map(|x| &x.fields).collect::<Vec<_>>()
        }
        _ => {
            return Err(TokenStream::from(
                quote! { compile_error!("Expected an enum."); },
            ))
        }
    };

    for f in fields.into_iter() {
        if *f != Fields::Unit {
            return Err(TokenStream::from(
                quote! { compile_error!("Expected an enum with unit variants."); },
            ));
        }
    }

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            variants.into_iter().map(|x| x.ident).collect::<Vec<_>>()
        }
        _ => {
            return Err(TokenStream::from(
                quote! { compile_error!("Expected an enum."); },
            ))
        }
    };
    Ok(variants)
}

pub fn derive_to_bytes_enum(input: DeriveInput) -> TokenStream {
    let enum_ident = input.ident.clone();
    let variants = match variants(input) {
        Ok(variants) => variants,
        Err(error_stream) => return error_stream,
    };

    let mut append_bytes = TokenStream2::new();
    append_bytes.append_all(variants.iter().enumerate().map(|(index, ident)| {
        let index = (index + 1) as u32;
        quote! {
          #enum_ident::#ident => #index,
        }
    }));

    let expanded = quote! {
      impl casper_types::bytesrepr::ToBytes for #enum_ident {
        fn serialized_length(&self) -> usize {
          1
        }

        fn to_bytes(&self) -> std::result::Result<std::vec::Vec<u8>, casper_types::bytesrepr::Error> {
          let mut vec = std::vec::Vec::with_capacity(self.serialized_length());
          vec.append(
            &mut match self {
                #append_bytes
              }
              .to_bytes()?,
          );
          std::result::Result::Ok(vec)
        }
      }
    };

    TokenStream::from(expanded)
}

pub fn derive_from_bytes_enum(input: DeriveInput) -> TokenStream {
    let enum_ident = input.ident.clone();
    let variants = match variants(input) {
        Ok(variants) => variants,
        Err(error_stream) => return error_stream,
    };

    let mut append_bytes = TokenStream2::new();
    append_bytes.append_all(variants.iter().enumerate().map(|(index, ident)| {
        let index: u32 = index as u32 + 1;
        quote! {
          #index => std::result::Result::Ok((#enum_ident::#ident, bytes)),
        }
    }));

    let expanded = quote! {
      impl casper_types::bytesrepr::FromBytes for #enum_ident {
        fn from_bytes(bytes: &[u8]) -> std::result::Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
          let (variant, bytes) = casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
          match variant {
            #append_bytes
            _ => std::result::Result::Err(casper_types::bytesrepr::Error::Formatting),
          }
        }
      }
    };

    TokenStream::from(expanded)
}
