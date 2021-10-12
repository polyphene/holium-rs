//! CLI command to manage data from, to and inside a Holium repository.

use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};
use thiserror::Error;

use crate::data::import::{handle_import_cmd, import_cmd};
use crate::data::list::{handle_list_cmd, list_cmd};
use crate::data::remove::{handle_remove_cmd, remove_cmd};

pub(crate) mod import;
mod list;
mod remove;

#[derive(Error, Debug)]
/// Errors for data operations.
pub(crate) enum DataError {
    /// Thrown when the user specifies a file type that fails to be parsed
    #[error("unknown file type : {0}")]
    InvalidImportFileTypeOptionValue(String),
    /// Thrown when failing to open a file in order to import it
    #[error("failed to open file requested for import")]
    FailedToOpenImportFile,
    /// Thrown when failing to open and read a file in order to import it
    #[error("failed to read file requested for import")]
    FailedToReadImportFile,
    /// Thrown when failing to parse a file for data import
    #[error("failed to parse file requested for import")]
    FailedToParseImportFile,
}

/// `data` command
pub(crate) fn data_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("data")
        .about("Manipulates data from, to and inside a Holium repository")
        .subcommand(import_cmd())
        .subcommand(list_cmd())
        .subcommand(remove_cmd())
}

/// `data` command handler
pub(crate) fn handle_cmd(data_matches: &ArgMatches) -> Result<()> {
    // Match sub-subcommands
    match data_matches.subcommand() {
        ("import", Some(matches)) => handle_import_cmd(matches),
        ("ls", Some(matches)) => handle_list_cmd(matches),
        ("rm", Some(matches)) => handle_remove_cmd(matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    }
}
