use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::repo::context::RepositoryContext;
use crate::utils::repo::models::portation::Portation;
use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List all portations")
}

/// handler
pub(crate) fn handle_cmd(_matches: &ArgMatches) -> Result<()> {
    // create repository context
    let repo_context = RepositoryContext::new()?;
    // iterate through stored objects
    let objects: Vec<&Portation> = repo_context.portations.values().collect();
    // print
    Portation::table_print(objects);
    Ok(())
}
