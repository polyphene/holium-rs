use crate::utils::interplanetary::context::InterplanetaryContext;
use crate::utils::local::context::LocalContext;
use crate::utils::local::dag::models::PipelineDag;
use crate::utils::local::helpers::prints::commands_outputs::{
    print_interplanetary_health_success, print_project_import_success,
};

use crate::utils::local::import::import_project;

use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("import")
        .about("Replace current local project with the content of the interplanetary area")
        .arg(
            Arg::with_name("no-write")
                .help("Check ability to import without replacing current local project")
                .long("no-write"),
        )
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create an interplanetary context and a temporary local one
    let (tmp_local_context, tmp_dir) = LocalContext::new_tmp()?;
    let ip_context = InterplanetaryContext::new()?;
    // import blocks from the interplanetary area into the temporary local area
    import_project(&ip_context, &tmp_local_context)?;
    // check validity of the temp local project
    let dag = PipelineDag::from_local_context(&tmp_local_context)?;
    dag.is_valid_pipeline()?;
    // with the --no-write option, stop the execution there
    if matches.is_present("no-write") {
        print_interplanetary_health_success();
        return Ok(());
    }
    // move the imported content to the project local area
    let local_context = LocalContext::new()?;
    tmp_local_context.mv_local_area(&local_context)?;
    // close temporary local context directory
    tmp_dir.close()?;
    // print success message
    print_project_import_success();
    Ok(())
}
