use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Error as AnyhowError, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{
    BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument,
    ObjectAlreadyExistsForGivenKey,
};
use crate::utils::local::context::helpers::{build_connection_id, validate_node_name};
use crate::utils::local::context::helpers::{validate_pipeline_node_existence, NodeType};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::jsonschema::validate_pipeline_node_json_schema;
use crate::utils::local::helpers::prints::commands_outputs::print_create_success;
use crate::utils::local::helpers::prints::errors::Error::StructureCreationError;
use crate::utils::local::helpers::selector::validate_selector;
use crate::utils::local::models::connection::Connection;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a connection")
        .args(&[
            Arg::with_name("tail-type")
                .help("Type of the node at the tail of the connection")
                .display_order(1)
                .required(true)
                .takes_value(true)
                .possible_values(&NodeType::variants())
                .case_insensitive(true)
                .value_name("TYPE")
                .long("tail-type"),
            Arg::with_name("tail-name")
                .help("Name of the node at the tail of the connection")
                .display_order(2)
                .required(true)
                .takes_value(true)
                .value_name("NAME")
                .long("tail-name"),
            Arg::with_name("tail-selector")
                .help("Selector at the tail of the connection")
                .display_order(3)
                .required(true)
                .takes_value(true)
                .value_name("JSON-SCHEMA")
                .long("tail-selector"),
            Arg::with_name("head-type")
                .help("Type of the node at the head of the connection")
                .display_order(4)
                .required(true)
                .takes_value(true)
                .possible_values(&NodeType::variants())
                .case_insensitive(true)
                .value_name("TYPE")
                .long("head-type"),
            Arg::with_name("head-name")
                .help("Name of the node at the head of the connection")
                .display_order(5)
                .required(true)
                .takes_value(true)
                .value_name("NAME")
                .long("head-name"),
            Arg::with_name("head-selector")
                .help("Selector at the head of the connection")
                .display_order(6)
                .required(true)
                .takes_value(true)
                .value_name("JSON-SCHEMA")
                .long("head-selector"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let tail_type_str = matches
        .value_of("tail-type")
        .context(MissingRequiredArgument("tail-type".to_string()))?;
    let tail_name = matches
        .value_of("tail-name")
        .context(MissingRequiredArgument("tail-name".to_string()))?;
    let tail_selector = matches
        .value_of("tail-selector")
        .context(MissingRequiredArgument("tail-selector".to_string()))?;
    let head_type_str = matches
        .value_of("head-type")
        .context(MissingRequiredArgument("head-type".to_string()))?;
    let head_name = matches
        .value_of("head-name")
        .context(MissingRequiredArgument("head-name".to_string()))?;
    let head_selector = matches
        .value_of("head-selector")
        .context(MissingRequiredArgument("head-selector".to_string()))?;
    // validate the existence of tail and head nodes
    let tail_type = tail_type_str
        .parse::<NodeType>()
        .map_err(AnyhowError::msg)?;
    let head_type = head_type_str
        .parse::<NodeType>()
        .map_err(AnyhowError::msg)?;
    let tail_typed_name = validate_pipeline_node_existence(&local_context, &tail_type, tail_name)?;
    let head_typed_name = validate_pipeline_node_existence(&local_context, &head_type, head_name)?;
    // create a key for the object
    let id = build_connection_id(&tail_typed_name, &head_typed_name);
    // check that the object does not already exist
    if local_context
        .connections
        .contains_key(&id)
        .context(DbOperationFailed)?
    {
        return Err(ObjectAlreadyExistsForGivenKey(id).into());
    }
    // validate selectors
    validate_selector(tail_selector)?;
    validate_selector(head_selector)?;
    // create new object
    let object = Connection {
        id: id.clone(),
        tail_selector: tail_selector.to_string(),
        head_selector: head_selector.to_string(),
    };
    // store new object
    let encoded: Vec<u8> = bincode::serialize(&object).context(BinCodeSerializeFailed)?;
    local_context
        .connections
        .compare_and_swap(object.id, None as Option<&[u8]>, Some(encoded))
        .context(DbOperationFailed)?
        .ok()
        .context(StructureCreationError("connection".to_string(), id.clone()))?;
    print_create_success(&id);
    Ok(())
}
