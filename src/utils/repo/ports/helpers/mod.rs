use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, NoObjectForGivenKey,
};
use crate::utils::local::context::helpers::{
    parse_node_typed_name, parse_portation_id, NodeType, PortationDirectionType,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::jsonschema::{parse_root_json_schema, HoliumJsonSchema};
use crate::utils::local::models::shaper::Shaper;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::transformation::Transformation;

use crate::utils::repo::models::portation::Portation;
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to parse json schema")]
    JsonSchemaParseFailed,
}

/// Get the json schema related to a portation.
pub fn get_portation_json_schema(
    local_context: &LocalContext,
    portation: &Portation,
) -> Result<HoliumJsonSchema> {
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
    let json_schema_value: Value =
        serde_json::from_str(&json_schema_lit).context(Error::JsonSchemaParseFailed)?;
    parse_root_json_schema(&json_schema_value)
}
