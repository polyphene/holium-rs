use std::env;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

mod config;
mod repo;
mod utils;

fn main() {
    // Create CLI matches
    let matches = App::new("Holium")
        .bin_name("holium")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Enjoy the power of the Holium Framework.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes a repository of Holium objects")
                .args(&[
                    Arg::with_name("no-scm")
                        .help("Initiate Holium in directory that is not tracked by any SCM tool.")
                        .long("no-scm"),
                    Arg::with_name("no-dvc")
                        .help("Initiate Holium in directory that is not tracked by any DVC tool.")
                        .long("no-dvc"),
                    Arg::with_name("force")
                        .help("Overwrites existing Holium project")
                        .short("f")
                        .long("force"),
                ]),
        )
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
                        .conflicts_with_all(&["project", "local"]),
                    Arg::with_name("unset")
                        .help("Unset option.")
                        .short("u")
                        .long("unset")
                        .conflicts_with("value"),
                ]),
        )
        .get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("init", Some(init_matches)) => repo::handle_cmd(init_matches),
        ("config", Some(config_matches)) => config::handle_cmd(config_matches),
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
