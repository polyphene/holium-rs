use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use std::str::from_utf8;
use crate::utils::repo::models::portation::Portation;
use prettytable::{Table, format};
use crate::utils::local::helpers::prints::printable_model::PrintableModel;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("List all portations")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // iterate through stored objects
    let objects: Vec<&Portation> = local_context.portations
        .values()
        .collect();
    // print
    // Portation::table_print(objects); // TODO
    Ok(())
}
