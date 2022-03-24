pub mod tests {
    use quote::format_ident;
    use syn::parse_quote;

    use crate::contract::parser::CasperContractItem;

    pub fn mock_valid_item() -> CasperContractItem {
        CasperContractItem {
            trait_token: Default::default(),
            ident: format_ident!("{}", "ContractTrait"),
            trait_methods: vec![
                parse_quote! { fn init(&mut self); },
                parse_quote! { fn do_something(&mut self, amount: U256); },
            ],
            caller_ident: format_ident!("{}", "ContractCaller"),
            contract_ident: format_ident!("{}", "Contract"),
            contract_test_ident: format_ident!("{}", "ContractTest"),
            package_hash: "contract".to_string(),
            wasm_file_name: "contract_wasm".to_string(),
        }
    }

    pub fn mock_no_init_item() -> CasperContractItem {
        CasperContractItem {
            trait_methods: vec![
                parse_quote! { fn contrustor(&mut self); },
                parse_quote! { fn do_something(&mut self, amount: U256); },
            ],
            ..mock_valid_item()
        }
    }
}
