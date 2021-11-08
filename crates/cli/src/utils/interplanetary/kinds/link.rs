use cid::Cid;
use std::convert::{TryInto, TryFrom};
use anyhow::{Result, Context};
use anyhow::Error as AnyhowError;
use sk_cbor::Value;
use sk_cbor::cbor_tagged;
use sk_cbor::cbor_bytes;
use std::io::Cursor;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate link kind")]
    FailedToManipulate,
}

/// CBOR Tag ID registered to identify IPLD content identifiers.
/// Reference: https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml
static IPLD_CONTENT_CBOR_TAG_ID: u64 = 42;

pub struct Link(pub Cid);

impl Into<sk_cbor::Value> for Link {
    fn into(self) -> Value {
        let cid_bytes = self.0.to_bytes();
        cbor_tagged!(
            IPLD_CONTENT_CBOR_TAG_ID,
            cbor_bytes!(cid_bytes)
        )
    }
}

impl TryFrom<sk_cbor::Value> for Link {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let Value::Tag(_, boxed_bytes) = value {
            if let Value::ByteString(cid_bytes) = *boxed_bytes {
                let cid = Cid::read_bytes(Cursor::new(cid_bytes)).context(Error::FailedToManipulate)?;
                return Ok(Link(cid))
            }
        }
        Err(Error::FailedToManipulate.into())
    }
}