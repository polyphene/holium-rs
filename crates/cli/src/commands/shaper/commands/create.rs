use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, ObjectAlreadyExistsForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::jsonschema::validate_json_schema;
use crate::utils::local::context::helpers::validate_node_name;
use crate::utils::local::helpers::prints::commands_outputs::print_create_success;
use crate::utils::local::models::shaper::Shaper;
use crate::utils::local::helpers::prints::errors::Error::StructureCreationError;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
            Arg::with_name("json-schema")
                .help("JSON Schema of the node")
                .required(true)
                .takes_value(true)
                .value_name("JSON-SCHEMA")
                .long("json-schema"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let name = matches.value_of("name")
        .context(MissingRequiredArgument("name".to_string()))?;
    let json_schema = matches.value_of("json-schema")
        .context(MissingRequiredArgument("json-schema".to_string()))?;
    // check that the object does not already exist
    if local_context.shapers.contains_key(name).context(DbOperationFailed)? {
        return Err(ObjectAlreadyExistsForGivenKey(name.to_string()).into());
    }
    // validate the node name
    validate_node_name(name)?;
    // validate JSON schema
    validate_json_schema(json_schema)?;
    // create new object
    let object = Shaper {
        name: name.to_string(),
        json_schema: json_schema.to_string(),
    };
    // store new object
    let encoded: Vec<u8> = bincode::serialize(&object)
        .context(BinCodeSerializeFailed)?;
    local_context.shapers
        .compare_and_swap(object.name, None as Option<&[u8]>, Some(encoded))
        .context(DbOperationFailed)?
        .ok()
        .context(StructureCreationError("shaper".to_string(), name.to_string()))?;
    print_create_success(name);
    Ok(())
}
