use anyhow::{anyhow, Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{MissingRequiredArgument, BinCodeSerializeFailed, DbOperationFailed, ObjectAlreadyExistsForGivenKey};
use crate::utils::local::models::transformation::Transformation;
use crate::utils::local::helpers::prints::{print_create_success, print_duplicate_key_warning};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("create")
        .about("Create a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
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
    let handle = matches.value_of("handle")
        .context(MissingRequiredArgument("handle".to_string()))?;
    // check that the object does not already exist
    if local_context.transformations.contains_key(name).context(DbOperationFailed)? {
        return Err(ObjectAlreadyExistsForGivenKey(name.to_string()).into());
    }
    // create new object
    let object = Transformation {
        name: name.to_string(),
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
