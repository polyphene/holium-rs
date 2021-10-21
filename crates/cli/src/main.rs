use std::env;

use clap::{App, AppSettings, Arg, crate_authors, crate_version, SubCommand};

use crate::data::data_cmd;
use crate::transformation::transformation_cmd;
use crate::init::init_cmd;

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
        .subcommand(
            SubCommand::with_name("config")
                .about("Manages the persistent configuration of Holium repositories")
                .args(&[
                    Arg::with_name("name")
                        .help("Option name.")
                        .index(1)
                        .required(true),
                    Arg::with_name("value").help("Option new value.").index(2),
                    Arg::with_name("global")
                        .help("Use global configuration.")
                        .long("global")
                        .conflicts_with_all(&["project", "local"]),
                    Arg::with_name("project")
                        .help(&*format!(
                            "Use project configuration ({}/{}).",
                            utils::PROJECT_DIR,
                            utils::CONFIG_FILE
                        ))
                        .long("project")
                        .conflicts_with_all(&["global", "local"]),
                    Arg::with_name("local")
                        .help(&*format!(
                            "Use local configuration ({}/{}).",
                            utils::PROJECT_DIR,
                            utils::LOCAL_CONFIG_FILE
                        ))
                        .long("local")
                        .conflicts_with_all(&["project", "global"]),
                    Arg::with_name("unset")
                        .help("Unset option.")
                        .short("u")
                        .long("unset")
                        .conflicts_with("value"),
                ]),
        )
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
