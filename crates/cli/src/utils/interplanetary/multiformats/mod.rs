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