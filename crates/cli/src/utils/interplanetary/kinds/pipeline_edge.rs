use std::io::Cursor;
use cid::Cid;
use std::collections::HashMap;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use std::convert::{TryInto, TryFrom};
use sk_cbor::Value;
use sk_cbor::cbor_map;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::kinds::link::Link;
use sk_cbor::cbor_array;
use sk_cbor::cbor_unsigned;

/// [ PipelineEdge ] holds the data used as an attribute of edges of a pipeline graph.
#[derive(Clone)]
pub struct PipelineEdge {
    pub tail_index: u64,
    pub head_index: u64,
    pub connection_cid: Cid,
}

impl From<PipelineEdge> for sk_cbor::Value {
    fn from(o: PipelineEdge) -> Self {
        let connection_link: Value = Link(&o.connection_cid).into();
        cbor_array![
            cbor_unsigned!( o.tail_index ),
            cbor_unsigned!( o.head_index ),
            connection_link,
        ]
    }
}