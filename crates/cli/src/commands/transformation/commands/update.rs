use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{MissingRequiredArgument, BinCodeSerializeFailed, DbOperationFailed, NoObjectForGivenKey};
use crate::utils::local::trees::transformation::{Transformation, OptionalTransformation};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Update a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
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
    let handle = matches.value_of("handle");
    // check that the object exists
    if !local_context.transformations.contains_key(name).context(DbOperationFailed)? {
        return Err(NoObjectForGivenKey(name.to_string()).into());
    }
    // merge object
    let merge_transformation = OptionalTransformation { handle: handle.map(|s| s.to_string()) };
    let merge_transformation_encoded = bincode::serialize(&merge_transformation)
        .context(BinCodeSerializeFailed)?;
    local_context.transformations.merge(name, merge_transformation_encoded)
        .context(DbOperationFailed)?;
    Ok(())
}
