use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{MissingRequiredArgument, DbOperationFailed};
use console::style;

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
        println!("{}", style(format!("No object found with name: {}", name)).yellow())
    }
    // print
    println!("ok TODO");
    Ok(())
}
