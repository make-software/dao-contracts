use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::{Bytes, ToBytes};

use crate::{Mapping, Variable};

pub struct Events {
    pub events: Mapping<u32, Bytes>,
    pub length: Variable<u32>,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            events: Mapping::new(String::from("events")),
            length: Variable::new(String::from("length")),
        }
    }
}

impl Events {
    pub fn init(&mut self) {
        self.events.init();
        self.length.set(0);
    }

    pub fn emit<T: ToBytes>(&mut self, event: T) {
        let lenght = self.length.get();
        let bytes: Bytes = event.to_bytes().unwrap_or_revert().into();
        self.events.set(&lenght, bytes);
        self.length.set(lenght + 1);
    }
}
