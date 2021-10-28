pub mod helpers;

use anyhow::{anyhow, Result, Context};
use crate::utils::local::models;
use std::path::PathBuf;
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::constants::{HOLIUM_DIR, LOCAL_DIR, PORTATIONS_FILE};
use crate::utils::repo::models::portation::Portations;
use sled::Db;
use std::fs::File;

/// Context structure helping accessing the local store in a consistent way throughout the CLI
/// commands.
pub struct LocalContext {
    pub sources: sled::Tree,
    pub shapers: sled::Tree,
    pub transformations: sled::Tree,
    pub connections: sled::Tree,
    pub portations: Portations,
}

impl LocalContext {
    /// Public function helping to initialize a [ LocalContext ] object, from the implementation of
    /// any command of the CLI, and whatever the current directory the command has been called from,
    /// provided it is inside a Holium-initialized project.
    pub fn new() -> Result<Self> {
        let root_path = get_root_path()?;
        let holium_root_path = root_path
            .join(HOLIUM_DIR);
        LocalContext::from_holium_root_path(&holium_root_path)
    }

    /// Initialize a [ LocalContext ] object from the path of a Holium root directory path.
    fn from_holium_root_path(holium_root_path: &PathBuf) -> Result<Self> {
        let local_area_path = holium_root_path.join(LOCAL_DIR);
        let db: sled::Db = sled::open(local_area_path)?;
        let portations_file_path = holium_root_path.join(PORTATIONS_FILE);
        LocalContext::from_db_and_conf_files(db, portations_file_path)
    }

    /// Initialize a [ LocalContext ] object from a local [ sled::Db ] object and the path of the
    /// portations file.
    fn from_db_and_conf_files(db: sled::Db, portations_file_path: PathBuf) -> Result<Self> {
        // Get trees from the DB
        let sources: sled::Tree = db.open_tree(models::source::TREE_NAME)?;
        sources.set_merge_operator(models::source::merge);
        let shapers: sled::Tree = db.open_tree(models::shaper::TREE_NAME)?;
        shapers.set_merge_operator(models::shaper::merge);
        let transformations: sled::Tree = db.open_tree(models::transformation::TREE_NAME)?;
        transformations.set_merge_operator(models::transformation::merge);
        let connections: sled::Tree = db.open_tree(models::connection::TREE_NAME)?;
        connections.set_merge_operator(models::connection::merge);
        // Get portations handler from the configuration file
        let portations = Portations::from_path(portations_file_path)?;
        // Return the context handler
        Ok(LocalContext{ sources, shapers, transformations, connections, portations })
    }
}