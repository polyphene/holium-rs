//! Module responsible for interfacing Rust types representing objects from the Holium Framework
//! with their stored representations of a file system.

use std::{env, fs};
use std::convert::TryFrom;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use cid::Cid;
use console::style;
use thiserror::Error;

use holium::data::linked_data_tree::{
    Node as LinkedDataTreeNode,
    Value as LinkedDataTreeValue,
};
use holium::fragment_serialize::HoliumDeserializable;

use crate::utils::{OBJECTS_DIR, PROJECT_DIR};
use crate::utils::storage::StorageError::{FailedToParseCid, WrongObjectPath};

const CID_SPLIT_POSITION: usize = 9;


#[derive(Error, Debug)]
/// Errors for the storage utility module.
enum StorageError {
    /// This error is thrown when a command that should only be run inside a Holium repository is ran
    /// outside of any repository.
    #[error("this command can only be run inside a Holium repository")]
    OutsideHoliumRepo,
    /// Thrown when failing to write data object in the repository
    #[error("failed to write holium data object : {0}")]
    FailedToWriteObject(String),
    /// Thrown when an error occurs while working with the path of a supposed object in a local repository
    #[error("wrong local object path : {0}")]
    WrongObjectPath(String),
    /// Thrown when failing to make a valid CID from a string
    #[error("failed to parse CID string : {0}")]
    FailedToParseCid(String),
}

/// Deterministically convert an object CID to a path for storage.
/// The result path should start with `objects/`.
fn cid_to_object_path(cid: &Cid) -> PathBuf {
    // Base the result path on the CID string representation broken into two parts
    let cid_str = cid.to_string();
    let (cid_prefix, cid_suffix) = cid_str.split_at(CID_SPLIT_POSITION);
    PathBuf::new()
        .join(OBJECTS_DIR)
        .join(cid_prefix)
        .join(cid_suffix)
}

/// Deterministically convert an object local path to related CID.
/// This function should be the inverse of [cid_to_object_path].
fn object_path_to_cid(path: PathBuf) -> Result<Cid> {
    let default_err = || { WrongObjectPath(path.to_string_lossy().to_string()) };
    let mut ancestors = path.ancestors();
    let cid_suffix = ancestors
        .next().ok_or(default_err())?
        .file_name().ok_or(default_err())?;
    let cid_prefix = ancestors
        .next().ok_or(default_err())?
        .file_name().ok_or(default_err())?;
    let cid_str = format!("{}{}", cid_prefix.to_string_lossy(), cid_suffix.to_string_lossy());
    Cid::try_from(cid_str.clone()).context(FailedToParseCid(cid_str))
}

/// Contextual structure representing a Holium repository on a file system
pub(crate) struct RepoStorage {
    /// Root path of the holium repository. Basically, it should lead to a `.holium` directory.
    root: PathBuf,
    /// List of data objects' CIDs
    pub(crate) data_cids: Vec<Cid>,
}

impl RepoStorage {
    /// Create a [ RepoStorage ] from its root path.
    fn new(root_path: &PathBuf) -> Result<Self> {
        // check that root path is indeed inside a valid repository
        if !root_path.exists() || !root_path.is_dir() {
            return Err(StorageError::OutsideHoliumRepo.into());
        }

        /*
         build the `root` field value
         */

        let root = root_path.clone();

        /*
         build the `data_cids` field value
         */

        let mut data_cids: Vec<Cid> = Vec::new();
        let object_dir = root.join(OBJECTS_DIR);
        // if `objects` is a file, warn the user
        if object_dir.exists() && object_dir.is_file() {
            eprintln!("{}", style("found a file instead of the objects directory").yellow())
        } else {
            if !object_dir.exists() {
                // if the objects directory does not exist, create it
                fs::create_dir(&object_dir)
                    .context(anyhow!("failed to create objects directory"))?;
            }
            for sup_entry in fs::read_dir(&object_dir)
                .context(anyhow!("failed to read objects directory"))? {
                let sup_entry = sup_entry
                    .context(anyhow!("failed to read objects directory"))?;
                let sup_path = sup_entry.path();
                if sup_path.is_dir() {
                    for sub_entry in fs::read_dir(&sup_path)
                        .context(anyhow!("failed to read sub objects directory"))? {
                        let sub_entry = sub_entry
                            .context(anyhow!("failed to read sub objects directory"))?;
                        let sub_path = sub_entry.path();
                        // read file
                        let data = fs::read(&sub_path)
                            .context(anyhow!("failed to read object file: {}", sub_path.to_string_lossy()))?;
                        // check that path leads to a valid CID and build it
                        let cid_res = object_path_to_cid(sub_path);
                        match cid_res {
                            Err(e) => eprintln!("{}", style(e).yellow()),
                            Ok(cid) => {
                                // try to recognize the type of object and push its CID to the right context field
                                match data {
                                    _ if { LinkedDataTreeValue::is_of_type(&data)? } => {
                                        data_cids.push(cid)
                                    }
                                    _ => {
                                        eprintln!("{}", style(format!("could not detect the type of an object: {}", cid.to_string())).yellow())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // return
        Ok(RepoStorage { root, data_cids })
    }

    /// Create a [ RepoStorage ] from current directory, fetched from environment, that may be up to
    /// the parent directory of a `.holium` directory.
    pub(crate) fn from_cur_dir() -> Result<Self> {
        let cur_dir = env::current_dir()?;
        let holium_dir = cur_dir.join(PROJECT_DIR);
        Self::new(&holium_dir)
    }

    /// Write a [LinkedDataTreeValue] to a single file on the file system and return its CID.
    fn write_data_tree_value(&self, v: &LinkedDataTreeValue) -> Result<String> {
        // Build path to write to, based on the CID string representation broken into two parts
        let path = self.root
            .join(cid_to_object_path(&v.cid));
        // Create the parent directory if missing
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?
        };
        // Write data to file
        let cid_str = v.cid.to_string();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_to_object_path() {
        let cid = Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap();
        let path = cid_to_object_path(&cid);
        assert_eq!(path, PathBuf::new().join("objects").join("bafir4idb").join("vg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq"))
    }

    #[test]
    fn test_object_path_to_cid() {
        let path = PathBuf::new().join("objects").join("bafir4idb").join("vg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq");
        let cid = object_path_to_cid(path).unwrap();
        assert_eq!(cid, Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap())
    }

    #[test]
    fn test_cid_to_object_path_to_cid() {
        let cid = Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap();
        assert_eq!(
            cid,
            object_path_to_cid(cid_to_object_path(&cid)).unwrap()
        )
    }
}