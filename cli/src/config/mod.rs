//! Manages configuration files.

mod validation;

use crate::config::validation::ConfigTemplate;
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
    fragments: [Config],
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
        assert!(config.config.core.is_none());
    }
}
