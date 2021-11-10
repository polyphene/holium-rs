//! Helper methods for the local context.

use anyhow::{Context, Error as AnyhowError, Result};
use clap::{arg_enum, value_t};
use std::io::{Read, Seek};
use thiserror;

use crate::utils::cbor::as_holium_cbor::AsHoliumCbor;
use crate::utils::errors::Error::{DbOperationFailed, NoDataForObject};
use crate::utils::local::context::constants::{
    CONNECTION_ID_SEPARATOR, PORTATION_FROM_HOLIUM_PREFIX, PORTATION_PREFIX_SEPARATOR,
    PORTATION_TO_HOLIUM_PREFIX, TYPED_NODE_NAME_SEPARATOR,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::models::data::HoliumCbor;
use sled::Serialize;
use std::str::from_utf8;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("a node name cannot contain the '{1}' character: {0}")]
    InvalidNodeName(String, String),
    #[error("invalid connection id: {0}")]
    InvalidConnectionId(String),
    #[error("invalid pipeline node type: {0}")]
    InvalidNodeType(String),
    #[error("no {0} node found with name: {1}")]
    NoPipelineNodeWithName(String, String),
    #[error("invalid typed name: {0}")]
    InvalidTypedName(String),
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
        return Err(
            Error::InvalidNodeName(name.to_string(), CONNECTION_ID_SEPARATOR.to_string()).into(),
        );
    }
    Ok(())
}

/// Build a node typed name (*eg* `transformation:my-transformation`) from its type (*eg* `transformation`)
/// and name (*eg* `my-transformation`).
pub fn build_node_typed_name(node_type: &NodeType, node_name: &str) -> String {
    format!(
        "{}{}{}",
        node_type.to_string(),
        TYPED_NODE_NAME_SEPARATOR,
        node_name
    )
}

/// Parse a type name for a node and returns its type along with its name
pub fn parse_node_typed_name(typed_name: &str) -> Result<(NodeType, String)> {
    use std::str::FromStr;

    let splits: Vec<&str> = typed_name.split(TYPED_NODE_NAME_SEPARATOR).collect();
    if splits.len() != 2 {
        return Err(Error::InvalidTypedName(typed_name.to_string()).into());
    }
    Ok((
        NodeType::from_str(splits[0])
            .or(Err(Error::InvalidNodeType(splits[0].to_string().into())))?,
        splits[1].to_string(),
    ))
}

/// Check from a node type string and a node name that a pipeline node does exist in the local Holium area.
/// In case it does exists, return its typed name.
pub fn validate_pipeline_node_existence(
    local_context: &LocalContext,
    node_type: &NodeType,
    node_name: &str,
) -> Result<String> {
    let tree = match node_type {
        NodeType::shaper => &local_context.shapers,
        NodeType::source => &local_context.sources,
        NodeType::transformation => &local_context.transformations,
    };
    if !tree.contains_key(node_name).context(DbOperationFailed)? {
        return Err(
            Error::NoPipelineNodeWithName(node_type.to_string(), node_name.to_string()).into(),
        );
    }
    let typed_name = build_node_typed_name(node_type, node_name);
    Ok(typed_name)
}

/// Build a connection id (*eg* `source:my-source→transformation:my-transformation`) from the typed
/// names of its tail and head nodes ((*eg* `source:my-source` and `transformation:my-transformation`).
pub fn build_connection_id(tail_typed_name: &str, head_typed_name: &str) -> String {
    format!(
        "{}{}{}",
        tail_typed_name, CONNECTION_ID_SEPARATOR, head_typed_name
    )
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
    format!(
        "{}{}{}",
        direction_prefix, PORTATION_PREFIX_SEPARATOR, node_typed_name
    )
}

/// Helper method parsing a vectorized key name from the DB into its string version.
pub fn db_key_to_str(k: sled::IVec) -> Result<String> {
    let name = from_utf8(k.as_ref())?;
    Ok(name.to_string())
}

/// Helper to get data for a node from a local context
/// TODO when portation implemented, add a first step to use the portation first
pub fn node_data(local_context: &LocalContext, node_typed_name: &str) -> Result<HoliumCbor> {
    Ok(local_context
        .data
        .get(node_typed_name)
        .context(DbOperationFailed)?
        .ok_or(NoDataForObject(node_typed_name.to_string().into()))?
        .as_ref()
        .to_vec())
}

#[cfg(test)]
mod test {
    use super::*;

    /*************************************
     * Validate node name
     *************************************/

    #[test]
    fn cannot_validate_node_name_with_arrow_character() {
        let name = "my→name";

        let res = validate_node_name(name);

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("a node name cannot contain the '→' character"))
    }

    #[test]
    fn can_validate_node_name() {
        let name = "node_name";

        validate_node_name(name).unwrap();
    }

    /*************************************
     * Build node typed name
     *************************************/
    #[test]
    fn can_build_node_typed_name() {
        let name = "node_name";
        let expected_node_typed_name = format!("source:{}", name);

        let res = build_node_typed_name(&NodeType::source, name);
        assert_eq!(expected_node_typed_name, res);
    }
}
