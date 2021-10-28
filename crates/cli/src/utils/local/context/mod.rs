pub mod helpers;

use anyhow::Result;
use crate::utils::local::models;
use std::path::PathBuf;
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::paths::{HOLIUM_DIR, LOCAL_DIR};

pub struct LocalContext {
    pub sources: sled::Tree,
    pub shapers: sled::Tree,
    pub transformations: sled::Tree,
    pub connections: sled::Tree,
}

impl LocalContext {
    pub fn new() -> Result<Self> {
        let root_path = get_root_path()?;
        let local_area_path = root_path
            .join(HOLIUM_DIR)
            .join(LOCAL_DIR);
        LocalContext::from_local_area_path(&local_area_path)
    }

    fn from_local_area_path(local_area_path: &PathBuf) -> Result<Self> {
        let db: sled::Db = sled::open(local_area_path)?;
        LocalContext::from_db(db)
    }

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