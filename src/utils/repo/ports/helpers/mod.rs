use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, NoObjectForGivenKey,
};
use crate::utils::local::context::helpers::{
    build_node_typed_name, build_portation_id, parse_node_typed_name, parse_portation_id, NodeType,
    PortationDirectionType,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::jsonschema::{parse_root_json_schema, HoliumJsonSchema};
use crate::utils::local::models::shaper::Shaper;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::transformation::Transformation;
use crate::utils::repo::context::RepositoryContext;
use crate::utils::repo::models::portation::Portation;
use anyhow::{Context, Result};
use serde_json::Value;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to parse json schema")]
    JsonSchemaParseFailed,
}

/// Check from a repository context object if a specific portation object can be found.
pub fn get_portation(
    repo_context: &RepositoryContext,
    node_type: &NodeType,
    node_name: &str,
    direction: &PortationDirectionType,
) -> Option<Portation> {
    let id = build_portation_id(
        direction,
        build_node_typed_name(node_type, node_name).as_str(),
    );
    repo_context
        .portations
        .get(&id.to_string())
        .map(|portation| portation.clone())
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
