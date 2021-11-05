use std::path::PathBuf;

use anyhow::{Context, Result, Error as AnyhowError};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::repo::models::portation::{Portation, PortationFileFormat};
use crate::utils::local::helpers::prints::commands_outputs::print_update_success;
use crate::utils::local::helpers::selector::validate_selector;
use crate::utils::repo::helpers::to_relative_path_to_project_root;
use crate::utils::repo::context::RepositoryContext;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Update a portation")
        .args(&[
            Arg::with_name("id")
                .help("ID of the portation")
                .required(true)
                .value_name("ID"),
            Arg::with_name("file-path")
                .help("Path of the file to port from/to")
                .display_order(1)
                .takes_value(true)
                .value_name("FILE")
                .long("file-path"),
            Arg::with_name("file-format")
                .help("Format of the file to port from/to")
                .display_order(2)
                .takes_value(true)
                .possible_values(&PortationFileFormat::variants())
                .case_insensitive(true)
                .value_name("FORMAT")
                .long("file-format"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create repository context
    let mut repo_context = RepositoryContext::new()?;
    // get argument values
    let id = matches.value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    let file_path_os_string = matches.value_of("file-path");
    let file_format_str = matches.value_of("file-format");
    // check that the object exists
    let old_value = repo_context.portations
        .get(&id.to_string())
        .ok_or(NoObjectForGivenKey(id.to_string()))?;
    // validate file path, if any
    let file_path = if let Some(s) = file_path_os_string {
        Some(to_relative_path_to_project_root(s)?)
    } else { None };
    // parse file format, if any
    let file_format = if let Some(s) = file_format_str {
        Some(s.parse::<PortationFileFormat>().map_err(AnyhowError::msg)?)
    } else { None };
    // merge objects
    let object = Portation {
        id: (*old_value.id).to_string(),
        file_path: file_path.unwrap_or((*old_value.file_path).to_string()),
        file_format: file_format.unwrap_or(old_value.file_format.clone()),
    };
    // store object
    repo_context.portations
        .insert(object.id.clone(), object)?;
    print_update_success(id);
    Ok(())
}
