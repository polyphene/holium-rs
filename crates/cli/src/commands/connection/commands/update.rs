use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::bytecode::read_all_wasm_module;
use crate::utils::local::models::connection::{OptionalConnection, Connection};
use crate::utils::local::helpers::jsonschema::validate_json_schema;
use crate::utils::local::helpers::prints::commands_outputs::print_update_success;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Update a connection")
        .args(&[
            Arg::with_name("id")
                .help("ID of the connection")
                .required(true)
                .value_name("ID"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let id = matches.value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    // check that the object exists
    if !local_context.connections.contains_key(id).context(DbOperationFailed)? {
        return Err(NoObjectForGivenKey(id.to_string()).into());
    }
    // merge object
    let merge_connection = OptionalConnection {
        id: None,
    };
    let merge_connection_encoded = bincode::serialize(&merge_connection)
        .context(BinCodeSerializeFailed)?;
    local_context.connections.merge(id, merge_connection_encoded)
        .context(DbOperationFailed)?;
    print_update_success(id);
    Ok(())
}
