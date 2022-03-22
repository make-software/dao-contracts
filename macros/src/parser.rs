use convert_case::{Case, Casing};
use proc_macro2::Ident;
use quote::format_ident;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Token, TraitItemMethod};

#[derive(Debug)]
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

        Ok(CasperContractItem {
            trait_token,
            trait_methods: methods,
            ident,
            contract_ident,
            caller_ident,
            contract_test_ident,
            package_hash,
            wasm_file_name,
        })
    }
}
