//! Holds structures used for (de)serialization

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Default)]
/// Configuration structure used for data validation.
pub(crate) struct ConfigTemplate {
    pub(crate) core: CoreTemplate,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct CoreTemplate {
    pub(crate) no_scm: Option<bool>,
    pub(crate) no_dvc: Option<bool>,
}

impl ConfigTemplate {
    /// The standard configuration holds default non-empty values for all configuration fields.
    ///
    /// Two different structures could be used here instead of one, implementing the `Into` trait.
    pub fn standard() -> Self {
        ConfigTemplate {
            core: CoreTemplate {
                no_scm: Some(false),
                no_dvc: Some(false),
            },
        }
    }
}

/// Provides the `ShadowMerge` trait for objects that can be merged in shadowing order.
/// A derive macro could easily generate implementation code for structures.
pub trait ShadowMerge {
    fn shadow_merge(&mut self, other: Self);
}

impl ShadowMerge for ConfigTemplate {
    fn shadow_merge(&mut self, other: Self) {
        self.core.shadow_merge(other.core);
    }
}

impl ShadowMerge for CoreTemplate {
    fn shadow_merge(&mut self, other: Self) {
        self.no_scm.shadow_merge(other.no_scm);
        self.no_dvc.shadow_merge(other.no_dvc);
    }
}

impl<T> ShadowMerge for Option<T> {
    fn shadow_merge(&mut self, other: Self) {
        if other.is_some() {
            *self = other;
        }
    }
}

impl ConfigTemplate {
    /// Parses a configuration file defined by its path.
    /// Returns the default sparse configuration if the path does not exists.
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

        assert_eq!(config.core.no_scm.unwrap(), true);
    }

    #[test]
    fn can_parse_non_existent_config_file() {
        // Try to parse non existent configuration file
        let path = PathBuf::new();
        let config = ConfigTemplate::from_config_file(&path).unwrap();
        // Default sparse configuration should thus be returned
        assert!(config.core.no_scm.is_none());
    }

    /// Utility function used to scaffold tests on the `core.no_scp` configuration field.
    fn test_merge_config(shadowed: Option<bool>, shadowing: Option<bool>, expected: Option<bool>) {
        // Prepare original configurations
        let mut config = ConfigTemplate::default();
        config.core.no_scm = shadowed;
        let mut shadowing_config = ConfigTemplate::default();
        shadowing_config.core.no_scm = shadowing;
        // Merge configurations
        config.shadow_merge(shadowing_config);
        // Test
        assert_eq!(config.core.no_scm, expected);
    }

    #[test]
    fn can_merge_configs_none_none() {
        test_merge_config(None, None, None)
    }

    #[test]
    fn can_merge_configs_none_true() {
        test_merge_config(None, Some(true), Some(true))
    }

    #[test]
    fn can_merge_configs_true_none() {
        test_merge_config(Some(true), None, Some(true))
    }

    #[test]
    fn can_merge_configs_true_false() {
        test_merge_config(Some(true), Some(false), Some(false))
    }

    #[test]
    fn can_merge_configs_false_true() {
        test_merge_config(Some(false), Some(true), Some(true))
    }
}
