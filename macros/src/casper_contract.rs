use marked_yaml::types::{MarkedMappingNode, MarkedScalarNode};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, Token};

#[derive(Debug)]
pub struct Contract {
    ident: Ident,
    methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Method {
    ident: Ident,
    contract_ident: Ident,
    args: Punctuated<Arg, Token![,]>,
}

#[derive(Debug)]
pub struct Arg {
    ident: Ident,
    ty: Vec<Ident>,
}

pub fn extend_casper_contract(schema_filename: &str) -> TokenStream {
    let contract = parser::parse_contract_schema(schema_filename);
    let base_methods = generate_base_methods(&contract);
    let interface_methods = generate_interface_methods(&contract);
    quote! {
        #base_methods

        #interface_methods
    }
}
fn generate_base_methods(contract: &Contract) -> TokenStream {
    let contract_ident = &contract.ident;

    quote! {
        #[no_mangle]
        fn call() {
            casper_dao_contracts::#contract_ident::install();
        }

        #[no_mangle]
        fn init() {
            casper_dao_contracts::#contract_ident::default().init();
        }
    }
}

fn generate_interface_methods(contract: &Contract) -> TokenStream {
    let contract_ident = &contract.ident;

    let mut methods = TokenStream::new();
    methods.append_all(contract.methods.iter().map(|method| {
        let ident = &method.ident;
        let args = &method.args;

        let mut casper_args = TokenStream::new();
        args.iter()
            .for_each(|arg| CasperArg::to_tokens(arg, &mut casper_args));

        quote! {
            #[no_mangle]
            fn #ident() {
                #casper_args
                let mut contract = casper_dao_contracts::#contract_ident::default();
                casper_dao_contracts::ReputationContractInterface::#ident(&mut contract, #args);
            }
        }
    }));
    methods
}

trait AsString {
    fn as_string(&self, idx: &str) -> String;
}

impl AsString for MarkedMappingNode {
    fn as_string(&self, idx: &str) -> String {
        self.get_scalar(idx)
            .map(MarkedScalarNode::as_str)
            .unwrap()
            .to_string()
    }
}

trait CasperArg {
    fn to_tokens(&self, tokens: &mut TokenStream);
}

impl CasperArg for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        tokens.append_all(quote! {
            let #ident = casper_contract::contract_api::runtime::get_named_arg(stringify!(#ident));
        });
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        tokens.append_all(quote! { #ident });
    }
}

mod parser {
    use std::{fs, path::PathBuf};

    use convert_case::Casing;
    use marked_yaml::{types::MarkedSequenceNode, Node};
    use proc_macro2::Ident;
    use project_root::get_project_root;
    use quote::format_ident;
    use syn::{punctuated::Punctuated, Token};

    use super::{Arg, AsString, Contract, Method};

    pub fn parse_contract_schema(schema_filename: &str) -> Contract {
        let mut methods: Vec<Method> = vec![];
        let yaml = read_yaml(schema_filename);

        let entry_points = read_entry_points(&yaml);
        let ident = format_ident!("{}", read_contract_name(&yaml));

        let comma = syn::token::Comma([proc_macro2::Span::call_site()]);

        for i in 0..entry_points.len() {
            let mut punctuated_args: Punctuated<Arg, Token![,]> = Punctuated::new();
            let method_node = entry_points.get_mapping(i).unwrap();
            let args_node = method_node.get_sequence("arguments").unwrap();

            for j in 0..args_node.len() {
                let args = args_node.get_mapping(j).unwrap();
                let ty: Vec<Ident> = args
                    .as_string("cl_type")
                    .split("::")
                    .map(|ty| format_ident!("{}", ty))
                    .collect();

                punctuated_args.push_value(Arg {
                    ident: format_ident!("{}", args.as_string("name")),
                    ty,
                });
                punctuated_args.push_punct(comma);
            }

            methods.push(Method {
                ident: format_ident!("{}", method_node.as_string("name")),
                contract_ident: ident.to_owned(),
                args: punctuated_args,
            })
        }
        Contract { ident, methods }
    }

    fn get_ymal_path(schema_filename: &str) -> PathBuf {
        let mut path: PathBuf = get_project_root().unwrap();
        path.push("contracts");
        path.push("resources");
        path.push(schema_filename);
        path
    }

    fn read_yaml(schema_filename: &str) -> Node {
        let filepath = get_ymal_path(schema_filename);

        let contents =
            fs::read_to_string(filepath).expect("Can't open the file - something went wrong");
        marked_yaml::parse_yaml(0, contents).unwrap()
    }

    fn read_entry_points(yaml: &Node) -> MarkedSequenceNode {
        let entry_points = yaml
            .as_mapping()
            .and_then(|n| n.get_sequence("entry_points"))
            .unwrap();

        entry_points.to_owned()
    }

    fn read_contract_name(yaml: &Node) -> String {
        let name = yaml.as_mapping().unwrap().as_string("name");

        name.to_case(convert_case::Case::Pascal)
    }
}
