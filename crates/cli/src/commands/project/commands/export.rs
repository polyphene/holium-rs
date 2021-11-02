use anyhow::{Result, Context};
use clap::{App, SubCommand, Arg, ArgMatches};
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use std::str::from_utf8;
use crate::utils::local::models::shaper::Shaper;
use prettytable::{Table, format};
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::dag::models::PipelineDag;
use crate::utils::local::helpers::prints::commands_outputs::print_pipeline_health_success;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("export")
        .about("Export the current local project to the interplanetary area")
        .arg(
            Arg::with_name("no-write")
                .help("Check ability to export without writing to the interplanetary area")
                .long("no-write")
        )
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // create pipeline dag
    let dag = PipelineDag::from_local_context(&local_context)?;
    // check if the dag is healthy for export
    dag.is_valid_pipeline()?;
    // with the --no-write option, stop the execution there
    if matches.is_present("no-write") {
        print_pipeline_health_success(&mut std::io::stdout());
        return Ok(());
    }
    todo!();
    Ok(())
}
