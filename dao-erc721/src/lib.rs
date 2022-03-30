mod erc721;
pub use erc721::*;

#[cfg(feature = "test-support")]
pub use erc721::ERC721Test;
