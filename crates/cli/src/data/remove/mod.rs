use std::collections::HashMap;
use std::convert::TryFrom;
use std::{env, fs};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use cid::Cid;
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};

use holium::data::data_tree::Node as DataTreeNode;
use holium::data::linked_data_tree::Node as LinkedDataTreeNode;

use crate::utils::PROJECT_DIR;
use crate::utils::storage::{cid_to_object_path, RepoStorage};
use crate::utils::storage::StorageError::FailedToParseCid;

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
        .collect();
    // create a map of available CIDs
    let mut is_valid_cid = HashMap::new();
    for valid_cid in &repo.data_cids {
        is_valid_cid.insert(valid_cid.to_string().to_lowercase(), true);
    }
    // check if each CID relates to a valid data object
    // and remove all requested data objects
    repo.remove_objects_if_available(&cid_strings, &is_valid_cid)?;
    // return
    Ok(())
}
