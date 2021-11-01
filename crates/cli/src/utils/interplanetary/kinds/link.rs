use cid::Cid;
use std::convert::TryInto;
use anyhow::Result;
use anyhow::Error as AnyhowError;
use sk_cbor::Value;
use sk_cbor::cbor_tagged;
use sk_cbor::cbor_bytes;

/// CBOR Tag ID registered to identify IPLD content identifiers.
/// Reference: https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml
static IPLD_CONTENT_CBOR_TAG_ID: u64 = 42;

pub struct Link<'a>(pub &'a Cid);

impl Into<sk_cbor::Value> for Link<'_> {
    fn into(self) -> Value {
        let cid_bytes = self.0.to_bytes();
        cbor_tagged!(
            IPLD_CONTENT_CBOR_TAG_ID,
            cbor_bytes!(cid_bytes)
        )
    }
}