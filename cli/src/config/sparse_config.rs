//! Holds structures used for TOML (de)serialization of project configurations.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};

use crate::config::models::ConfigError;
use crate::config::updatable_field::UpdatableField;

#[derive(Serialize, Deserialize, Clone, Default)]
/// Sparse version of the Config structure.
/// A macro should probably do this job.
pub(crate) struct SparseConfig {
    pub(crate) core: Option<SparseConfigCore>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(crate) struct SparseConfigCore {
    pub(crate) no_scm: Option<bool>,
    pub(crate) no_dvc: Option<bool>,
}

impl SparseConfig {
    /// Parses a configuration file defined by its path.
    pub(crate) fn from_config_file(path: &PathBuf) -> Result<SparseConfig> {
        // If the file does not exist, return an empty configuration
        if !path.exists() || !path.is_file() {
            return Ok(SparseConfig::default());
        }
        // read the file..
        let data = fs::read_to_string(path)?;
        // ...and parse it
        let parsed_data = toml::from_str(data.as_str()).context(
            ConfigError::ConfigFileDeserialization(path.display().to_string()),
        )?;
        Ok(parsed_data)
    }

    /// Saves a SparseConfig on the file system.
    pub(crate) fn save_to_config_file(&self, path: &PathBuf) -> Result<()> {
        // Serialize the configuration
        let data = toml::to_string_pretty(self).context(ConfigError::ConfigFileSerialization)?;
        fs::write(path, data).context(ConfigError::ConfigFileWrite(path.display().to_string()))?;
        Ok(())
    }

    /// Gets, sets or unsets a field of a sparse configuration.
    ///
    /// # Errors
    ///
    /// In case an unknown key is submitted, an error is ret
    fn edit(
        &mut self,
        chained_prop_name: &str,
        opt_new_value: Option<toml::Value>,
        unset: bool,
    ) -> Result<Option<String>> {
        Ok(match chained_prop_name {
            "core.no_scm" => self
                .core
                .get_or_insert_with(Default::default)
                .no_scm
                .update(opt_new_value, unset),
            "core.no_dvc" => self
                .core
                .get_or_insert_with(Default::default)
                .no_dvc
                .update(opt_new_value, unset),
            _ => {
                return Err(
                    ConfigError::ConfigTemplateUnknownKey(chained_prop_name.to_string()).into(),
                );
            }
        })
    }

    /// Gets the value of configuration option, without editing it.
    pub(crate) fn get(&mut self, chained_prop_name: &str) -> Result<Option<String>> {
        self.edit(chained_prop_name, None, false)
    }

    /// Sets a new value for a configuration option, and returns it.
    pub(crate) fn set(
        &mut self,
        chained_prop_name: &str,
        prop_value: toml::Value,
    ) -> Result<Option<String>> {
        self.edit(chained_prop_name, Some(prop_value), false)
    }

    /// Unsets a configuration option, and returns it.
    pub(crate) fn unset(&mut self, chained_prop_name: &str) -> Result<Option<String>> {
        self.edit(chained_prop_name, None, true)
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;

    use super::*;

    #[test]
    fn can_parse_non_existent_config_file() {
        // Try to parse non existent configuration file
        let path = PathBuf::new();
        let config = SparseConfig::from_config_file(&path).unwrap();
        // Default sparse configuration should thus be returned
        assert!(config.core.is_none());
    }

    fn prepare_basic_sparse_config() -> SparseConfig {
        // Prepare a valid configuration
        let conf_str = r#"
            [core]
                no_scm = true
        "#;
        // Create the configuration file
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let conf_path = temp_dir.child("config");
        conf_path.write_str(conf_str).unwrap();
        // Try to parse and return it
        SparseConfig::from_config_file(&conf_path.to_path_buf()).unwrap()
    }

    #[test]
    fn can_parse_config_file() {
        // Prepare basic configuration
        let config = prepare_basic_sparse_config();
        // Test it
        assert_eq!(config.core.unwrap().no_scm, Some(true));
    }

    #[test]
    fn can_get_config_field() {
        // Prepare basic configuration
        let mut config = prepare_basic_sparse_config();
        // Get value
        let v = config.get("core.no_scm").unwrap().unwrap();
        // Test it
        assert_eq!(v, String::from("true"));
    }

    #[test]
    fn can_set_config_field() {
        // Prepare basic configuration
        let mut config = prepare_basic_sparse_config();
        // Set value
        let v = config
            .set("core.no_scm", toml::Value::Boolean(false))
            .unwrap()
            .unwrap();
        // Test it
        assert_eq!(v, String::from("false"));
    }

    #[test]
    fn can_unset_config_field() {
        // Prepare basic configuration
        let mut config = prepare_basic_sparse_config();
        // Unset value
        let opt_v = config.unset("core.no_scm").unwrap();
        // Test it
        assert!(opt_v.is_none());
    }

    #[test]
    fn cannot_get_config_unknown_field() {
        // Prepare basic configuration
        let mut config = prepare_basic_sparse_config();
        // Try to get field with wrong key
        let res = config.get("wrong.key");
        // Check that an error was raised
        assert!(res.is_err());
    }

    #[test]
    fn can_write_config_file() {
        // Prepare sparse configuration from scratch
        let mut initial_config = SparseConfig::default();
        let mut initial_core = SparseConfigCore::default();
        initial_core.no_scm = Some(false);
        initial_config.core = Some(initial_core);
        // Try to save it to the file system
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let conf_path = temp_dir.child("config");
        let conf_path_buf = &conf_path.to_path_buf();
        initial_config.save_to_config_file(conf_path_buf).unwrap();
        // Try to re-parse it
        let parsed_config = SparseConfig::from_config_file(conf_path_buf).unwrap();
        // Test it
        assert_eq!(parsed_config.core.unwrap().no_scm.unwrap(), false);
    }
}
