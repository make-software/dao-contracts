use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{braced, token, Ident, Token, TraitItemMethod};

#[derive(Debug)]
pub struct ContractTrait {
    pub trait_token: Token![trait],
    pub ident: Ident,
    pub brace_token: token::Brace,
    pub methods: Vec<TraitItemMethod>,
}

impl Parse for ContractTrait {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let trait_token: Token![trait] = input.parse()?;
        let ident: Ident = input.parse()?;
        let brace_token = braced!(content in input);

        let mut methods = Vec::new();
        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(ContractTrait {
            trait_token,
            ident,
            brace_token,
            methods,
        })
    }
}

pub fn generate(model: &ContractTrait) -> TokenStream {
    let id = &model.ident;

    let mut methods = TokenStream::new();
    methods.append_all(&model.methods);

    quote! {
        trait #id {
            #methods
        }
    }
}
