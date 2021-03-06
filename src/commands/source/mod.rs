//! Manipulate source nodes.

mod commands;

use anyhow::Result;
use clap::{App, AppSettings, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("source")
        .about("Manipulate source nodes of a pipeline")
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
