//! Manages configuration files.

mod validation;

use std::path::PathBuf;
use crate::config::validation::ConfigTemplate;


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
static LEVELS: &'static [ConfigLevel] = &[
    ConfigLevel::Global,
    ConfigLevel::Repo,
    ConfigLevel::Local
];

/// A structure built by merging configurations from multiple levels
pub struct MergedConfig {
    /// Configuration resulting from the merging process
    config: validation::ConfigTemplate,
    /// Individual fragments making for the merged configuration
    fragments: [Config],
}

/// Base structure for manipulation of a configuration file
pub struct Config {
    /// Configuration level
    level: ConfigLevel,
    /// Path to the configuration file
    path: PathBuf,
    /// Actual Holium configuration
    config: ConfigTemplate,
}
