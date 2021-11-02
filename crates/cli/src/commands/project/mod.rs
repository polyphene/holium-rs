//! Run commands related to the whole project

mod commands;

use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};
use console::style;
use thiserror::Error;


/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("project")
        .about("Run commands related to the whole project")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(commands::export::cmd())
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("export", Some(matches)) => commands::export::handle_cmd(matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    }
}