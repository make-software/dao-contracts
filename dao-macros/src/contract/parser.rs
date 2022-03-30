use std::fmt::Debug;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{braced, Token, TraitItemMethod};

use super::utils;

pub struct CasperContractItem {
    pub trait_token: Token![trait],
    pub trait_methods: Vec<TraitItemMethod>,
    pub ident: Ident,
    pub contract_ident: Ident,
    pub caller_ident: Ident,
    pub contract_test_ident: Ident,
    pub package_hash: String,
    pub wasm_file_name: String,
}

impl Debug for CasperContractItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trait_methods = &self.trait_methods;
        let trait_methods = quote! { #(#trait_methods)* }.to_string();
        f.debug_struct("CasperContractItem")
            .field("trait_token", &"trait")
            .field("trait_methods", &trait_methods)
            .field("ident", &self.ident)
            .field("contract_ident", &self.contract_ident)
            .field("caller_ident", &self.caller_ident)
            .field("contract_test_ident", &self.contract_test_ident)
            .field("package_hash", &self.package_hash)
            .field("wasm_file_name", &self.wasm_file_name)
            .finish()
    }
}

impl Parse for CasperContractItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        let _pub_token: Result<Token![pub], _> = input.parse();
        let trait_token: Token![trait] = input.parse()?;
        let ident: Ident = input.parse()?;
        let _brace_token = braced!(content in input);

        let mut methods = Vec::new();
        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        let name = ident.to_string();
        let parts: Vec<&str> = name.split("Interface").collect();
        let name = parts.first().unwrap();

        let contract_ident = format_ident!("{}", name);
        let caller_ident = format_ident!("{}Caller", name);
        let contract_test_ident = format_ident!("{}Test", name);

        let package_hash = format!("{}_package_hash", name.to_case(Case::Snake));
        let wasm_file_name = format!("{}.wasm", name.to_case(Case::Snake));

        let item = CasperContractItem {
            trait_token,
            trait_methods: methods,
            ident,
            contract_ident,
            caller_ident,
            contract_test_ident,
            package_hash,
            wasm_file_name,
        };
        validate_item(&item)?;

        Ok(item)
    }
}

fn validate_item(item: &CasperContractItem) -> Result<(), syn::Error> {
    if utils::find_method(item, "init").is_none() {
        return Err(syn::Error::new(
            Span::call_site(),
            "Contract must define init() method",
        ));
    }

    Ok(())
}
