use crate::utils::interplanetary::kinds::link::Link;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use cid::Cid;

use sk_cbor::cbor_text;
use sk_cbor::Value;

use std::convert::TryFrom;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate pipeline vertex kind")]
    FailedToManipulate,
}

/// [ PipelineVertex ] holds the Map used as an attribute of vertices of a pipeline graph.
#[derive(Default, Clone)]
pub struct PipelineVertex {
    pub dry_transformation: Option<Cid>,
    pub data: Option<Cid>,
    pub metadata: Option<Cid>,
}

impl From<PipelineVertex> for sk_cbor::Value {
    fn from(object: PipelineVertex) -> Self {
        let mut tuples: Vec<(Value, Value)> = Vec::new();
        if let Some(dry_transformation) = object.dry_transformation {
            tuples.push((cbor_text!("dt"), Link(dry_transformation).into()))
        }
        if let Some(data) = object.data {
            tuples.push((cbor_text!("rde"), Link(data).into()))
        }
        if let Some(metadata) = object.metadata {
            tuples.push((cbor_text!("meta"), Link(metadata).into()))
        }
        Value::Map(tuples)
    }
}

impl TryFrom<sk_cbor::Value> for PipelineVertex {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        let mut vertex = PipelineVertex::default();
        if let Value::Map(map) = value {
            for (key, value) in &map {
                if *key == cbor_text!("dt") {
                    let Link(cid) = Link::try_from(value.clone())?;
                    vertex.dry_transformation = Some(cid);
                }
                if *key == cbor_text!("rde") {
                    let Link(cid) = Link::try_from(value.clone())?;
                    vertex.data = Some(cid);
                }
                if *key == cbor_text!("meta") {
                    let Link(cid) = Link::try_from(value.clone())?;
                    vertex.metadata = Some(cid);
                }
            }
            return Ok(vertex);
        }
        Err(Error::FailedToManipulate.into())
    }
}
