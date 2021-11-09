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
    #[error("failed to initialize interplanetary context")]
    FailedToInit,
}

/// Context structure helping accessing the interplanetary area in a consistent way throughout the CLI
/// commands.
pub struct InterplanetaryContext {
    pub ip_area_path: PathBuf,
}

impl InterplanetaryContext {
    /// Public function helping to initialize a [ InterplanetaryContext ] object, from the implementation of
    /// any command of the CLI
    pub fn new() -> Result<Self> {
        let root_path = get_root_path()?;
        Self::from_root_path(&root_path)
    }

    /// Initialize a [ InterplanetaryContext ] object in a temporary directory.
    pub fn new_tmp() -> Result<Self> {
        let root_dir = tempdir()
            .context(Error::FailedToInit)?;
        let root_path = root_dir.path().to_path_buf();
        Self::from_root_path(&root_path)
    }

    /// Initialize an interplanetary context from a project root path.
    fn from_root_path(root_path: &PathBuf) -> Result<Self> {
        // create the holium root directory if it does not exist
        let holium_root_path = root_path
            .join(HOLIUM_DIR);
        if !holium_root_path.exists() { fs::create_dir(&holium_root_path).context(Error::FailedToInit)? }
        // create the interplanetary area directory if it does not exist
        let ip_area_path = holium_root_path.join(INTERPLANETARY_DIR);
        if !ip_area_path.exists() { fs::create_dir(&ip_area_path).context(Error::FailedToInit)? }
        // configure local context
        Ok(InterplanetaryContext { ip_area_path })
    }
}
