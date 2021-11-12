//! Generate tab-completion scripts for different types of shells.

use crate::{build_cli, BIN_NAME};
use anyhow::Result;
use clap::value_t;
use clap::Shell;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::io;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("generate-shell-completions")
        .alias("gsc")
        .about("Prints the completion script dedicated to the CLI for a given shell")
        .setting(AppSettings::ArgRequiredElseHelp)
        .args(&[Arg::with_name("shell")
            .help("Type of shell to generate the completion script for.")
            .required(true)
            .takes_value(true)
            .possible_values(&Shell::variants())
            .case_insensitive(true)
            .value_name("SHELL")])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    if let Ok(generator) = value_t!(matches, "shell", Shell) {
        let mut app = build_cli();
        app.gen_completions_to(BIN_NAME, generator, &mut io::stdout());
    }
    Ok(())
}
