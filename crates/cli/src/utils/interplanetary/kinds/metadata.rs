//! Manipulate interplanetary blocks, linked from pipelines' nodes, and holding metadata about
//! sources, shapers and transformations. 

use anyhow::Context;
use anyhow::Result;
use anyhow::Error as AnyhowError;
use sk_cbor::cbor_text;
use sk_cbor::Value;
use sled::IVec;
use crate::utils::local::context::helpers::{NodeType, build_node_typed_name};
use crate::utils::local::models::shaper::Shaper;
use crate::utils::errors::Error::BinCodeDeserializeFailed;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::transformation::Transformation;
use std::convert::TryFrom;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to manipulate metadata kind")]
    FailedToManipulate,
}

static DISCRIMINANT_KEY_V0: &str = "meta_0";

#[derive(Default, Clone)]
pub struct Metadata {
    pub name: Option<String>,
    pub json_schema: Option<String>,
    pub json_schema_in: Option<String>,
    pub json_schema_out: Option<String>,
}

impl Metadata {
    pub fn new(name: &str, encoded: &IVec, node_type: &NodeType) -> Result<Self> {
        let mut metadata = Metadata::default();
        metadata.name = Some(build_node_typed_name(node_type, name));
        let a = match node_type {
            NodeType::shaper => {
                let decoded: Shaper = bincode::deserialize(&encoded[..])
                    .ok()
                    .context(BinCodeDeserializeFailed)?;
                metadata.json_schema = Some(decoded.json_schema);
            }
            NodeType::source => {
                let decoded: Source = bincode::deserialize(&encoded[..])
                    .ok()
                    .context(BinCodeDeserializeFailed)?;
                metadata.json_schema = Some(decoded.json_schema);
            }
            NodeType::transformation => {
                let decoded: Transformation = bincode::deserialize(&encoded[..])
                    .ok()
                    .context(BinCodeDeserializeFailed)?;
                metadata.json_schema_in = Some(decoded.json_schema_in);
                metadata.json_schema_out = Some(decoded.json_schema_out);
            }
        };
        Ok(metadata)
    }
}

impl From<Metadata> for sk_cbor::Value {
    fn from(o: Metadata) -> Self {
        let mut values = vec![
            (cbor_text!("typedVersion"), cbor_text!(DISCRIMINANT_KEY_V0))
        ];
        if let Some(name) = o.name {
            values.push((cbor_text!("name"), cbor_text!(name)))
        }
        if let Some(json_schema) = o.json_schema {
            values.push((cbor_text!("json_schema"), cbor_text!(json_schema)))
        }
        if let Some(json_schema_in) = o.json_schema_in {
            values.push((cbor_text!("json_schema_in"), cbor_text!(json_schema_in)))
        }
        if let Some(json_schema_out) = o.json_schema_out {
            values.push((cbor_text!("json_schema_out"), cbor_text!(json_schema_out)))
        }
        Value::Map(values)
    }
}

impl TryFrom<sk_cbor::Value> for Metadata {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        let mut metadata = Metadata::default();
        if let Value::Map(map) = value {
            for (key, value) in &map {
                if *key == cbor_text!("name") {
                    if let Value::TextString(name) = value {
                        metadata.name = Some(name.clone());
                    }
                }
                if *key == cbor_text!("json_schema") {
                    if let Value::TextString(json_schema) = value {
                        metadata.json_schema = Some(json_schema.clone());
                    }
                }
                if *key == cbor_text!("json_schema_in") {
                    if let Value::TextString(json_schema_in) = value {
                        metadata.json_schema_in = Some(json_schema_in.clone());
                    }
                }
                if *key == cbor_text!("json_schema_out") {
                    if let Value::TextString(json_schema_out) = value {
                        metadata.json_schema_out = Some(json_schema_out.clone());
                    }
                }
            }
            return Ok(metadata)
        }
        Err(Error::FailedToManipulate.into())
    }
}