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
use sk_cbor::cbor_text;

/// [ PipelineVertex ] holds the Map used as an attribute of vertices of a pipeline graph.
#[derive(Default, Clone)]
pub struct PipelineVertex {
    pub dry_transformation: Option<Cid>,
    pub metadata: Option<Cid>,
}

impl From<PipelineVertex> for sk_cbor::Value {
    fn from(o: PipelineVertex) -> Self {
        let mut tuples: Vec<(Value, Value)> = Vec::new();
        if let Some(dry_transformation) = o.dry_transformation {
            tuples.push((
                cbor_text!("dt"),
                Link(&dry_transformation).into(),
            ))
        }
        if let Some(metadata) = o.metadata {
            tuples.push((
                cbor_text!("meta"),
                Link(&metadata).into(),
            ))
        }
        Value::Map(tuples)
    }
}