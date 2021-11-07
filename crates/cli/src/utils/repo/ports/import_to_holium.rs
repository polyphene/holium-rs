use anyhow::{Result, Context};
use crate::utils::repo::models::portation::{Portation, PortationFileFormat};
use std::io::{Write, BufReader};
use crate::utils::repo::context::RepositoryContext;
use std::path::Path;
use serde_json::Value;
use crate::utils::repo::helpers::get_root_path;
use crate::utils::local::context::LocalContext;
use crate::utils::local::context::helpers::{parse_portation_id, PortationDirectionType, parse_node_typed_name, NodeType};
use crate::utils::errors::Error::{DbOperationFailed, NoObjectForGivenKey, BinCodeDeserializeFailed};
use crate::utils::local::models::shaper::Shaper;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::transformation::Transformation;
use crate::utils::local::helpers::jsonschema::{parse_root_json_schema, HoliumJsonSchema};
use crate::utils::repo::ports::formats::json::JsonPorter;
use crate::utils::repo::ports::formats::FormatPorter;
use crate::utils::repo::ports::formats::cbor::CborPorter;
use crate::utils::repo::ports::formats::bin::BinPorter;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to open file {0} for portation {1}")]
    FailedToOpenFile(String, String),
    #[error("portation in wrong direction")]
    WrongPortationDirection,
    #[error("failed to parse json schema")]
    JsonSchemaParseFailed,
}

/// Read data from a Portation and write it as HoliumCBOR data.
pub fn import_to_holium<W: Write>(local_context: &LocalContext, portation: &Portation, writer: &mut W) -> Result<()> {
    // get json schema from the portation
    let json_schema = get_portation_json_schema(&local_context, &portation)?;
    // open file from its path
    let path = get_root_path()?.join(&portation.file_path);
    let file = std::fs::File::open(&path)
        .context(Error::FailedToOpenFile(
            path
                .file_name()
                .map(|oss| oss.to_string_lossy().to_string())
                .unwrap_or("".to_string()),
            portation.id.clone())
        )?;
    let mut reader = BufReader::new(file);
    // parse data in Holium CBOR format
    match portation.file_format {
        PortationFileFormat::bin => BinPorter::import_to_holium(&json_schema, &mut reader, writer),
        PortationFileFormat::cbor => CborPorter::import_to_holium(&json_schema, &mut reader, writer),
        PortationFileFormat::csv => todo!(),
        PortationFileFormat::json => JsonPorter::import_to_holium(&json_schema, &mut reader, writer),
    }?;
    Ok(())
}

/// Get the json schema related to a portation.
pub fn get_portation_json_schema(local_context: &LocalContext, portation: &Portation) -> Result<HoliumJsonSchema> {
    // get details of the portation
    let (direction, node_typed_name) = parse_portation_id(&portation.id)?;
    let (node_type, node_name) = parse_node_typed_name(node_typed_name)?;
    let tree = local_context.get_tree_from_node_type(&node_type);
    let encoded = tree
        .get(&node_name)
        .context(DbOperationFailed)?
        .ok_or(NoObjectForGivenKey(node_name.to_string()))?;
    let json_schema_lit = match node_type {
        NodeType::shaper => {
            let decoded: Shaper = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.json_schema
        }
        NodeType::source => {
            let decoded: Source = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.json_schema
        }
        NodeType::transformation => {
            let decoded: Transformation = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            if direction == PortationDirectionType::toHolium {
                decoded.json_schema_in
            } else {
                decoded.json_schema_out
            }
        }
    };
    // parse json schema
    let json_schema_value: Value = serde_json::from_str(&json_schema_lit)
        .context(Error::JsonSchemaParseFailed)?;
    parse_root_json_schema(&json_schema_value)
}