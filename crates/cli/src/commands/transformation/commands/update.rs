use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::prints::print_update_success;
use crate::utils::local::models::transformation::{OptionalTransformation, Transformation};

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
    // check that the object exists
    if !local_context.transformations.contains_key(name).context(DbOperationFailed)? {
        return Err(NoObjectForGivenKey(name.to_string()).into());
    }
    // validate the bytecode file path, if any
    let bytecode = match bytecode_path_os_string {
        Some(path_os_string) => {
            let bytecode_path = PathBuf::from(path_os_string);
            Some(read_all_wasm_module(&bytecode_path)?)
        }
        None => None,
    };
    // merge object
    let merge_transformation = OptionalTransformation {
        name: None,
        bytecode,
        handle: handle.map(|s| s.to_string()),
    };
    let merge_transformation_encoded = bincode::serialize(&merge_transformation)
        .context(BinCodeSerializeFailed)?;
    local_context.transformations.merge(name, merge_transformation_encoded)
        .context(DbOperationFailed)?;
    print_update_success(name);
    Ok(())
}
