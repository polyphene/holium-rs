//! Helper methods related to the use of multiformats.
//! Reference: https://multiformats.io/

use std::{env, fs, io};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use sk_cbor::Value;
use cid::{Cid, Version};
use cid::multihash::Multihash;
use cid::multibase::Base;
use console::style;
use thiserror;

use holium::data::linked_data_tree::{
    Node as LinkedDataTreeNode,
    Value as LinkedDataTreeValue,
};
use holium::fragment_serialize::HoliumDeserializable;
use holium::transformation::Transformation;

use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR};
use crate::utils::repo::errors::Error::OutsideHoliumRepo;
use crate::utils::interplanetary::multiformats::Error::{WrongObjectPath, FailedToParseCid};
use std::io::{Read, Seek};
use crate::utils::local::context::LocalContext;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;


/// Blake3 multicodec code.
pub const BLAKE3_HASH_FUNC_TYPE: u8 = 0x1e;
/// Cid version used in the framework
pub const CID_VERSION: Version = Version::V1;
/// Default multi base for printing and file naming operations
pub const DEFAULT_MULTIBASE: Base = Base::Base32Lower;

#[derive(thiserror::Error, Debug)]
/// Errors for the interplanetary utility module.
pub(crate) enum Error {
    #[error("failed to run the hashing algorithm")]
    Blake3HashingError,
    #[error("failed to create multihash from blake3 hash")]
    Blake3MultihashCreationError,
    #[error("failed to create cid")]
    CidCreationError,
    #[error("failed to create block path from cid")]
    BlockPathCreationError,
    #[error("failed to parse block path as cid")]
    CidFromPathError,
    /// Thrown when failing to write data object in the repository
    #[error("failed to write holium data object : {0}")]
    FailedToWriteObject(String),
    /// Thrown when an error occurs while working with the path of a supposed object in a local repository
    #[error("wrong local object path : {0}")]
    WrongObjectPath(String),
    /// Thrown when failing to make a valid CID from a string
    #[error("failed to parse CID string : {0}")]
    FailedToParseCid(String),
    /// Thrown when a provided identifier cannot be linked to a known Holium object
    #[error("unknown object identifier: {0}")]
    UnknownObjectIdentifier(String),
}

/// Compute the cid of a block;
pub fn compute_cid<T: Read + Seek>(mut content: T, codec: &BlockMulticodec) -> Result<Cid> {
    // initialize a hasher
    let mut hasher = blake3::Hasher::new();
    // write block content to it
    io::copy(&mut content, &mut hasher)
        .context(Error::Blake3HashingError)?;
    content.rewind()
        .context(Error::Blake3HashingError)?;
    // finalize hashing
    let hash = hasher.finalize();
    // create multihash
    let multihash = blake3_hash_to_multihash(*hash.as_bytes())?;
    // create and return cid
    let cid = Cid::new(CID_VERSION,  codec.into(), multihash)
        .context(Error::CidCreationError)?;
    Ok(cid)
}

/// Create a Multihash from a 32-byte blake3 digest.
/// [hash] should be the output of the Blake3 algorithm although obviously no verification
/// is performed.
pub fn blake3_hash_to_multihash(hash: [u8; 32]) -> Result<Multihash> {
    let mut multihash_bytes = vec![BLAKE3_HASH_FUNC_TYPE, hash.len() as u8];
    multihash_bytes.extend_from_slice(hash.as_ref());
    Multihash::from_bytes(multihash_bytes.as_slice())
        .context(Error::Blake3MultihashCreationError)
}


/// Deterministically convert an object CID to a path for local storage.
pub(crate) fn cid_to_path(cid: &Cid, local_context: &LocalContext) -> Result<PathBuf> {
    // create relative path from cid
    let rel_path = cid_to_object_path(&cid)?;
    // create absolute path with context and return
    Ok(local_context.root_path
        .join(HOLIUM_DIR)
        .join(INTERPLANETARY_DIR)
        .join(rel_path))
}

