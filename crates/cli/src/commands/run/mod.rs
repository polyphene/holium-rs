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

#[derive(Error, Debug)]
/// errors
enum CmdError {
    #[error("{0} node not found in transformation graph key mapping")]
    EdgeEndpointNotFoundInKeyMapping(&'static str),
}

/// command
pub(crate) fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("run").about("Run local transformation pipeline is it is valid")
}

/// handler
pub(crate) fn handle_cmd(matches: &ArgMatches) -> Result<()> {
    // create local context
    let local_context = LocalContext::new()?;
    // create pipeline dag
    let dag = PipelineDag::from_local_context(&local_context)?;
    // check if the dag is healthy for export
    let ordered_node_list = dag.is_valid_pipeline()?;

    for node_index in ordered_node_list.into_iter() {
        dbg!(node_index.clone());
        dbg!(dag.key_mapping.get_by_right(&node_index));
        for edge_reference in dag.graph.edges(node_index) {
            // Get tail and head typed name
            let tail_typed_name = dag
                .key_mapping
                .get_by_right(&edge_reference.source())
                .ok_or(CmdError::EdgeEndpointNotFoundInKeyMapping("source"))?;
            let head_typed_name = dag
                .key_mapping
                .get_by_right(&edge_reference.target())
                .ok_or(CmdError::EdgeEndpointNotFoundInKeyMapping("target"))?;
            // Build connection id
            let connection_id = build_connection_id(tail_typed_name, head_typed_name);
            // Retrieve connection object
            let encoded_connection = local_context
                .connections
                .get(&connection_id)
                .context(DbOperationFailed)?
                .ok_or(NoObjectForGivenKey(connection_id.to_string()))?;
            let mut decoded_connection: Connection = bincode::deserialize(&encoded_connection[..])
                .ok()
                .context(BinCodeDeserializeFailed)?;
            decoded_connection.id = connection_id.to_string();
            dbg!(decoded_connection);
            // TODO Check if there is portation
            let data = local_context
                .data
                .get(tail_typed_name)
                .context(DbOperationFailed)?
                .ok_or(NoDataForObject(tail_typed_name.clone()))?;
            dbg!(data);
        }
    }

    Ok(())
}
