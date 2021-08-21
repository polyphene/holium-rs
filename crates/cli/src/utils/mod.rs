//! Shared utilities
use thiserror::Error;

pub(crate) mod storage;

/// Name of the global directory where data related to the Holium Framework is stored.
pub(crate) const GLOBAL_PROJECT_DIR: &'static str = "holium";
/// Name of the directory where all data related to the Holium Framework in a repository is stored.
pub(crate) const PROJECT_DIR: &'static str = ".holium";
/// Name of the cache directory.
pub(crate) const CACHE_DIR: &'static str = "cache";
/// Name of the objects directory.
pub(crate) const OBJECTS_DIR: &'static str = "objects";
/// Name of the configuration file.
pub(crate) const CONFIG_FILE: &'static str = "config";
/// Name of the local, untracked, configuration file.
pub(crate) const LOCAL_CONFIG_FILE: &'static str = "config.local";
