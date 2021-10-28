pub mod helpers;

use anyhow::Result;
use crate::utils::local::models;
use std::path::PathBuf;
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::paths::{HOLIUM_DIR, LOCAL_DIR};

/// Context structure helping accessing the local store in a consistent way throughout the CLI
/// commands.
pub struct LocalContext {
    pub sources: sled::Tree,
    pub shapers: sled::Tree,
    pub transformations: sled::Tree,
    pub connections: sled::Tree,
}

impl LocalContext {
    /// Public function helping to initialize a [ LocalContext ] object, from the implementation of
    /// any command of the CLI, and whatever the current directory the command has been called from,
    /// provided it is inside a Holium-initialized project.
    pub fn new() -> Result<Self> {
        let root_path = get_root_path()?;
        let local_area_path = root_path
            .join(HOLIUM_DIR)
            .join(LOCAL_DIR);
        LocalContext::from_local_area_path(&local_area_path)
    }

    /// Initialize a [ LocalContext ] object from the path of a local Holium area directory.
    fn from_local_area_path(local_area_path: &PathBuf) -> Result<Self> {
        let db: sled::Db = sled::open(local_area_path)?;
        LocalContext::from_db(db)
    }

    /// Initialize a [ LocalContext ] object from a [ sled::Db ] object.
    fn from_db(db: sled::Db) -> Result<Self> {
        let sources: sled::Tree = db.open_tree(models::source::TREE_NAME)?;
        sources.set_merge_operator(models::source::merge);
        let shapers: sled::Tree = db.open_tree(models::shaper::TREE_NAME)?;
        shapers.set_merge_operator(models::shaper::merge);
        let transformations: sled::Tree = db.open_tree(models::transformation::TREE_NAME)?;
        transformations.set_merge_operator(models::transformation::merge);
        let connections: sled::Tree = db.open_tree(models::connection::TREE_NAME)?;
        connections.set_merge_operator(models::connection::merge);
        Ok(LocalContext{ sources, shapers, transformations, connections })
    }
}