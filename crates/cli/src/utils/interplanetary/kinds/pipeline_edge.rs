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

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate pipeline edge kind")]
    FailedToManipulate,
}

/// [ PipelineEdge ] holds the data used as an attribute of edges of a pipeline graph.
#[derive(Clone, Default)]
pub struct PipelineEdge {
    pub tail_index: u64,
    pub head_index: u64,
    pub connection_cid: Cid,
}

impl From<PipelineEdge> for sk_cbor::Value {
    fn from(o: PipelineEdge) -> Self {
        let connection_link: Value = Link(o.connection_cid).into();
        cbor_array![
            cbor_unsigned!( o.tail_index ),
            cbor_unsigned!( o.head_index ),
            connection_link,
        ]
    }
}

impl TryFrom<sk_cbor::Value> for PipelineEdge {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        let mut edge = PipelineEdge::default();
        if let Value::Array(tuple) = value {
            if tuple.get(0..3).is_some() {
                if let Value::Unsigned(tail_index) = tuple[0] {
                    edge.tail_index = tail_index;
                }
                if let Value::Unsigned(head_index) = tuple[1] {
                    edge.head_index = head_index;
                }
                let Link(connection_cid) = Link::try_from(tuple[2].clone())?;
                edge.connection_cid = connection_cid;
                return Ok(edge)
            }
        }
        Err(Error::FailedToManipulate.into())
    }
}