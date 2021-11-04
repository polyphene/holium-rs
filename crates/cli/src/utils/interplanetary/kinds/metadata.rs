//! Manipulate interplanetary blocks, linked from pipelines' nodes, and holding metadata about
//! sources, shapers and transformations. 

use anyhow::Context;
use anyhow::Result;
use sk_cbor::cbor_text;
use sk_cbor::Value;
use sled::IVec;
use crate::utils::local::context::helpers::NodeType;
use crate::utils::local::models::shaper::Shaper;
use crate::utils::errors::Error::BinCodeDeserializeFailed;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::transformation::Transformation;

static DISCRIMINANT_KEY_V0: &str = "meta_0";

#[derive(Default)]
pub struct Metadata {
    pub name: Option<String>,
    pub json_schema: Option<String>,
    pub json_schema_in: Option<String>,
    pub json_schema_out: Option<String>,
}

impl Metadata {
    pub fn new(name: &str, encoded: &IVec, node_type: &NodeType) -> Result<Self> {
        let mut metadata = Metadata::default();
        metadata.name = Some(name.to_string());
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