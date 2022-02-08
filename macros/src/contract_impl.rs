use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        pub fn install() {

        }

        pub fn entry_points() -> casper_types::EntryPoints {
            let mut entry_points = casper_types::EntryPoints::new();
            entry_points.add_entry_point(casper_types::EntryPoint::new(
                "init",
                vec![],
                <() as casper_types::CLTyped>::cl_type(),
                casper_types::EntryPointAccess::Groups(vec![casper_types::Group::new("init")]),
                casper_types::EntryPointType::Contract,
            ));
            entry_points
        }
    }
}
