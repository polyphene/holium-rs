use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{MissingRequiredArgument, DbOperationFailed, NoObjectForGivenKey};
use console::style;
use crate::utils::local::helpers::prints::commands_outputs::print_delete_success;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete a node")
        .args(&[
            Arg::with_name("name")
                .help("Name of the node")
                .required(true)
                .value_name("NAME"),
        ])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // get argument values
    let name = matches.value_of("name")
        .context(MissingRequiredArgument("name".to_string()))?;
    // delete object from local database
    let old_value = local_context.transformations.remove(name)
        .context(DbOperationFailed)?;
    if old_value.is_none() {
        return Err(NoObjectForGivenKey(name.to_string()).into());
    }
    // print
    print_delete_success(name);
    Ok(())
}
