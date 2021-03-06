use crate::utils::interplanetary::context::InterplanetaryContext;
use crate::utils::interplanetary::fs::helpers::clear_ip_area::clear_ip_area;
use crate::utils::local::context::LocalContext;
use crate::utils::local::dag::models::PipelineDag;
use crate::utils::local::export::export_project;
use crate::utils::local::helpers::prints::commands_outputs::{
    print_local_pipeline_health_success, print_project_export_success,
};

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("export")
        .about("Export the current local project to the interplanetary area")
        .arg(
            Arg::with_name("no-write")
                .help("Check ability to export without writing to the interplanetary area")
                .long("no-write"),
        )
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local and interplanetary contexts
    let local_context = LocalContext::new()?;
    let ip_context = InterplanetaryContext::new()?;
    // create pipeline dag
    let dag = PipelineDag::from_local_context(&local_context)?;
    // check if the dag is healthy for export
    dag.is_valid_pipeline()?;
    // with the --no-write option, stop the execution there
    if matches.is_present("no-write") {
        print_local_pipeline_health_success();
        return Ok(());
    }
    // clean the interplanetary area
    clear_ip_area(&ip_context)?;
    // export pipeline from the local area to the interplanetary area
    let pipeline_cid = export_project(&local_context, &ip_context)?;
    // print success message
    print_project_export_success(&pipeline_cid);
    Ok(())
}
