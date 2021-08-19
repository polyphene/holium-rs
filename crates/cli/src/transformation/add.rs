use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap::{App, Arg, arg_enum, ArgMatches, SubCommand, value_t};

use crate::transformation::TransformationError;
use crate::utils::PROJECT_DIR;
use crate::utils::storage::RepoStorage;

/// `transformation` `add` command
pub(crate) fn add_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("add a transformation")
}

/// `transformation` `add` command handler
pub(crate) fn handle_add_cmd(matches: &ArgMatches) -> Result<()> {
    Ok(())
}