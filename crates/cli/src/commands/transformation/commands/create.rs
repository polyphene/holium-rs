use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, ObjectAlreadyExistsForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::models::transformation::Transformation;
use crate::utils::local::helpers::jsonschema::{validate_transformation_json_schema};
use crate::utils::local::helpers::keys::validate_node_name;
use crate::utils::local::helpers::prints::commands_outputs::print_create_success;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
            Arg::with_name("bytecode")
                .help("Wasm module holding the pure transformation")
                .required(true)
                .takes_value(true)
                .value_name("FILE")
                .short("b")
                .long("bytecode"),
            Arg::with_name("handle")
                .help("Handle of the pure function in the Wasm module")
                .required(true)
                .takes_value(true)
                .value_name("HANDLE")
                .short("h")
                .long("handle"),
            Arg::with_name("json-schema-in")
                .help("JSON Schema of the input parameter")
                .required(true)
                .takes_value(true)
                .value_name("JSON-SCHEMA-IN")
                .long("json-schema-in"),
            Arg::with_name("json-schema-out")
                .help("JSON Schema of the output parameter")
                .required(true)
                .takes_value(true)
                .value_name("JSON-SCHEMA-OUT")
                .long("json-schema-out"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let name = matches.value_of("name")
        .context(MissingRequiredArgument("name".to_string()))?;
    let bytecode_path_os_string = matches.value_of("bytecode")
        .context(MissingRequiredArgument("bytecode".to_string()))?;
    let handle = matches.value_of("handle")
        .context(MissingRequiredArgument("handle".to_string()))?;
    let json_schema_in = matches.value_of("json-schema-in")
        .context(MissingRequiredArgument("json-schema-in".to_string()))?;
    let json_schema_out = matches.value_of("json-schema-out")
        .context(MissingRequiredArgument("json-schema-out".to_string()))?;
    // check that the object does not already exist
    if local_context.transformations.contains_key(name).context(DbOperationFailed)? {
        return Err(ObjectAlreadyExistsForGivenKey(name.to_string()).into());
    }
    // validate the node name
    validate_node_name(name)?;
    // validate the bytecode file path
    let bytecode_path = PathBuf::from(bytecode_path_os_string);
    let bytecode = read_all_wasm_module(&bytecode_path)?;
    // validate JSON schemata
    validate_transformation_json_schema(json_schema_in)?;
    validate_transformation_json_schema(json_schema_out)?;
    // create new object
    let object = Transformation {
        name: name.to_string(),
        bytecode,
        handle: handle.to_string(),
        json_schema_in: json_schema_in.to_string(),
        json_schema_out: json_schema_out.to_string(),
    };
    // store new object
    let encoded: Vec<u8> = bincode::serialize(&object)
        .context(BinCodeSerializeFailed)?;
    local_context.transformations
        .compare_and_swap(object.name, None as Option<&[u8]>, Some(encoded))
        .context(DbOperationFailed)?
        .ok()
        .context(anyhow!("cannot create transformation with name: {}", name))?;
    print_create_success(name);
    Ok(())
}
