use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};

use crate::transformation::TransformationError;
use crate::utils::PROJECT_DIR;
use crate::utils::storage::RepoStorage;

/// `transformation` `list` command
pub(crate) fn list_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("ls")
        .alias("list")
        .about("List transformation objects in a repository")
}

/// `transformation` `list` command handler
pub(crate) fn handle_list_cmd(matches: &ArgMatches) -> Result<()> {
    // build a repository context
    let repo = RepoStorage::from_cur_dir()?;
    // print list of transformations' CIDs
    let mut cids: Vec<String> = repo.transformation_cids.iter().map(|cid| cid.to_string()).collect();
    cids.sort_unstable();
    cids.iter().for_each(|cid| println!("{}", cid));
    // return
    Ok(())
}
