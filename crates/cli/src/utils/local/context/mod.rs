use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use sled::Db;

use crate::utils::local::models;
use crate::utils::repo::constants::{HOLIUM_DIR, LOCAL_DIR, PORTATIONS_FILE};
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::models::portation::Portations;
use crate::utils::local::context::helpers::NodeType;
use tempfile::tempdir;
use std::fs;

pub mod helpers;
pub mod constants;

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
        Self::from_root_path(&root_path)
    }

    /// Initialize a [ LocalContext ] object in a temporary directory.
    pub fn new_tmp() -> Result<Self> {
        let root_dir = tempdir()
            .context("TODO")?;
        let root_path = root_dir.path().to_path_buf();
        Self::from_root_path(&root_path)
    }

    /// Initialize a local context from a project root path.
    fn from_root_path(root_path: &PathBuf) -> Result<Self> {
        // create the holium root directory if it does not exist
        let holium_root_path = root_path
            .join(HOLIUM_DIR);
        if !holium_root_path.exists() { fs::create_dir(&holium_root_path).context("TODO")? }
        // create the local area directory if it does not exist
        let local_area_path = holium_root_path.join(LOCAL_DIR);
        if !local_area_path.exists() { fs::create_dir(&local_area_path).context("TODO")? }
        // initialize database handle
        let db: sled::Db = sled::open(&local_area_path).context("TODO")?;
        // create the portation file if it does not exist
        let portations_file_path = holium_root_path.join(PORTATIONS_FILE);
        if !portations_file_path.exists() {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&portations_file_path)
                .context("TODO")?;
        }
        // configure local context
        LocalContext::from_db_and_conf_files(db, portations_file_path)
    }

    /// Initialize a [ LocalContext ] object from a project a local [ sled::Db ] object
    /// and the path of the portations file.
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
        Ok(LocalContext { sources, shapers, transformations, connections, portations })
    }

    /// For all fields of a local context, select the ones related to nodes of a [ PipelineDag ] and
    /// tuple them with the related [ NodeType ].
    pub fn get_nodes_tree_type_tuples(&self) -> Vec<(&sled::Tree, NodeType)> {
        vec![
            (&self.sources, NodeType::source),
            (&self.shapers, NodeType::shaper),
            (&self.transformations, NodeType::transformation),
        ]
    }
}
