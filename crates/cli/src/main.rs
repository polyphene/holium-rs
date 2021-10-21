use std::env;

use clap::{App, AppSettings, Arg, crate_authors, crate_version, SubCommand};

use crate::data::data_cmd;
use crate::transformation::transformation_cmd;
use crate::init::init_cmd;
use crate::config::config_cmd;

mod config;
mod init;
mod utils;
mod data;
mod transformation;

fn main() {
    // Create CLI matches
    let matches = App::new("Holium")
        .bin_name("holium")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Enjoy the power of the Holium Framework.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(init_cmd())
        .subcommand(config_cmd())
        .subcommand(data_cmd())
        .subcommand(transformation_cmd())
        .get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("init", Some(init_matches)) => init::handle_cmd(init_matches),
        ("config", Some(config_matches)) => config::handle_cmd(config_matches),
        ("data", Some(data_matches)) => data::handle_cmd(data_matches),
        ("transformation", Some(transformation_matches)) => transformation::handle_cmd(transformation_matches),
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
