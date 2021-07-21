//! Holds structures used for (de)serialization

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
/// Configuration structure used for data validation.
pub struct ConfigTemplate {
    pub(crate) core: Option<CoreTemplate>,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct CoreTemplate {
    pub(crate) no_scm: Option<bool>,
    pub(crate) no_dvc: Option<bool>,
}

impl ConfigTemplate {
    /// Parses a configuration file defined by its path.
    /// Returns the default configuration if the path does not exists.}
    pub fn from_config_file(path: &PathBuf) -> Result<ConfigTemplate> {
        // If the file does not exist, return an empty configuration
        if !path.exists() || !path.is_file() {
            return Ok(ConfigTemplate::default());
        }
        // read the file..
        let mut f = File::open(path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        // ...and parse it
        Ok(toml::from_str(buffer.as_str())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn can_parse_config_file() {
        // Prepare a valid configuration
        let conf_str = r#"
            [core]
                no_scm = true
        "#;
        // Create the configuration file
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let conf_path = temp_dir.child("config");
        conf_path.write_str(conf_str).unwrap();
        // Try to parse it
        let config = ConfigTemplate::from_config_file(&conf_path.to_path_buf()).unwrap();

        assert_eq!(config.core.unwrap().no_scm.unwrap(), true);
    }

    #[test]
    fn can_parse_non_existent_config_file() {
        // Try to parse non existent configuration file
        let path = PathBuf::new();
        let config = ConfigTemplate::from_config_file(&path).unwrap();

        assert!(config.core.is_none());
    }
}
