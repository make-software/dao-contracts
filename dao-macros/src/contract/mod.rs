mod caller;
mod contract_bin;
mod contract_struct;
mod contract_test;
mod generator;
mod parser;
mod utils;
pub use generator::generate_code;
pub use parser::CasperContractItem;
