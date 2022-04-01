use casper_dao_utils::{casper_dao_macros::Instance, Variable};

#[derive(Instance)]
pub struct MetadataERC721 {
    name: Variable<String>,
    symbol: Variable<String>,
}

impl MetadataERC721 {
    pub fn name(&self) -> String {
        self.name.get()
    }

    pub fn symbol(&self) -> String {
        self.symbol.get()
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol.set(symbol);
    }
}
