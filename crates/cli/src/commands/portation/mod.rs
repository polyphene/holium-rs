//! Manipulate portation objects.

mod commands;

use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};
use console::style;
use thiserror::Error;

use crate::utils::repo::constants::{HOLIUM_DIR, LOCAL_DIR, INTERPLANETARY_DIR, PORTATIONS_FILE};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("portation")
        .about("Manipulate portations from and to nodes of a pipeline")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(commands::create::cmd())
        .subcommand(commands::read::cmd())
        .subcommand(commands::update::cmd())
        .subcommand(commands::delete::cmd())
        .subcommand(commands::list::cmd())
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("create", Some(matches)) => commands::create::handle_cmd(matches),
        ("read", Some(matches)) => commands::read::handle_cmd(matches),
        ("update", Some(matches)) => commands::update::handle_cmd(matches),
        ("delete", Some(matches)) => commands::delete::handle_cmd(matches),
        ("list", Some(matches)) => commands::list::handle_cmd(matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    }
}