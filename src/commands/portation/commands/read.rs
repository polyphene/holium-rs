use crate::utils::errors::Error::{MissingRequiredArgument, NoObjectForGivenKey};

use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::repo::context::RepositoryContext;
use crate::utils::repo::models::portation::Portation;
use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("read")
        .about("Read a portation")
        .args(&[Arg::with_name("id")
            .help("ID of the portation")
            .required(true)
            .value_name("ID")])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create repository context
    let repo_context = RepositoryContext::new()?;
    // get argument values
    let id = matches
        .value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    // get object
    let object = repo_context
        .portations
        .get(&id.to_string())
        .ok_or(NoObjectForGivenKey(id.to_string()))?;
    // print
    Portation::table_print(vec![object]);
    Ok(())
}
