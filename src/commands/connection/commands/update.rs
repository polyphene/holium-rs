use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::utils::errors::Error::{
    BinCodeSerializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey,
};
use crate::utils::local::context::LocalContext;

use crate::utils::local::helpers::prints::commands_outputs::print_update_success;
use crate::utils::local::helpers::selector::validate_selector;
use crate::utils::local::models::connection::OptionalConnection;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Update a connection")
        .args(&[
            Arg::with_name("id")
                .help("ID of the connection")
                .required(true)
                .value_name("ID"),
            Arg::with_name("tail-selector")
                .help("Selector at the tail of the connection")
                .display_order(1)
                .takes_value(true)
                .value_name("JSON-SCHEMA")
                .long("tail-selector"),
            Arg::with_name("head-selector")
                .help("Selector at the head of the connection")
                .display_order(2)
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
    let id = matches
        .value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    let tail_selector = matches.value_of("tail-selector");
    let head_selector = matches.value_of("head-selector");
    // check that the object exists
    if !local_context
        .connections
        .contains_key(id)
        .context(DbOperationFailed)?
    {
        return Err(NoObjectForGivenKey(id.to_string()).into());
    }
    // validate selectors, if any
    if let Some(tail_selector) = tail_selector {
        validate_selector(tail_selector)?;
    }
    if let Some(head_selector) = head_selector {
        validate_selector(head_selector)?;
    }
    // merge object
    let merge_connection = OptionalConnection {
        id: None,
        tail_selector: tail_selector.map(|s| s.to_string()),
        head_selector: head_selector.map(|s| s.to_string()),
    };
    let merge_connection_encoded =
        bincode::serialize(&merge_connection).context(BinCodeSerializeFailed)?;
    local_context
        .connections
        .merge(id, merge_connection_encoded)
        .context(DbOperationFailed)?;
    print_update_success(id);
    Ok(())
}
