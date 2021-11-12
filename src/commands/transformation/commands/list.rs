use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use crate::utils::local::context::helpers::db_key_to_str;
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::models::transformation::Transformation;
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
    let objects_result: Result<Vec<Transformation>> = local_context
        .transformations
        .iter()
        .map(|o| -> Result<Transformation> {
            let (name_vec, encoded) = o.context(DbOperationFailed)?;
            let name = db_key_to_str(name_vec)?;
            let mut decoded: Transformation = bincode::deserialize(&encoded[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded.name = name;
            Ok(decoded)
        })
        .collect();
    let objects = objects_result?;
    let references: Vec<&Transformation> = objects.iter().collect();
    // print
    Transformation::table_print(references);
    Ok(())
}
