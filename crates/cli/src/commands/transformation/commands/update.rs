use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::prints::print_update_success;
use crate::utils::local::models::transformation::{OptionalTransformation, Transformation};
use crate::utils::local::helpers::jsonschema::validate_json_schema;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Update a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
            Arg::with_name("bytecode")
                .help("Wasm module holding the pure transformation")
                .takes_value(true)
                .value_name("FILE")
                .short("b")
                .long("bytecode"),
            Arg::with_name("handle")
                .help("Handle of the pure function in the Wasm module")
                .takes_value(true)
                .value_name("HANDLE")
                .short("h")
                .long("handle"),
            Arg::with_name("json-schema-in")
                .help("JSON Schema of the input parameter")
                .takes_value(true)
                .value_name("JSON-SCHEMA-IN")
                .short("i")
                .long("json-schema-in"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let name = matches.value_of("name")
        .context(MissingRequiredArgument("name".to_string()))?;
    let bytecode_path_os_string = matches.value_of("bytecode");
    let handle = matches.value_of("handle");
    let json_schema_in = matches.value_of("json-schema-in");
    // check that the object exists
    if !local_context.transformations.contains_key(name).context(DbOperationFailed)? {
        return Err(NoObjectForGivenKey(name.to_string()).into());
    }
    // validate the bytecode file path, if any
    let bytecode = bytecode_path_os_string.map(|path_os_string| {
        let bytecode_path = PathBuf::from(path_os_string);
        read_all_wasm_module(&bytecode_path)
    }).transpose()?;
    // validate JSON schemata, if any
    if let Some(json_schema_in) = json_schema_in {
        validate_json_schema(json_schema_in)?;
    }
    // merge object
    let merge_transformation = OptionalTransformation {
        name: None,
        bytecode,
        handle: handle.map(|s| s.to_string()),
        json_schema_in: json_schema_in.map(|s| s.to_string()),
    };
    let merge_transformation_encoded = bincode::serialize(&merge_transformation)
        .context(BinCodeSerializeFailed)?;
    local_context.transformations.merge(name, merge_transformation_encoded)
        .context(DbOperationFailed)?;
    print_update_success(name);
    Ok(())
}
