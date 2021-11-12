//! Run commands related to the whole project

mod commands;

use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use console::style;
use thiserror::Error;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("project")
        .about("Run commands related to the whole project")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(commands::export::cmd())
        .subcommand(commands::import::cmd())
        .subcommand(commands::run::cmd())
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("export", Some(matches)) => commands::export::handle_cmd(matches),
        ("import", Some(matches)) => commands::import::handle_cmd(matches),
        ("run", Some(matches)) => commands::run::handle_cmd(matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    }
}
