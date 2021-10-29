use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use std::str::from_utf8;
use crate::utils::local::models::source::Source;
use prettytable::{Table, format};
use crate::utils::local::helpers::prints::printable_model::PrintableModel;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("List all nodes of this type")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // iterate through stored objects
    let objects_result: Result<Vec<Source>> = local_context.sources
        .iter()
        .map(|o| -> Result<Source> {
            let (name_vec, encoded) = o.context(DbOperationFailed)?;
            let name = from_utf8(name_vec.as_ref())?;
            let mut decoded: Source = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.name = name.to_string();
            Ok(decoded)
        })
        .collect();
    let objects = objects_result?;
    // print
    Source::table_print(objects);
    Ok(())
}