use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use crate::utils::local::context::helpers::db_key_to_str;
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::models::source::Source;
use anyhow::{Context, Result};
use clap::{App, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List all nodes of this type")
}

/// handler
pub(crate) fn handle_cmd(_matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // iterate through stored objects
    let objects_result: Result<Vec<Source>> = local_context
        .sources
        .iter()
        .map(|o| -> Result<Source> {
            let (name_vec, encoded) = o.context(DbOperationFailed)?;
            let name = db_key_to_str(name_vec)?;
            let mut decoded: Source = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.name = name;
            Ok(decoded)
        })
        .collect();
    let objects = objects_result?;
    let references: Vec<&Source> = objects.iter().collect();
    // print
    Source::table_print(references);
    Ok(())
}
