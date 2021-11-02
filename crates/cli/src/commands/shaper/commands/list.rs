use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use std::str::from_utf8;
use crate::utils::local::models::shaper::Shaper;
use prettytable::{Table, format};
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::context::helpers::db_key_to_str;

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
    let objects_result: Result<Vec<Shaper>> = local_context.shapers
        .iter()
        .map(|o| -> Result<Shaper> {
            let (name_vec, encoded) = o.context(DbOperationFailed)?;
            let name = db_key_to_str(name_vec)?;
            let mut decoded: Shaper = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.name = name;
            Ok(decoded)
        })
        .collect();
    let objects = objects_result?;
    let references: Vec<&Shaper> = objects.iter().collect();
    // print
    Shaper::table_print(references);
    Ok(())
}
