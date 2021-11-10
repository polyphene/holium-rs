//! Run a transformation pipeline

use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, NoDataForNodeInput, NoObjectForGivenKey,
};
use crate::utils::local::context::helpers::build_connection_id;
use crate::utils::local::context::LocalContext;
use crate::utils::local::dag::models::PipelineDag;
use crate::utils::local::helpers::prints::commands_outputs::print_pipeline_run_success;
use crate::utils::local::models::connection::Connection;
use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use console::style;
use petgraph::prelude::EdgeRef;
use thiserror::Error;

use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR, LOCAL_DIR, PORTATIONS_FILE};
use crate::utils::run::runtime::Runtime;

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run").about("Run local transformation pipeline if it is valid")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // create runtime
    let mut runtime = Runtime::new()?;

    // Run Pipeline dag from local context
    PipelineDag::run(&mut runtime, &local_context)?;

    print_pipeline_run_success();
    // TODO when portation integrated, print all successful export

    Ok(())
}
