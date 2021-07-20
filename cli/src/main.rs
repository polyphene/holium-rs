mod repo;

use clap::{App, SubCommand, Arg, AppSettings, crate_authors, crate_version};
use std::env;

fn main() {
    // Get environment variables
    let cur_dir_path = env::current_dir().unwrap();
    let cur_dir = cur_dir_path.to_str().unwrap();

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
                .arg(
                    Arg::with_name("root_dir")
                        .help("Path to the repository's root directory")
                        .takes_value(true)
                        .required(true)
                        .default_value(cur_dir)
                ),
        )
        .get_matches();

    // Match subcommands
    match matches.subcommand() {
        ("init", Some(init_matches)) => {
            repo::init(
                init_matches.value_of("root_dir").unwrap(),
                false,
                false,
                false,
            ).unwrap();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else should be unreachable!()
    }
}