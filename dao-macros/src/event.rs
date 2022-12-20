use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn expand_derive_events(input: DeriveInput) -> TokenStream {
    let struct_ident = input.ident.clone();
    let fields = match named_fields_with_types(input) {
        Ok(fields) => fields,
        Err(error_stream) => return error_stream,
    };

    let mut name_literal = TokenStream::new();
    name_literal.append_all(quote! {
      stringify!(#struct_ident)
    });

    let mut deserialize_fields = TokenStream::new();
    deserialize_fields.append_all(fields.iter().map(|(ident, _)| {
        quote! {
          let (#ident, bytes) = casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        }
    }));

    let mut construct_struct = TokenStream::new();
    construct_struct.append_all(fields.iter().map(|(ident, _)| quote! { #ident, }));

    let mut sum_serialized_lengths = TokenStream::new();
    sum_serialized_lengths.append_all(quote! {
      size += #name_literal.serialized_length();
    });
    sum_serialized_lengths.append_all(fields.iter().map(|(ident, _)| {
        quote! {
          size += self.#ident.serialized_length();
        }
    }));

    let mut append_bytes = TokenStream::new();
    append_bytes.append_all(fields.iter().map(|(ident, _)| {
        quote! {
          vec.extend(self.#ident.to_bytes()?);
        }
    }));

    let mut type_check = TokenStream::new();
    type_check.append_all(quote! {
      let (event_name, bytes): (String, _) = casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
      if &event_name != #name_literal {
          return core::result::Result::Err(casper_types::bytesrepr::Error::Formatting)
      }
    });

    let mut event_def = TokenStream::new();
    event_def.append_all(quote! {
      casper_dao_utils::definitions::EventDef::new(stringify!(#struct_ident))
    });
    event_def.append_all(fields.iter().map(|(ident, ty)| {
        quote! {
          .with_field(casper_dao_utils::definitions::ElemDef::new::<#ty>(stringify!(#ident)))
        }
    }));

    quote! {
      impl #struct_ident {
        pub fn emit(self) {
          casper_dao_utils::casper_env::emit(self);
        }
      }

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

      impl casper_dao_utils::definitions::EventDefinition for #struct_ident {
        fn event_def() -> casper_dao_utils::definitions::EventDef {
            #event_def
        }
      }
    }
}

fn named_fields_with_types(
    input: DeriveInput,
) -> Result<Vec<(proc_macro2::Ident, syn::Type)>, TokenStream> {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => named_fields
            .named
            .into_iter()
            .map(|x| (x.ident.unwrap(), x.ty))
            .collect::<Vec<_>>(),
        _ => {
            return Err(
                quote! { compile_error!("Expected a struct with named fields and types."); },
            )
        }
    };
    Ok(fields)
}
