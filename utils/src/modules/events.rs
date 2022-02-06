use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::{Bytes, ToBytes};

use crate::{consts, list::List};

pub struct Events {
    pub events: List<Bytes>,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            events: List::new(consts::NAME_EVENTS),
        }
    }
}

impl Events {
    pub fn init(&mut self) {
        self.events.init();
    }

    pub fn emit<T: ToBytes>(&mut self, event: T) {
        let bytes: Bytes = event.to_bytes().unwrap_or_revert().into();
        self.events.add(bytes);
    }
}
