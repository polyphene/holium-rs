use anyhow::Result;
use crate::utils::local::models::transformation;
use std::path::PathBuf;
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::paths::{HOLIUM_DIR, LOCAL_DIR};

pub struct LocalContext {
    pub transformations: sled::Tree,
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
        let transformations: sled::Tree = db.open_tree(transformation::TREE_NAME)?;
        transformations.set_merge_operator(transformation::merge);
        Ok(LocalContext{ transformations })
    }
}