use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use std::str::from_utf8;
use crate::utils::repo::models::portation::Portation;
use prettytable::{Table, format};
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::repo::context::RepositoryContext;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("List all portations")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create repository context
    let mut repo_context = RepositoryContext::new()?;
    // iterate through stored objects
    let objects: Vec<&Portation> = repo_context.portations
        .values()
        .collect();
    // print
    Portation::table_print(objects);
    Ok(())
}
