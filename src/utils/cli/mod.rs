use crate::commands;
use clap::{crate_authors, crate_version, App, AppSettings};

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
