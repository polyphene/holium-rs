use std::env;

use clap::{App, AppSettings, crate_authors, crate_version};

mod utils;
mod commands;

fn main() {
    // Create CLI matches
    let matches = App::new("Holium")
        .bin_name("holium")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Enjoy the power of the Holium Framework.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(commands::init::cmd())
        .get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("init", Some(init_matches)) => commands::init::handle_cmd(init_matches),
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    };

    // Use execution result
    std::process::exit(match exec_res {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    })
}
