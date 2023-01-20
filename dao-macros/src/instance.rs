use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
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
    let fields = data_struct
        .fields
        .into_iter()
        .map(|field| {
            let ident = field.ident.unwrap();

            let scope = field
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident("scoped"))
                .map(|attr| match attr.parse_meta().unwrap() {
                    syn::Meta::NameValue(name_value) => match name_value.lit {
                        syn::Lit::Str(str) => {
                            str.value().parse::<Scope>().unwrap_or(Scope::Invalid)
                        }
                        _ => Scope::Invalid,
                    },
                    _ => Scope::Invalid,
                })
                .next()
                .unwrap_or(Scope::None);

            match scope {
                Scope::Contract => Ok(quote! {
                    #ident: casper_dao_utils::instance::Instanced::instance({
                        let idx = namespace.rfind("__");
                        let namespace = match idx {
                            Some(value) => &namespace[value+2..],
                            None => namespace
                        };
                        format!("{}__{}", stringify!(#ident), namespace).as_str()
                    }),
                }),
                Scope::Parent => Ok(quote! {
                    #ident: casper_dao_utils::instance::Instanced::instance({
                        let idx = namespace.find("__");
                        let namespace = match idx {
                            Some(value) => &namespace[value+2..],
                            None => namespace
                        };
                        format!("{}__{}", stringify!(#ident), namespace).as_str()
                    }),
                }),
                Scope::None => Ok(quote! {
                    #ident: casper_dao_utils::instance::Instanced::instance(
                        format!("{}__{}", stringify!(#ident), namespace).as_str()
                    ),
                }),
                Scope::Invalid => Err(syn::Error::new(
                    ident.span(),
                    "Invalid scope: available options are `contract` and `parent`",
                )),
            }
        })
        .try_collect::<TokenStream>()?;

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

#[derive(Debug, Clone, Copy)]
enum Scope {
    Contract,
    Parent,
    Invalid,
    None,
}

impl FromStr for Scope {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "contract" => Self::Contract,
            "parent" => Self::Parent,
            _ => Self::None,
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{
        parse::Parser,
        punctuated::Punctuated,
        token,
        DataStruct,
        Field,
        Fields,
        FieldsNamed,
        Token,
    };

    use super::parse_data;

    #[test]
    fn parsing_struct_data_works() {
        let mut fields: Punctuated<Field, Token![,]> = Punctuated::new();
        fields.push(Field::parse_named.parse2(quote!(b: B)).unwrap());
        fields.push(Field::parse_named.parse2(quote!(c: C)).unwrap());
        fields.push(
            Field::parse_named
                .parse2(quote! {
                    #[scoped = "parent"]
                    d : D
                })
                .unwrap(),
        );
        let input: DataStruct = build_struct(fields);

        let result = parse_data(quote::format_ident!("A"), input)
            .unwrap()
            .to_string();
        let expected = quote! {
            impl casper_dao_utils::instance::Instanced for A {
                fn instance(namespace: &str) -> Self {
                    Self {
                        b: casper_dao_utils::instance::Instanced::instance(
                            format!("{}__{}", stringify!(b), namespace).as_str()
                        ),
                        c: casper_dao_utils::instance::Instanced::instance(
                            format!("{}__{}", stringify!(c), namespace).as_str()
                        ),
                        d: casper_dao_utils::instance::Instanced::instance({
                            let idx = namespace.find("__");
                            let namespace = match idx {
                                Some(value) => &namespace[value+2..],
                                None => namespace
                            };
                            format!("{}__{}", stringify!(d), namespace).as_str()
                        }),
                    }
                }
            }
        }
        .to_string();

        pretty_assertions::assert_eq!(expected, result);
    }

    #[test]
    fn test_namespaces() {
        let module = "access_control";

        let parent_namespace = "token__contract";
        let ident = format!("{}__{}", module, parent_namespace);
        assert_eq!("access_control__token__contract", &ident);

        let namespace = "variable_repository__token__contract";
        let idx = namespace.find("__");
        let namespace = match idx {
            Some(value) => &namespace[value + 2..],
            None => namespace,
        };
        let ident = format!("{}__{}", module, namespace);
        assert_eq!("access_control__token__contract", &ident);

        let namespace = "module__variable_repository__token__contract";
        let idx = namespace.rfind("__");
        let namespace = match idx {
            Some(value) => &namespace[value + 2..],
            None => namespace,
        };
        let ident = format!("{}__{}", module, namespace);
        assert_eq!("access_control__contract", &ident);
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
