use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{spanned::Spanned, DataStruct, DeriveInput, Ident};

pub fn generate_code(input: DeriveInput) -> TokenStream {
    match generate_or_err(input) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn generate_or_err(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let span = input.span();
    match input.data {
        syn::Data::Struct(data_struct) => parse_data(input.ident, data_struct),
        syn::Data::Enum(_) => Err(syn::Error::new(span, "Cannot instantiate an enum")),
        syn::Data::Union(_) => Err(syn::Error::new(span, "Cannot instantiate a union")),
    }
}

fn parse_data(struct_ident: Ident, data_struct: DataStruct) -> Result<TokenStream, syn::Error> {
    let mut fields = TokenStream::new();
    fields.append_all(data_struct.fields.into_iter().map(|field| {
        let ident = field.ident.unwrap();
        quote! {
            #ident: casper_dao_utils::instance::Instanced::instance(if namespace.is_empty() {
                format!("{}", stringify!(#ident))
            } else {
                format!("{}_{}", stringify!(#ident), namespace)
            }
            .as_str()),
        }
    }));

    Ok(quote! {
        impl casper_dao_utils::instance::Instanced for #struct_ident {

            fn instance(namespace: &str) -> Self {
                Self {
                    #fields
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{
        punctuated::Punctuated, token, DataStruct, Field, Fields, FieldsNamed, Token, Type,
        VisPublic, Visibility,
    };

    use super::parse_data;

    #[test]
    fn parsing_struct_data_works() {
        let mut fields: Punctuated<Field, Token![,]> = Punctuated::new();
        fields.push(create_field("b", syn::parse_quote! { B }));
        fields.push(create_field("c", syn::parse_quote! { C }));
        fields.push(create_field("d", syn::parse_quote! { D }));
        let input: DataStruct = build_struct(fields);

        let result = parse_data(quote::format_ident!("A"), input)
            .unwrap()
            .to_string();
        let expected = quote! {
            impl casper_dao_utils::instance::Instanced for A {
                fn instance(namespace: &str) -> Self {
                    Self {
                        b: casper_dao_utils::instance::Instanced::instance(if namespace.is_empty() {
                            format!("{}", stringify!(b))
                        } else {
                            format!("{}_{}", stringify!(b), namespace)
                        }
                        .as_str()),
                        c: casper_dao_utils::instance::Instanced::instance(if namespace.is_empty() {
                            format!("{}", stringify!(c))
                        } else {
                            format!("{}_{}", stringify!(c), namespace)
                        }
                        .as_str()),
                        d: casper_dao_utils::instance::Instanced::instance(if namespace.is_empty() {
                            format!("{}", stringify!(d))
                        } else {
                            format!("{}_{}", stringify!(d), namespace)
                        }
                        .as_str()),
                    }
                }
            }
        }
        .to_string();

        pretty_assertions::assert_eq!(expected, result);
    }

    fn create_field(name: &str, ty: Type) -> Field {
        Field {
            attrs: vec![],
            colon_token: Default::default(),
            ident: Some(quote::format_ident!("{}", name)),
            vis: Visibility::Public(VisPublic {
                pub_token: Default::default(),
            }),
            ty,
        }
    }

    fn build_struct(fields: Punctuated<Field, token::Comma>) -> DataStruct {
        DataStruct {
            struct_token: Default::default(),
            semi_token: Default::default(),
            fields: Fields::Named(FieldsNamed {
                brace_token: Default::default(),
                named: fields,
            }),
        }
    }
}