/// Deterministically convert an object CID to a relative path.
/// A form of sharding is performed.
/// We follow specifications from the `ipfs/go-ds-flatfs` repository.
/// Reference: https://github.com/ipfs/go-ds-flatfs/blob/master/readme.go
fn cid_to_object_path(cid: &Cid) -> Result<PathBuf> {
    // create base 32 cid string
    let cid_str = cid.to_string_of_base(DEFAULT_MULTIBASE)
        .context(Error::BlockPathCreationError)?;
    // get the next-to-last two characters
    let c0 = cid_str.chars().nth_back(2).ok_or(Error::BlockPathCreationError)?;
    let c1 = cid_str.chars().nth_back(1).ok_or(Error::BlockPathCreationError)?;
    // remove multibase character
    let mut cs = cid_str.chars();
    cs.next().ok_or(Error::BlockPathCreationError)?;
    // create relative path
    Ok(PathBuf::from(format!("{}{}/{}", c0, c1, cs.as_str())))
}

/// Deterministically convert an object absolute path used for local storage to related CID.
pub fn path_to_cid(path: &PathBuf, local_context: &LocalContext) -> Result<Cid> {
    // check if given path is in expected directory according to the context
    let interplanetary_dir_path = local_context.root_path
        .join(HOLIUM_DIR)
        .join(INTERPLANETARY_DIR);
    if !path.starts_with(&interplanetary_dir_path) {
        return Err(Error::CidFromPathError.into())
    }
    // strip prefix, convert relative path to cid, and return it
    let rel_path = path
        .strip_prefix(&interplanetary_dir_path)
        .context(Error::CidFromPathError)?;
    object_path_to_cid(&rel_path.to_path_buf())
}

/// Deterministically convert an object local path to related CID.
/// This function should be the inverse of [cid_to_object_path].
fn object_path_to_cid(path: &PathBuf) -> Result<Cid> {
    let file_name = path.file_name()
        .ok_or(Error::CidFromPathError)?;
    let cid_str = format!("b{}", file_name.to_string_lossy());
    Cid::try_from(cid_str)
        .context(Error::CidFromPathError)
}


// TODO remove
/// Contextual structure representing a Holium repository on a file system
#[deprecated]
pub(crate) struct RepoStorage {
    /// Root path of the holium repository. Basically, it should lead to a `.holium` directory.
    pub(crate) root: PathBuf,
    /// List of data objects' CIDs
    pub(crate) data_cids: Vec<Cid>,
    /// List of transformation objects' CIDs
    pub(crate) transformation_cids: Vec<Cid>,
}

