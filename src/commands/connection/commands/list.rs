use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use crate::utils::local::context::helpers::db_key_to_str;
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::models::connection::Connection;
use anyhow::{Context, Result};
use clap::{App, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List all connections")
}

/// handler
pub(crate) fn handle_cmd(_matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // iterate through stored objects
    let objects_result: Result<Vec<Connection>> = local_context
        .connections
        .iter()
        .map(|o| -> Result<Connection> {
            let (id_vec, encoded) = o.context(DbOperationFailed)?;
            let id = db_key_to_str(id_vec)?;
            let mut decoded: Connection = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.id = id.to_string();
            Ok(decoded)
        })
        .collect();
    let objects = objects_result?;
    let references: Vec<&Connection> = objects.iter().collect();
    // print
    Connection::table_print(references);
    Ok(())
}
