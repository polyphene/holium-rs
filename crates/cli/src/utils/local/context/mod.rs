use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use sled::Db;

use crate::utils::local::models;
use crate::utils::repo::constants::{HOLIUM_DIR, LOCAL_DIR, PORTATIONS_FILE};
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::models::portation::Portations;
use crate::utils::local::context::helpers::NodeType;
use tempfile::{tempdir, TempDir};
use std::fs;

pub mod constants;
pub mod helpers;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to initialize local context")]
    FailedToInit,
}

/// Context structure helping accessing the local store in a consistent way throughout the CLI
/// commands.
pub struct LocalContext {
    pub data: sled::Tree,
    pub root_path: PathBuf,
    pub db: sled::Db,
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

    /// Initialize a [ LocalContext ] object in a new temporary directory.
    pub fn new_tmp() -> Result<(Self, TempDir)> {
        let root_dir = tempdir()
            .context(Error::FailedToInit)?;
        let root_path = root_dir.path().to_path_buf();
        Ok((Self::from_root_path(&root_path)?, root_dir))
    }

    /// Initialize a local context from a project root path.
    fn from_root_path(root_path: &PathBuf) -> Result<Self> {
        // create the holium root directory if it does not exist
        let holium_root_path = root_path
            .join(HOLIUM_DIR);
        if !holium_root_path.exists() { fs::create_dir(&holium_root_path).context(Error::FailedToInit)? }
        // create the local area directory if it does not exist
        let local_area_path = holium_root_path.join(LOCAL_DIR);
        if !local_area_path.exists() { fs::create_dir(&local_area_path).context(Error::FailedToInit)? }
        // initialize database handle
        let db: sled::Db = sled::open(&local_area_path).context(Error::FailedToInit)?;
        // create the portation file if it does not exist
        let portations_file_path = holium_root_path.join(PORTATIONS_FILE);
        if !portations_file_path.exists() {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(&portations_file_path)
                .context(Error::FailedToInit)?;
        }
        // configure local context
        LocalContext::from_db_and_conf_files(root_path, db, portations_file_path)
    }

    /// Initialize a [ LocalContext ] object from a project a local [ sled::Db ] object
    /// and the path of the portations file.
    fn from_db_and_conf_files(root_path: &PathBuf, db: sled::Db, portations_file_path: PathBuf) -> Result<Self> {
        // Get trees from the DB
        let data: sled::Tree = db.open_tree(models::data::TREE_NAME)?;
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
        Ok(LocalContext { data, root_path: root_path.clone(), db, sources, shapers, transformations, connections, portations })
    }

    /// Move local area from a context to another.
    ///
    /// # Warnings
    ///
    /// The destination local context may be inconsistent after this operation and should be
    /// rebuilt from its root path.
    pub fn mv_local_area(&self, destination: &Self) -> Result<()> {
        // manually replace the destination database with a new one
        let to_local_area_path = destination.root_path.join(HOLIUM_DIR).join(LOCAL_DIR);
        if to_local_area_path.exists() {
            fs::remove_dir_all(&to_local_area_path)?;
        }
        fs::create_dir_all(&to_local_area_path);
        let new_db = sled::open(&to_local_area_path)?;
        // export and import the db to move it
        let dump = self.db.export();
        new_db.import(dump);
        Ok(())
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
