//! Manages configuration files.

mod validation;

use crate::config::validation::{ConfigTemplate, ShadowMerge};
use crate::utils;
use anyhow::Result;
use dirs::config_dir;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors for the config module.
enum ConfigError {
    /// Thrown when trying to initialize a local configuration with no project directory provided
    #[error("cannot initialize a local configuration without a project directory")]
    LocalConfigWithNoDir,
    /// Thrown when the user's global configuration directory cannot be found
    #[error("user's configuration directory cannot be found")]
    GlobalConfigDirectoryNotFound,
}

#[derive(Clone, Debug, PartialEq)]
/// Configurations may be stored in different places that determine their level of importance.
/// Here is the list of these levels.
enum ConfigLevel {
    /// Used for cross-project configurations
    Global,
    /// Default configuration, meant to be tracked by an SCM
    Repo,
    /// Local untracked configuration
    Local,
}

/// List of all possible configuration levels, in shadowing order.
static LEVELS: &'static [ConfigLevel] =
    &[ConfigLevel::Global, ConfigLevel::Repo, ConfigLevel::Local];

/// A structure built by merging configurations from multiple levels
struct MergedConfig {
    /// Configuration resulting from the merging process
    config: validation::ConfigTemplate,
    /// Individual fragments making for the merged configuration
    fragments: Vec<Config>,
}

/// Base structure for manipulation of a configuration file
struct Config {
    /// Configuration level
    level: ConfigLevel,
    /// Path to the configuration file
    path: PathBuf,
    /// Actual Holium configuration
    config: ConfigTemplate,
}

impl Config {
    /// Creates an object to manipulate a configuration, from a configuration level and optional
    /// path to an Holium project directory.
    pub fn new(level: ConfigLevel, holium_dir: Option<PathBuf>) -> Result<Config> {
        // Get the path of the config file
        let path = get_config_file_path(&level, holium_dir)?;

        // Parse the configuration
        let config = ConfigTemplate::from_config_file(&path)?;

        Ok(Config {
            level,
            path,
            config,
        })
    }
}

impl MergedConfig {
    /// Takes an optional holium directory path and merges all existing configuration objects in
    /// shadowing order to output a unified merged configuration.
    pub fn new(holium_dir: Option<PathBuf>) -> Result<MergedConfig> {
        // Initialize a default configuration and list of fragments
        let mut config = ConfigTemplate::standard();
        let mut fragments: Vec<Config> = Vec::with_capacity(LEVELS.len());

        // Loop over all existing levels in shadowing order, get related configurations and merge them
        for level in LEVELS.iter() {
            // if no directory is provided, do not treat local levels
            // otherwise, get fragment of configuration
            let dir = match level {
                &ConfigLevel::Global => None,
                &ConfigLevel::Repo | &ConfigLevel::Local => {
                    if holium_dir.is_none() {
                        break;
                    }
                    holium_dir.clone()
                }
            };
            let fragment = Config::new(level.clone(), dir)?;
            // merge and push
            config.shadow_merge(fragment.config.clone());
            fragments.push(fragment);
        }

        Ok(MergedConfig { config, fragments })
    }
}

/// Gets the path of a config file for a specific level and, if relevant, a holium project directory path
fn get_config_file_path(level: &ConfigLevel, holium_dir: Option<PathBuf>) -> Result<PathBuf> {
    Ok(match level {
        ConfigLevel::Global => get_global_holium_dir()?.join(utils::CONFIG_FILE),
        ConfigLevel::Repo => holium_dir
            .ok_or(ConfigError::LocalConfigWithNoDir)?
            .join(utils::CONFIG_FILE),
        ConfigLevel::Local => holium_dir
            .ok_or(ConfigError::LocalConfigWithNoDir)?
            .join(utils::LOCAL_CONFIG_FILE),
    })
}

/// Gets the OS-dependent path to holium global configuration directory
fn get_global_holium_dir() -> Result<PathBuf> {
    let conf_dir = config_dir().ok_or(ConfigError::GlobalConfigDirectoryNotFound)?;
    Ok(conf_dir.join(utils::GLOBAL_PROJECT_DIR))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;

    #[test]
    fn can_build_config_object() {
        // Work on an empty directory
        let temp_dir = assert_fs::TempDir::new().unwrap();

        // Build a config
        let original_level = ConfigLevel::Repo;
        let original_path = temp_dir.to_path_buf();
        let config = Config::new(original_level.clone(), Some(original_path.clone())).unwrap();

        assert_eq!(config.level, original_level);
        assert_eq!(config.path, original_path.join("config"));
        assert_eq!(config.config.core.no_scm, None);
    }

    #[test]
    fn can_build_default_merged_config_object() {
        // Work on an empty directory
        let temp_dir = assert_fs::TempDir::new().unwrap();

        // Build a merged config with no additional configuration
        let original_path = temp_dir.to_path_buf();
        let config = MergedConfig::new(Some(original_path)).unwrap();

        assert_eq!(config.fragments.len(), 3);
        assert_eq!(config.fragments[0].level, ConfigLevel::Global);
        assert_eq!(config.fragments[1].level, ConfigLevel::Repo);
        assert_eq!(config.fragments[2].level, ConfigLevel::Local);
        assert_eq!(config.config.core.no_scm, ConfigTemplate::standard().core.no_scm);
    }

    #[test]
    fn can_build_merged_config_out_of_an_holium_repo() {
        // Build a merged config out of an holium repository
        let config = MergedConfig::new(None).unwrap();

        assert_eq!(config.fragments.len(), 1);
        assert_eq!(config.fragments[0].level, ConfigLevel::Global);
        assert_eq!(config.config.core.no_scm, ConfigTemplate::standard().core.no_scm);
    }

    #[test]
    fn can_merge_configuration_0() {
        // Prepare two valid configuration
        let repo_conf_str = r#"
            [core]
                no_scm = true
        "#;
        let local_conf_str = r#"
            [core]
                no_scm = false
        "#;

        // Create configuration files
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let repo_conf_path = temp_dir.child("config");
        let local_conf_path = temp_dir.child("config.local");
        repo_conf_path.write_str(repo_conf_str).unwrap();
        local_conf_path.write_str(local_conf_str).unwrap();

        // Build a merged config for this project
        let original_path = temp_dir.to_path_buf();
        let config = MergedConfig::new(Some(original_path)).unwrap();

        // Test
        assert_eq!(config.fragments.len(), 3);
        assert_eq!(config.config.core.no_scm, Some(false));
        assert!(config.config.core.no_dvc.is_some());
        assert_eq!(config.config.core.no_dvc, ConfigTemplate::standard().core.no_dvc);
    }
}
