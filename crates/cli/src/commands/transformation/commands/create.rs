use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, ObjectAlreadyExistsForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::helpers::prints::print_create_success;
use crate::utils::local::models::transformation::Transformation;

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
    // check that the object does not already exist
    if local_context.transformations.contains_key(name).context(DbOperationFailed)? {
        return Err(ObjectAlreadyExistsForGivenKey(name.to_string()).into());
    }
    // validate the bytecode file path
    let bytecode_path = PathBuf::from(bytecode_path_os_string);
    let bytecode = read_all_wasm_module(&bytecode_path)?;
    // create new object
    let object = Transformation {
        name: name.to_string(),
        bytecode,
        handle: handle.to_string(),
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