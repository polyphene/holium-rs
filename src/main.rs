//! This crate provides a CLI offering a simple implementation of the [Holium](https://holium.org/) protocol.
//!
//! Check out the [official documentation](https://docs.holium.org/) for more information.

#[macro_use]
extern crate alloc;
extern crate humansize;
extern crate lazy_static;
extern crate prettytable;

use std::env;

use crate::utils::cli::build_cli;
use clap::{App, AppSettings};
use console::style;

mod commands;
mod utils;

fn main() {
    // Create CLI matches
    let matches = build_cli().get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("generate-shell-completions", Some(matches)) => {
            commands::completion_script::handle_cmd(matches)
        }
        ("init", Some(matches)) => commands::init::handle_cmd(matches),
        ("source", Some(matches)) => commands::source::handle_cmd(matches),
        ("shaper", Some(matches)) => commands::shaper::handle_cmd(matches),
        ("transformation", Some(matches)) => commands::transformation::handle_cmd(matches),
        ("connection", Some(matches)) => commands::connection::handle_cmd(matches),
        ("portation", Some(matches)) => commands::portation::handle_cmd(matches),
        ("project", Some(matches)) => commands::project::handle_cmd(matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    };

    // Use execution result
    std::process::exit(match exec_res {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", style(err).red());
            1
        }
    })
}
