use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};

use holium::data::data_tree::Node as DataTreeNode;
use holium::data::linked_data_tree::Node as LinkedDataTreeNode;

use crate::data::DataError;
use crate::utils::PROJECT_DIR;
use crate::utils::storage::RepoStorage;

/// `data` `remove` command
pub(crate) fn remove_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("rm")
        .alias("remove")
        .about("Remove holium data objects from a repository")
}

/// `data` `remove` command handler
pub(crate) fn handle_remove_cmd(matches: &ArgMatches) -> Result<()> {
    // return
    Ok(())
}
