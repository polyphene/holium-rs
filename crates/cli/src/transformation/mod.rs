//! CLI command to manage transformations in a Holium repository.

use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};
use thiserror::Error;

use crate::transformation::add::{handle_add_cmd, add_cmd};
use crate::transformation::list::{list_cmd, handle_list_cmd};
use crate::transformation::remove::{remove_cmd, handle_remove_cmd};

mod add;
mod list;
mod remove;

#[derive(Error, Debug)]
/// Errors for transformations operations.
pub(crate) enum TransformationError {
}

/// `transformation` command
pub(crate) fn transformation_cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("transformation")
        .about("Manipulates transformation in a Holium repository")
        .subcommand(add_cmd())
        .subcommand(list_cmd())
        .subcommand(remove_cmd())
}

/// `transformation` command handler
pub(crate) fn handle_cmd(transformation_matches: &ArgMatches) -> Result<()> {
    // Match sub-subcommands
    match transformation_matches.subcommand() {
        ("add", Some(matches)) => handle_add_cmd(matches),
        ("ls", Some(matches)) => handle_list_cmd(matches),
        ("rm", Some(matches)) => handle_remove_cmd(matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    }
}