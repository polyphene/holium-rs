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
    Ok(())
}
