use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(ident: &syn::Ident) -> TokenStream {
    let caller_name = format!("{}Caller", ident);
    let caller_ident = syn::Ident::new(&caller_name, ident.span());

    let expanded = quote! {

      struct #caller_ident {
        contract_package_hash: casper_types::ContractPackageHash,
      }
    };

    expanded
}
