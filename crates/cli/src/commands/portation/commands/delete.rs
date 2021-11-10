use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{MissingRequiredArgument, DbOperationFailed, NoObjectForGivenKey};
use console::style;
use crate::utils::local::helpers::prints::commands_outputs::print_delete_success;
use crate::utils::repo::context::RepositoryContext;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete a portation")
        .args(&[
            Arg::with_name("id")
                .help("ID of the portation")
                .required(true)
                .value_name("ID"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create repository context
    let mut repo_context = RepositoryContext::new()?;
    // get argument values
    let id = matches.value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    // delete object
    let old_value = repo_context.portations.remove(&id.to_string())?;
    if old_value.is_none() {
        return Err(NoObjectForGivenKey(id.to_string()).into());
    }
    // print
    print_delete_success(id);
    Ok(())
}
