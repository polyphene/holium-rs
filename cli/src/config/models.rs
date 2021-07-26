//! Defines objects used to manipulate configurations of Holium repositories.

use std::path::PathBuf;

use anyhow::Result;
use dirs::config_dir;
use thiserror::Error;

use crate::config::config::Config;
use crate::config::sparse_config::SparseConfig;
use crate::utils;

#[derive(Error, Debug)]
/// Errors for the config module.
pub(crate) enum ConfigError {
    /// Thrown when trying to initialize a local configuration with no project directory provided
    #[error("cannot initialize a local configuration without a project directory")]
    LocalConfigWithNoDir,
    /// Thrown when the user's global configuration directory cannot be found
    #[error("user's configuration directory cannot be found")]
    GlobalConfigDirectoryNotFound,
    /// Thrown when trying to access an unsupported configuration option
    #[error("unknown configuration option key : {0}")]
    ConfigTemplateUnknownKey(String),
    /// Thrown when failing to serialize a configuration
    #[error("failed to serialize configuration to TOML")]
    ConfigFileSerialization,
    /// Thrown when failing to write configuration file
    #[error("failed to write configuration file : {0}")]
    ConfigFileWrite(String),
    /// Thrown when failing to deserialize a configuration file
    #[error("failed to read the configuration file : {0}")]
    ConfigFileDeserialization(String),
}

#[derive(Clone, Debug, PartialEq)]
/// Configurations may be stored in different places that determine their level of importance.
/// Here is the list of these levels.
pub(crate) enum ConfigLevel {
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
pub(crate) struct ProjectConfig {
    /// Configuration resulting from the merging process
    pub(crate) config: Config,
    /// Individual fragments making for the merged configuration
    fragments: Vec<ProjectConfigFragment>,
}

impl ProjectConfig {
    /// Takes an optional holium directory path and merges all existing sparse configuration objects in
    /// shadowing order to output a unified merged configuration.
    pub fn new(holium_dir: Option<PathBuf>) -> Result<ProjectConfig> {
        // Initialize a default configuration and list of fragments
        let mut config = Config::standard();
        let mut fragments: Vec<ProjectConfigFragment> = Vec::with_capacity(LEVELS.len());
        // Loop over all existing levels in shadowing order, get related configuration and merge
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
            let fragment = ProjectConfigFragment::new(level.clone(), dir)?;
            // merge sparse configuration
            config.merge(&fragment.config);
            // push to the list of fragment
            fragments.push(fragment);
        }
        // ProjectConfig
        Ok(ProjectConfig { config, fragments })
    }
}

/// Base structure for manipulation of a sparse configuration in a project configuration
pub(crate) struct ProjectConfigFragment {
    /// Configuration level
    level: ConfigLevel,
    /// Path to the configuration file
    pub(crate) path: PathBuf,
    /// Actual sparse configuration
    pub(crate) config: SparseConfig,
}

impl ProjectConfigFragment {
    /// Creates an object to manipulate a configuration, from a configuration level and optional
    /// path to an Holium project directory.
    pub(crate) fn new(
        level: ConfigLevel,
        holium_dir: Option<PathBuf>,
    ) -> Result<ProjectConfigFragment> {
        // Get the path of the config file
        let path = get_config_file_path(&level, holium_dir)?;
        // Parse the configuration
        let config = SparseConfig::from_config_file(&path)?;
        // Return object
        Ok(ProjectConfigFragment {
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
    use assert_fs::prelude::*;

    use super::*;

    #[test]
    fn can_build_project_config_fragment() {
        // Work on an empty directory
        let temp_dir = assert_fs::TempDir::new().unwrap();
        // Build a config
        let original_level = ConfigLevel::Repo;
        let original_path = temp_dir.to_path_buf();
        let config =
            ProjectConfigFragment::new(original_level.clone(), Some(original_path.clone()))
                .unwrap();
        // Test it
        assert_eq!(config.level, original_level);
        assert_eq!(config.path, original_path.join("config"));
        assert!(config.config.core.is_none());
    }

    #[test]
    fn can_build_default_project_config_object() {
        // Work on an empty directory
        let temp_dir = assert_fs::TempDir::new().unwrap();
        // Build a merged config with no additional configuration
        let original_path = temp_dir.to_path_buf();
        let config = ProjectConfig::new(Some(original_path)).unwrap();
        // Test it
        assert_eq!(config.fragments.len(), 3);
        assert_eq!(config.fragments[0].level, ConfigLevel::Global);
        assert_eq!(config.fragments[1].level, ConfigLevel::Repo);
        assert_eq!(config.fragments[2].level, ConfigLevel::Local);
        assert_eq!(config.config.core.no_scm, Config::standard().core.no_scm);
    }

    #[test]
    fn can_build_project_config_outside_of_an_holium_repo() {
        // Build a merged config out of an holium repository
        let config = ProjectConfig::new(None).unwrap();
        // Test it
        assert_eq!(config.fragments.len(), 1);
        assert_eq!(config.fragments[0].level, ConfigLevel::Global);
        assert_eq!(config.config.core.no_scm, Config::standard().core.no_scm);
    }

    #[test]
    fn can_merge_configuration_fragments() {
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
        let config = ProjectConfig::new(Some(original_path)).unwrap();
        // Test
        assert_eq!(config.fragments.len(), 3);
        assert_eq!(config.config.core.no_scm, false);
        assert_eq!(config.config.core.no_dvc, Config::standard().core.no_dvc);
        assert!(config.fragments[0].config.core.is_none());
        assert_eq!(config.fragments[1].config.core.as_ref().unwrap().no_scm.unwrap(), true);
        assert_eq!(config.fragments[2].config.core.as_ref().unwrap().no_scm.unwrap(), false);
    }
}
