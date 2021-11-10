//! Run a transformation pipeline

use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, NoDataForObject, NoObjectForGivenKey,
};
use crate::utils::local::context::helpers::build_connection_id;
use crate::utils::local::context::LocalContext;
use crate::utils::local::dag::models::PipelineDag;
use crate::utils::local::models::connection::Connection;
use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use console::style;
use petgraph::prelude::EdgeRef;
use thiserror::Error;

use crate::utils::repo::constants::{HOLIUM_DIR, INTERPLANETARY_DIR, LOCAL_DIR, PORTATIONS_FILE};
use crate::utils::run::runtime::Runtime;

#[derive(Error, Debug)]
/// errors
enum CmdError {}

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run").about("Run local transformation pipeline is it is valid")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // create runtime
    let mut runtime = Runtime::new()?;

    local_context.data.insert(
        "source:hello",
        serde_cbor::to_vec(&serde_json::json!([1000, 100])).unwrap(),
    );
    local_context.data.insert(
        "source:hello_bis",
        serde_cbor::to_vec(&serde_json::json!([200])).unwrap(),
    );

    // Run Pipeline dag from local context
    PipelineDag::run(&mut runtime, &local_context)?;

    Ok(())
}
