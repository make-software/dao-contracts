#[cfg(not(feature = "casper-livenet"))]
compile_error!("This crate can only be used for CasperLabs Livenet");

fn main() {
    dao_client::cli::parse();
}
