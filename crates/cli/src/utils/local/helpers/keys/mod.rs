//! Various helper methods related to keys of the local Holium area store.

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("a node name cannot contain the '→' character: {0}")]
    InvalidNodeName(String),
}

/// Validate the name (used as storage key) of a DAG node.
pub fn validate_node_name(name: &str) -> Result<()> {
    /// Check that the string does not contain the '→' character.
    if name.to_string().contains("→") {
        return Err(Error::InvalidNodeName(name.to_string()).into())
    }
    Ok(())
}