use crate::utils::cbor::as_holium_cbor::AsHoliumCbor;
use crate::utils::cbor::write_holium_cbor::WriteHoliumCbor;
use crate::utils::interplanetary::kinds::selector::SelectorEnvelope;
use std::io::Cursor;

pub const TREE_NAME: &[u8] = b"data";

pub type HoliumCbor = Vec<u8>;

impl AsHoliumCbor for HoliumCbor {
    fn as_cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self)
    }
}

impl WriteHoliumCbor for HoliumCbor {
    fn as_cursor(&self) -> Cursor<&[u8]> {
        Cursor::new(&self)
    }
    fn from_bytes(cbor_bytes: &[u8]) -> Self {
        HoliumCbor::from(cbor_bytes)
    }
}
