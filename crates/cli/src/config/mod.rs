//! CLI command to manage a project configuration.

use std::env;

use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::config::models::{ConfigLevel, ProjectConfigFragment};
use crate::utils::PROJECT_DIR;

mod config;
pub(crate) mod models;
mod shadow_merge;
mod sparse_config;
mod updatable_field;

/// Parses arguments and handles the command.
pub(crate) fn handle_cmd(config_matches: &ArgMatches) -> Result<()> {
    // Get path to current directory
    let cur_dir = env::current_dir().unwrap();
    let holium_dir = cur_dir.join(PROJECT_DIR);
    // Get selected level
    let level = if config_matches.is_present("global") {
        ConfigLevel::Global
    } else if config_matches.is_present("local") {
        ConfigLevel::Local
    } else {
        ConfigLevel::Repo
    };
    // Get related config
    let mut config = ProjectConfigFragment::new(level, Some(holium_dir))
        .context("failed to read config file")?;
    // Get option name
    let name = config_matches
        .value_of("name")
        .context("failed to get option name")?;
    // Redirect to the right handler to get, set, or unset the property
    let value = {
        if config_matches.is_present("value") {
            // Set new value
            let new_value_str = config_matches
                .value_of("value")
                .context("failed to get option value")?;
            let new_value = parse_toml_single_value(new_value_str)?;
            let result = config.config.set(name, new_value);
            // Save to file
            config.config.save_to_config_file(&config.path)?;
            // Return
            result
        } else if config_matches.is_present("unset") {
            // Unset value
            let result = config.config.unset(name);
            // Save to file
            config.config.save_to_config_file(&config.path)?;
            // Return
            result
        } else {
            // Get
            config.config.get(name)
        }
    }?;
    // Print value if any
    if let Some(v) = value {
        println!("{}", v)
    }
    // Return
    Ok(())
}

/// Parses a string representing a TOML value.
/// The default parser may have difficulties parsing a TOML value when alone, thus this wrapper.
fn parse_toml_single_value(value_str: &str) -> Result<toml::Value> {
    // define the wrapping structure
    use serde_derive::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    struct S {
        f: toml::Value
    }
    // parse the artificial object
    let s: S = toml::from_str(format!("f = {}", value_str).as_str())
        .with_context(|| format!("failed to parse TOML value : {}", value_str))?;
    Ok(s.f)
}
