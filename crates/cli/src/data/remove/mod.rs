use std::collections::HashMap;
use std::convert::TryFrom;
use std::{env, fs};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use cid::Cid;
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};
use thiserror::Error;

use holium::data::data_tree::Node as DataTreeNode;
use holium::data::linked_data_tree::Node as LinkedDataTreeNode;

use crate::utils::PROJECT_DIR;
use crate::utils::storage::{cid_to_object_path, RepoStorage};
use crate::utils::storage::StorageError::FailedToParseCid;


#[derive(Error, Debug)]
/// Errors for data removal operations.
enum DataRemoveError {
    /// Thrown when a provided identifier cannot be linked to a known data object
    #[error("unknown data object identifier: {0}")]
    UnknownDataObjectIdentifier(String),
}

/// `data` `remove` command
pub(crate) fn remove_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("rm")
        .alias("remove")
        .about("Remove holium data objects from a repository")
        .arg(
            Arg::with_name("cid")
                .help("The CID of the data object to remove")
                .required(true)
                .multiple(true)
        )
}

/// `data` `remove` command handler
pub(crate) fn handle_remove_cmd(matches: &ArgMatches) -> Result<()> {
    // build a repository context
    let repo = RepoStorage::from_cur_dir()?;
    // get CIDs of data objects to remove from arguments
    let cid_strings: Vec<String> = matches
        .values_of("cid")
        .context(anyhow!("failed to get CIDs from command arguments"))?
        .map(|v| v.to_lowercase())
        .collect()
        ;
    // check if each CID relates to a valid data object
    let mut is_valid_cid = HashMap::new();
    for valid_cid in repo.data_cids {
        is_valid_cid.insert(valid_cid.to_string().to_lowercase(), true);
    }
    let mut paths_to_remove: Vec<PathBuf> = Vec::with_capacity(cid_strings.len());
    for cid_str in cid_strings {
        if !is_valid_cid.contains_key(cid_str.as_str()) {
            bail!(DataRemoveError::UnknownDataObjectIdentifier(cid_str.clone()))
        }
        let cid = Cid::try_from(cid_str.clone())
            .context(anyhow!(DataRemoveError::UnknownDataObjectIdentifier(cid_str.clone())))?;
        let path_to_remove = repo.root.join(cid_to_object_path(&cid));
        if !path_to_remove.exists() {
            bail!(DataRemoveError::UnknownDataObjectIdentifier(cid_str.clone()))
        }
        paths_to_remove.push(path_to_remove)
    }
    // remove all requested data objects
    for path_to_remove in paths_to_remove {
        fs::remove_file(&path_to_remove)
            .context(anyhow!("failed to remove file: {}", &path_to_remove.to_string_lossy()))?;
    }
    // return
    Ok(())
}
