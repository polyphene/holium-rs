//! Run a transformation pipeline

use crate::utils::local::context::LocalContext;
use crate::utils::local::dag::models::PipelineDag;
use crate::utils::local::helpers::prints::commands_outputs::{
    print_pipeline_export_success, print_pipeline_run_success,
};

use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

use crate::utils::repo::context::RepositoryContext;
use crate::utils::run::runtime::Runtime;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run").about("Run local transformation pipeline if it is valid")
}

/// handler
pub(crate) fn handle_cmd(_matches: &ArgMatches) -> Result<()> {
    // create contexts
    let local_context = LocalContext::new()?;
    let repo_context = RepositoryContext::new()?;
    // create runtime
    let mut runtime = Runtime::new()?;

    // Run Pipeline dag from local context
    let node_exports = PipelineDag::run(&mut runtime, &local_context, &repo_context)?;

    print_pipeline_run_success();

    if node_exports.len() > 0usize {
        println!();
        print_pipeline_export_success(&node_exports);
    }

    Ok(())
}
