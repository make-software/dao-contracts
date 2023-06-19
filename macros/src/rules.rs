use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataStruct, DeriveInput};

pub fn expand_derive_rule(input: DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Struct(data) => {
            let ident = input.ident;

            let fn_args = fn_args(&data);
            let struct_args = struct_args(&data);

            quote::quote! {
                impl #ident {
                    pub fn create(
                       #fn_args
                    ) -> Box<Self> {
                        Box::new(Self {
                            # ( #struct_args ),*
                        })
                    }
                }
            }
        }
        _ => quote! { compile_error!("Type is not supported."); },
    }
}

fn fn_args(data: &DataStruct) -> TokenStream {
    match &data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .flat_map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                quote!(#ident: #ty,)
            })
            .collect(),
        _ => quote! { compile_error!("Fields must be named."); },
    }
}

fn struct_args(data: &DataStruct) -> Vec<Ident> {
    match &data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .clone()
            .into_iter()
            .map(|f| f.ident.unwrap())
            .collect::<Vec<_>>(),
        _ => vec![],
    }
}
