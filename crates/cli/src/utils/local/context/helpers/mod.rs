//! Helper methods for the local context.

use anyhow::{Context, Result, Error as AnyhowError};
use clap::{arg_enum, value_t};
use thiserror;

use crate::utils::errors::Error::DbOperationFailed;
use crate::utils::local::context::LocalContext;

#[derive(Debug, thiserror::Error)]
enum Error {
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

/// Check from a node type string and a node name that a pipeline node does exist in the local Holium area.
pub fn validate_pipeline_node_existence(local_context: &LocalContext, node_type: &str, node_name: &str) -> Result<String> {
    let node_type = node_type.parse::<NodeType>().map_err(AnyhowError::msg)?;
    let tree = match node_type {
        NodeType::shaper => &local_context.shapers,
        NodeType::source => &local_context.sources,
        NodeType::transformation => &local_context.transformations,
    };
    if !tree.contains_key(node_name).context(DbOperationFailed)? {
        return Err(Error::NoPipelineNodeWithName(node_type.to_string(), node_name.to_string()).into());
    }
    let node_name = format!("{}:{}", node_type.to_string(), node_name);
    Ok(node_name)
}