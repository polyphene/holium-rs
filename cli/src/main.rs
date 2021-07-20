mod repo;

use clap::{App, SubCommand, Arg, AppSettings, crate_authors, crate_version};
use std::env;

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
                    Arg::with_name("force")
                        .help("Overwrites existing Holium project")
                        .short("f")
                        .long("force")
                ]),
        )
        .get_matches();

    // Match subcommands
    let exec_res = match matches.subcommand() {
        ("init", Some(init_matches)) => {
            // Get path to current directory
            let cur_dir = env::current_dir().unwrap();
            // Initialize a Holium repository in current directory
            repo::init(
                &cur_dir,
                false,
                false,
                init_matches.is_present("force"),
            )
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    };

    // Use execution result
    match exec_res {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}