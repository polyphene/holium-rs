//! Module responsible for interfacing Rust types representing objects from the Holium Framework
//! with their stored representations of a file system.

use std::fs;
use std::path::{PathBuf};
use thiserror::Error;

use anyhow::{Context, Result};

use holium::data::linked_data_tree::{
    Value as LinkedDataTreeValue,
    Node as LinkedDataTreeNode,
};

use crate::utils::OBJECTS_DIR;

const CID_SPLIT_POSITION: usize = 9;


#[derive(Error, Debug)]
/// Errors for the storage utility module.
enum StorageError {
    /// Thrown when failing to write data object in the repository
    #[error("failed to write holium data object : {0}")]
    FailedToWriteObject(String),
}

/// Contextual structure representing a Holium repository on a file system
pub(crate) struct RepoStorage {
    /// Root path of the holium repository. Basically, it should lead to a `.holium` directory.
    root: PathBuf,
}

impl RepoStorage {
    /// Create a [ RepoStorage ] from its root path.
    pub(crate) fn new(root_path: &PathBuf) -> Self {
        RepoStorage { root: root_path.clone() }
    }

    /// Write a [LinkedDataTreeValue] to a single file on the file system and return its CID.
    fn write_data_tree_value(&self, v: &LinkedDataTreeValue) -> Result<String> {
        // Build path to write to, based on the CID string representation broken into two parts
        let cid_str = &v.cid.to_string();
        let (cid_prefix, cid_suffix) = cid_str.split_at(CID_SPLIT_POSITION);
        let path = self.root
            .join(OBJECTS_DIR)
            .join(cid_prefix)
            .join(cid_suffix);
        // Create the parent directory if missing
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?
        };
        // Write data to file
        fs::write(&path, &v.cbor)
            .context(StorageError::FailedToWriteObject(cid_str.to_string()))?;
        // Return
        Ok(cid_str.to_string())
    }

    /// Recursively write all nodes from a Linked Data Tree to independent files on the file system.
    ///
    /// ## Warning
    ///
    /// Infinite loops are, for now, left undetected !
    pub(crate) fn write_data_tree(&self, n: &LinkedDataTreeNode) -> Result<String> {
        // Recursively write the tree
        for child in &n.children {
            self.write_data_tree(child)?;
        }
        // Write current node
        let cid_str = self.write_data_tree_value(&n.value)?;
        Ok(cid_str)
    }
}