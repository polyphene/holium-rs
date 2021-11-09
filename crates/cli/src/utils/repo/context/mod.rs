use std::fs::{File, OpenOptions};
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use sled::Db;
use tempfile::tempdir;

use crate::utils::local::context::helpers::NodeType;
use crate::utils::local::models;
use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR, LOCAL_DIR, PORTATIONS_FILE};
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::models::portation::Portations;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to initialize repository context")]
    FailedToInit,
}

/// Context structure helping accessing the repository area in a consistent way throughout the CLI
/// commands.
pub struct RepositoryContext {
    pub portations: Portations,
}

impl RepositoryContext {
    /// Public function helping to initialize a [ RepositoryContext ] object, from the implementation of
    /// any command of the CLI
    pub fn new() -> Result<Self> {
        let root_path = get_root_path()?;
        Self::from_root_path(&root_path)
    }

    /// Initialize an repository context from a project root path.
    fn from_root_path(root_path: &PathBuf) -> Result<Self> {
        // create the holium root directory if it does not exist
        let holium_root_path = root_path
            .join(HOLIUM_DIR);
        if !holium_root_path.exists() { fs::create_dir(&holium_root_path).context(Error::FailedToInit)? }
        // create the portation file if it does not exist
        let portations_file_path = holium_root_path.join(PORTATIONS_FILE);
        if !portations_file_path.exists() {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&portations_file_path)
                .context(Error::FailedToInit)?;
        }
        // Get portations handler from the configuration file
        let portations = Portations::from_path(portations_file_path)?;
        // configure local context
        Ok(RepositoryContext { portations })
    }
}
