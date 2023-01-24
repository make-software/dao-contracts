use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::{Bytes, ToBytes};

use crate::{consts, Error, List, OrderedCollection};

pub struct Events {
    pub events: OrderedCollection<Bytes>,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            events: OrderedCollection::new(consts::NAME_EVENTS),
        }
    }
}

impl Events {
    pub fn emit<T: ToBytes>(&mut self, event: T) {
        let bytes: Bytes = event
            .to_bytes()
            .unwrap_or_revert_with(Error::BytesConversionError)
            .into();
        self.events.add(bytes);
    }
}
