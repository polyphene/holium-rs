use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Error as AnyhowError, Result};
use clap::{arg_enum, App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{
    BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument,
    ObjectAlreadyExistsForGivenKey,
};
use crate::utils::local::context::helpers::{
    build_portation_id, validate_node_name, PortationDirectionType,
};
use crate::utils::local::context::helpers::{validate_pipeline_node_existence, NodeType};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::jsonschema::validate_json_schema;
use crate::utils::local::helpers::media_type::validate_mimetype_coherence;
use crate::utils::local::helpers::prints::commands_outputs::print_create_success;
use crate::utils::local::helpers::selector::validate_selector;
use crate::utils::repo::helpers::to_relative_path_to_project_root;
use crate::utils::repo::models::portation::{Portation, PortationFileFormat};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a portation")
        .args(&[
        Arg::with_name("direction")
            .help("Direction of the portation (FROM Holium to the file vs. from the file TO Holium")
            .display_order(1)
            .required(true)
            .takes_value(true)
            .possible_values(&PortationDirectionType::variants())
            .case_insensitive(true)
            .long("direction"),
        Arg::with_name("node-type")
            .help("Type of the node to port from/to")
            .display_order(2)
            .required(true)
            .takes_value(true)
            .possible_values(&NodeType::variants())
            .case_insensitive(true)
            .value_name("TYPE")
            .long("node-type"),
        Arg::with_name("node-name")
            .help("Name of the node to port from/to")
            .display_order(3)
            .required(true)
            .takes_value(true)
            .value_name("NAME")
            .long("node-name"),
        Arg::with_name("file-path")
            .help("Path of the file to port from/to")
            .display_order(4)
            .required(true)
            .takes_value(true)
            .value_name("FILE")
            .long("file-path"),
        Arg::with_name("file-format")
            .help("Format of the file to port from/to")
            .display_order(5)
            .required(true)
            .takes_value(true)
            .possible_values(&PortationFileFormat::variants())
            .case_insensitive(true)
            .value_name("FORMAT")
            .long("file-format"),
    ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let mut local_context = LocalContext::new()?;
    // get argument values
    let direction = matches
        .value_of("direction")
        .context(MissingRequiredArgument("direction".to_string()))?;
    let node_type_str = matches
        .value_of("node-type")
        .context(MissingRequiredArgument("node-type".to_string()))?;
    let node_name = matches
        .value_of("node-name")
        .context(MissingRequiredArgument("node-name".to_string()))?;
    let file_path_os_string = matches
        .value_of("file-path")
        .context(MissingRequiredArgument("file-path".to_string()))?;
    let file_format = matches
        .value_of("file-format")
        .context(MissingRequiredArgument("file-format".to_string()))?;
    // validate the existence of the node
    let node_type = node_type_str
        .parse::<NodeType>()
        .map_err(AnyhowError::msg)?;
    let node_typed_name = validate_pipeline_node_existence(&local_context, &node_type, node_name)?;
    // create a key for the object
    let direction = direction
        .parse::<PortationDirectionType>()
        .map_err(AnyhowError::msg)?;
    let id = build_portation_id(&direction, &node_typed_name);
    // check that the object does not already exist
    if local_context.portations.contains_key(&id.to_string()) {
        return Err(ObjectAlreadyExistsForGivenKey(id.to_string()).into());
    }
    // validate file path
    let file_path = to_relative_path_to_project_root(file_path_os_string)?;
    // parse file format
    let file_format = file_format
        .parse::<PortationFileFormat>()
        .map_err(AnyhowError::msg)?;
    // validate coherence of file format and file name
    validate_mimetype_coherence(&file_path, &file_format)?;
    // create new object
    let object = Portation {
        id: id.clone(),
        file_path,
        file_format,
    };
    // store new object
    local_context.portations.insert(object.id.clone(), object)?;
    print_create_success(&id);
    Ok(())
}
