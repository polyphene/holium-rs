#[macro_use]
extern crate alloc;
extern crate humansize;
extern crate lazy_static;
extern crate prettytable;

use std::env;

use clap::{crate_authors, crate_version, App, AppSettings};
use console::style;

mod commands;
mod utils;

fn main() {
    // Create CLI matches
    let matches = build_cli().get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("generate-shell-completions", Some(matches)) => commands::completion_script::handle_cmd(matches),
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

pub const BIN_NAME: &str = "holium";

pub fn build_cli() -> App<'static, 'static> {
    App::new("Holium")
        .bin_name(BIN_NAME)
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Enjoy the power of the Holium Framework")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            commands::completion_script::cmd(),
            commands::init::cmd(),
            commands::source::cmd(),
            commands::shaper::cmd(),
            commands::transformation::cmd(),
            commands::connection::cmd(),
            commands::portation::cmd(),
            commands::project::cmd(),
        ])
}