use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};

use holium::data::data_tree::Node as DataTreeNode;
use holium::data::linked_data_tree::Node as LinkedDataTreeNode;

use crate::data::DataError;
use crate::utils::PROJECT_DIR;
use crate::utils::repo::current_dir_is_valid_repo;

/// `data` `list` command
pub(crate) fn list_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("ls")
        .alias("list")
        .about("List holium data objects in a repository")
}

/// `data` `list` command handler
pub(crate) fn handle_list_cmd(matches: &ArgMatches) -> Result<()> {
    // check that the command is run inside a Holium repository
    current_dir_is_valid_repo()?;
    // return
    Ok(())
}
