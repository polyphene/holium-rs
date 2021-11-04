//! Helper methods for the local context.

use anyhow::{Context, Result, Error as AnyhowError};
use clap::{arg_enum, value_t};
use thiserror;

use crate::utils::errors::Error::DbOperationFailed;
use crate::utils::local::context::LocalContext;
use crate::utils::local::context::constants::{TYPED_NODE_NAME_SEPARATOR, CONNECTION_ID_SEPARATOR, PORTATION_PREFIX_SEPARATOR, PORTATION_FROM_HOLIUM_PREFIX, PORTATION_TO_HOLIUM_PREFIX};
use std::str::{from_utf8, FromStr};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("a node name cannot contain the '{1}' character: {0}")]
    InvalidNodeName(String, String),
    #[error("invalid connection id: {0}")]
    InvalidConnectionId(String),
    #[error("invalid node typed name: {0}")]
    InvalidNodeTypedName(String),
    #[error("invalid pipeline node type: {0}")]
    InvalidNodeType(String),
    #[error("no {0} node found with name: {1}")]
    NoPipelineNodeWithName(String, String),
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    /// NodeType enumerates the different types of pipeline nodes handled in the local Holium area.
    pub enum NodeType {
        shaper,
        source,
        transformation,
    }
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    /// Variants of the CLI argument coding for the direction of a new portation.
    pub enum PortationDirectionType {
        fromHolium,
        toHolium,
    }
}


/// Validate the name (used as storage key) of a DAG node.
pub fn validate_node_name(name: &str) -> Result<()> {
    /// Check that the string does not contain the [CONNECTION_NAME_SEPARATOR] character.
    if name.to_string().contains(CONNECTION_ID_SEPARATOR) {
        return Err(Error::InvalidNodeName(
            name.to_string(),
            CONNECTION_ID_SEPARATOR.to_string()
        ).into())
    }
    Ok(())
}

/// Build a node typed name (*eg* `transformation:my-transformation`) from its type (*eg* `transformation`)
/// and name (*eg* `my-transformation`).
pub fn build_node_typed_name(node_type: &NodeType, node_name: &str) -> String {
    format!("{}{}{}", node_type.to_string(), TYPED_NODE_NAME_SEPARATOR, node_name)
}

/// Parse a node typed name (*eg* `transformation:my-transformation`) into its type (*eg* `transformation`)
/// and untyped name (*eg* `my-transformation`).
pub fn parse_node_typed_name(node_typed_name: &str) -> Result<(NodeType, String)> {
    let splits: Vec<&str> = node_typed_name.split(TYPED_NODE_NAME_SEPARATOR).collect();
    if splits.len() != 2 {
        return Err(Error::InvalidNodeTypedName(node_typed_name.to_string()).into());
    }
    let node_type = NodeType::from_str(splits[0]).map_err(AnyhowError::msg)?;
    Ok((node_type, splits[1].to_string()))
}

/// Check from a node type string and a node name that a pipeline node does exist in the local Holium area.
/// In case it does exists, return its typed name.
pub fn validate_pipeline_node_existence(local_context: &LocalContext, node_type: &NodeType, node_name: &str) -> Result<String> {
    let tree = match node_type {
        NodeType::shaper => &local_context.shapers,
        NodeType::source => &local_context.sources,
        NodeType::transformation => &local_context.transformations,
    };
    if !tree.contains_key(node_name).context(DbOperationFailed)? {
        return Err(Error::NoPipelineNodeWithName(node_type.to_string(), node_name.to_string()).into());
    }
    let typed_name = build_node_typed_name(node_type, node_name);
    Ok(typed_name)
}

/// Build a connection id (*eg* `source:my-source→transformation:my-transformation`) from the typed
/// names of its tail and head nodes ((*eg* `source:my-source` and `transformation:my-transformation`).
pub fn build_connection_id(tail_typed_name: &str, head_typed_name: &str) -> String {
    format!("{}{}{}", tail_typed_name, CONNECTION_ID_SEPARATOR, head_typed_name)
}

/// Parse a connection id (*eg* `source:my-source→transformation:my-transformation`) and return a tuple holding the typed
/// names of its tail and head nodes ((*eg* `source:my-source` and `transformation:my-transformation`).
pub fn parse_connection_id(connection_id: &str) -> Result<(&str, &str)> {
    let splits: Vec<&str> = connection_id.split(CONNECTION_ID_SEPARATOR).collect();
    if splits.len() != 2 {
        return Err(Error::InvalidConnectionId(connection_id.to_string()).into());
    }
    Ok((splits[0], splits[1]))
}

/// Build a portation id (*eg* `from:transformation:my-transformation`) from the direction of the
/// portation (from Holium or to Holium) and the typed name of the node (*eg* `transformation:my-transformation`).
pub fn build_portation_id(direction: &PortationDirectionType, node_typed_name: &str) -> String {
    let direction_prefix = match direction {
        PortationDirectionType::fromHolium => PORTATION_FROM_HOLIUM_PREFIX,
        PortationDirectionType::toHolium => PORTATION_TO_HOLIUM_PREFIX,
    };
    format!("{}{}{}", direction_prefix, PORTATION_PREFIX_SEPARATOR, node_typed_name)
}

/// Helper method parsing a vectorized key name from the DB into its string version.
pub fn db_key_to_str(k: sled::IVec) -> Result<String> {
    let name = from_utf8(k.as_ref())?;
    Ok(name.to_string())
}