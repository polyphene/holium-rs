use crate::utils::errors::Error::{
    DbOperationFailed, MissingRequiredArgument, NoObjectForGivenKey,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::prints::commands_outputs::print_delete_success;
use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};
use console::style;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete a portation")
        .args(&[Arg::with_name("id")
            .help("ID of the portation")
            .required(true)
            .value_name("ID")])
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let mut local_context = LocalContext::new()?;
    // get argument values
    let id = matches
        .value_of("id")
        .context(MissingRequiredArgument("id".to_string()))?;
    // delete object
    let old_value = local_context.portations.remove(&id.to_string())?;
    if old_value.is_none() {
        return Err(NoObjectForGivenKey(id.to_string()).into());
    }
    // print
    print_delete_success(id);
    Ok(())
}
