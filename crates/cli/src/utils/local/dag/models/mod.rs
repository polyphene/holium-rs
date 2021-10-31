//! Module related to the organisation of a transformation pipeline as a Directed Acyclic Graph (DAG).

use anyhow::{anyhow, Context, Result};
use bimap::BiMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo;
use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::DbOperationFailed;
use crate::utils::local::context::helpers::{build_node_typed_name, NodeType, parse_connection_id, db_key_to_str};

#[derive(thiserror::Error, Debug)]
/// Errors related to the [ PipelineDag ] structure.
pub(crate) enum Error {
    #[error("a transformation pipeline graph cannot hold any cycle")]
    GraphIsCyclic,
    #[error("all nodes of a transformation pipeline should be connected")]
    UnconnectedGraphNodes,
}

/// Structure holing information useful to the management of a transformation pipeline as a DAG
pub struct PipelineDag {
    graph: DiGraph<(), ()>,
    key_mapping: BiMap::<String, NodeIndex>,
}

impl PipelineDag {
    /// Build a [PipelineDag] from a local area context object
    pub fn from_local_context(local_context: &LocalContext) -> Result<Self> {
        // init assets
        let mut graph = DiGraph::<(), ()>::new();
        let mut key_mapping = BiMap::<String, NodeIndex>::new();
        // list all nodes and add them to the graph
        for (local_context_tree, node_type) in
        local_context
            .get_nodes_tree_type_tuples()
            .iter() {
            for k in local_context_tree.iter().keys() {
                // get the typed name of the node
                let name_vec = k.context(DbOperationFailed)?;
                let name = db_key_to_str(name_vec)?;
                let typed_name = build_node_typed_name(&node_type, &name);
                // add it to the graph
                let node_index = graph.add_node(());
                // insert it into the bimap
                key_mapping.insert(typed_name, node_index);
            }
        }
        // list all edges and add them to the graph
        for k in local_context.connections.iter().keys() {
            // get the vertices' names
            let id_vec = k.context(DbOperationFailed)?;
            let id = db_key_to_str(id_vec)?;
            let (tail_typed_name, head_typed_name) = parse_connection_id(&id)?;
            // get the vertices' node indices
            let tail_index = key_mapping
                .get_by_left(tail_typed_name)
                .ok_or(anyhow!("NOT necessarily internal err, todo"))?;
            let head_index = key_mapping
                .get_by_left(head_typed_name)
                .ok_or(anyhow!("NOT necessarily internal err, todo"))?;
            // add the edge to the graph
            graph.add_edge(*tail_index, *head_index, ());
        }
        // return
        Ok(PipelineDag { graph, key_mapping, })
    }

    /// Check if a [ PipelineDag ] is healthy, meaning it holds one connected and acyclic graph.
    /// In case it is not, return an error.
    pub fn is_valid_pipeline(&self) -> Result<()> {
        if algo::is_cyclic_directed(&self.graph) {
            return Err(Error::GraphIsCyclic.into());
        }
        if algo::connected_components(&self.graph) != 1 {
            // TODO handle in a specific way the case with no node and no component
            return Err(Error::UnconnectedGraphNodes.into());
        }
        Ok(())
    }
}