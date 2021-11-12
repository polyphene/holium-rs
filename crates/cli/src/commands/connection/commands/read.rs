use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::models::connection::Connection;
use anyhow::{Context, Error, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("read")
        .about("Read a connection")
        .args(&[Arg::with_name("id")
            .help("ID of the connection")
            .required(true)
            .value_name("ID")])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let id = matches
        .value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    // get object from local database
    let encoded = local_context
        .connections
        .get(id)
        .context(DbOperationFailed)?
        .ok_or(NoObjectForGivenKey(id.to_string()))?;
    let mut decoded: Connection = bincode::deserialize(&encoded[..])
        .ok()
        .context(BinCodeDeserializeFailed)?;
    decoded.id = id.to_string();
    // print
    Connection::table_print(vec![&decoded]);
    Ok(())
}
