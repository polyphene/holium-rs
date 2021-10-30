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
        return Err(Error::InvalidNodeName(name.to_string()).into());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

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
}
