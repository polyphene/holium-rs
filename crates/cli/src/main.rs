#[macro_use]
extern crate humansize;
extern crate lazy_static;
extern crate prettytable;

use std::env;

use clap::{App, AppSettings, crate_authors, crate_version};
use console::style;

mod utils;
mod commands;

fn main() {
    // Create CLI matches
    let matches = App::new("Holium")
        .bin_name("holium")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Enjoy the power of the Holium Framework")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            commands::init::cmd(),
            commands::source::cmd(),
            commands::transformation::cmd(),
        ])
        .get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("init", Some(matches)) => commands::init::handle_cmd(matches),
        ("source", Some(matches)) => commands::source::handle_cmd(matches),
        ("transformation", Some(matches)) => commands::transformation::handle_cmd(matches),
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