// TODO remove
impl RepoStorage {
    /// Create a [ RepoStorage ] from its root path.
    fn new(root_path: &PathBuf) -> Result<Self> {
        // check that root path is indeed inside a valid repository
        if !root_path.exists() || !root_path.is_dir() {
            return Err(OutsideHoliumRepo.into());
        }

        /*
         build the `root` field
         */

        let root = root_path.clone();

        /*
         build the `data_cids` and `transformation_cids` fields
         */

        let mut data_cids: Vec<Cid> = Vec::new();
        let mut transformation_cids: Vec<Cid> = Vec::new();
        let object_dir = root.join(INTERPLANETARY_DIR);
        // if `objects` is a file, warn the user
        if object_dir.exists() && object_dir.is_file() {
            eprintln!("{}", style("found a file instead of the objects directory").yellow())
        } else {
            if object_dir.exists() {
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
                            // open file
                            let mut data_reader = fs::File::open(&sub_path)
                                .context(anyhow!("failed to open object file: {}", sub_path.to_string_lossy()))?;
                            // check that path leads to a valid CID and build it
                            let cid_res = object_path_to_cid(&sub_path);
                            match cid_res {
                                Err(e) => eprintln!("{}", style(e).yellow()),
                                Ok(cid) => {
                                    // try to recognize the type of object and push its CID to the right context field
                                    fn test_type<T: HoliumDeserializable>(mut f: &fs::File) -> Result<bool> {
                                        io::Seek::seek(&mut f, io::SeekFrom::Start(0)).context("TODO")?;
                                        T::is_of_type(&mut f)
                                    }
                                    // if Transformation::is_of_type(&mut data_reader)? {
                                    if test_type::<Transformation>(&data_reader)? {
                                        transformation_cids.push(cid)
                                        // } else if LinkedDataTreeValue::is_of_type(&mut data_reader)? {
                                    } else if test_type::<LinkedDataTreeValue>(&data_reader)? {
                                        data_cids.push(cid)
                                    } else {
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
        Ok(RepoStorage { root, data_cids, transformation_cids })
    }

    /// Create a [ RepoStorage ] from current directory, fetched from environment, that may be up to
    /// the parent directory of a `.holium` directory.
    pub(crate) fn from_cur_dir() -> Result<Self> {
        let cur_dir = env::current_dir()?;
        let holium_dir = cur_dir.join(HOLIUM_DIR);
        Self::new(&holium_dir)
    }

    /// Write a [LinkedDataTreeValue] to a single file on the file system and return its CID.
    fn write_data_tree_value(&self, v: &LinkedDataTreeValue) -> Result<String> {
        // Build path to write to, based on the CID string representation broken into two parts
        let path = self.root
            .join(cid_to_path(&v.cid, &LocalContext::new().unwrap()).unwrap());
        // Create the parent directory if missing
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?
        };
        // Write data to file
        let cid_str = v.cid.to_string();
        fs::write(&path, &v.cbor)
            .context(Error::FailedToWriteObject(cid_str.to_string()))?;
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

    /// Remove from the present repository a list of objects identified by their CID strings
    /// if and only they can ALL be found within a mask of available CIDs provided as a HashMap.
    pub(crate) fn remove_objects_if_available(&self, requested_cids: &Vec<String>, is_available_cid: &HashMap<String, bool>) -> Result<()> {
        // check if all requested CIDs relate to known objects
        let mut paths_to_remove: Vec<PathBuf> = Vec::with_capacity(requested_cids.len());
        for cid_str in requested_cids {
            if !is_available_cid.contains_key(cid_str.as_str()) {
                return Err(Error::UnknownObjectIdentifier(cid_str.clone()).into());
            }
            let cid = Cid::try_from(cid_str.clone())
                .context(Error::UnknownObjectIdentifier(cid_str.clone()))?;
            let path_to_remove = self.root.join(cid_to_path(&cid, &LocalContext::new().unwrap()).unwrap());
            if !path_to_remove.exists() {
                return Err(Error::UnknownObjectIdentifier(cid_str.clone()).into());
            }
            paths_to_remove.push(path_to_remove)
        }
        // remove all requested data objects
        for path_to_remove in paths_to_remove {
            fs::remove_file(&path_to_remove)
                .context(anyhow!("failed to remove file: {}", &path_to_remove.to_string_lossy()))?;
        };
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_multihash_from_blake3_hash() {
        let hash = [0x42 as u8; 32];
        let multihash = blake3_hash_to_multihash(hash.clone()).unwrap();
        assert_eq!(multihash.code(), BLAKE3_HASH_FUNC_TYPE as u64);
        assert_eq!(multihash.size(), 0x20);
        assert_eq!(*multihash.digest(), hash[..]);
    }

    #[test]
    fn test_cid_to_object_path() {
        let cid = Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap();
        let path = cid_to_object_path(&cid).unwrap();
        assert_eq!(path, PathBuf::new().join("3y").join("afir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq"))
    }

    #[test]
    fn test_object_path_to_cid() {
        let path = PathBuf::new().join("3y").join("afir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq");
        let cid = object_path_to_cid(&path).unwrap();
        assert_eq!(cid, Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap())
    }

    #[test]
    fn test_cid_to_object_path_to_cid() {
        let cid = Cid::try_from("bafir4idbvg7rb4h75xd5y52ytlrkwtfibmagzadomy3oig3aiegnr4f3yq").unwrap();
        assert_eq!(
            cid,
            object_path_to_cid(&cid_to_object_path(&cid).unwrap()).unwrap()
        )
    }
}