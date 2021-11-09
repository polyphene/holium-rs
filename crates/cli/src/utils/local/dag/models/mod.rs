//! Module related to the organisation of a transformation pipeline as a Directed Acyclic Graph (DAG).

use crate::utils::errors::Error::DbOperationFailed;
use crate::utils::local::context::helpers::{
    build_node_typed_name, db_key_to_str, parse_connection_id, NodeType,
};
use crate::utils::local::context::LocalContext;
use anyhow::{anyhow, Context, Result};
use bimap::BiMap;
use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(thiserror::Error, Debug)]
/// Errors related to the [ PipelineDag ] structure.
pub(crate) enum Error {
    /// This error is thrown when an operation on a [ PipelineDag ] fails for internal reasons.
    #[error("failed to operate on the pipeline graph")]
    DagOperationFailed,
    #[error(
        "a transformation pipeline graph cannot hold any cycle. Hint: {0} is part of a cycle."
    )]
    GraphIsCyclic(String),
    #[error("all nodes of a transformation pipeline should be connected")]
    UnconnectedGraphNodes,
    #[error("connection between unknown nodes: {0}")]
    ConnectionBetweenUnknownNodes(String),
}

/// Structure holing information useful to the management of a transformation pipeline as a DAG
pub struct PipelineDag {
    pub graph: DiGraph<(), ()>,
    pub key_mapping: BiMap<String, NodeIndex>,
}

impl PipelineDag {
    /// Build a [PipelineDag] from a local area context object
    pub fn from_local_context(local_context: &LocalContext) -> Result<Self> {
        // init assets
        let mut graph = DiGraph::<(), ()>::new();
        let mut key_mapping = BiMap::<String, NodeIndex>::new();
        // list all nodes and add them to the graph
        for (local_context_tree, node_type) in local_context.get_nodes_tree_type_tuples().iter() {
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
            let tail_index = key_mapping.get_by_left(tail_typed_name).ok_or(
                Error::ConnectionBetweenUnknownNodes(tail_typed_name.to_string()),
            )?;
            let head_index = key_mapping.get_by_left(head_typed_name).ok_or(
                Error::ConnectionBetweenUnknownNodes(head_typed_name.to_string()),
            )?;
            // add the edge to the graph
            graph.add_edge(*tail_index, *head_index, ());
        }
        // return
        Ok(PipelineDag { graph, key_mapping })
    }

    /// Check if a [ PipelineDag ] is healthy, meaning it holds one connected and acyclic graph.
    /// In case it is healthy, return a vector of nodes in topological order.
    /// In case it is not, return an error.
    pub fn is_valid_pipeline(&self) -> Result<Vec<NodeIndex>> {
        let sorted_nodes: Vec<NodeIndex>;
        match algo::toposort(&self.graph, None) {
            Ok(stack) => sorted_nodes = stack,
            Err(cycle) => {
                // get the name of a node in a cycle
                let node_id = cycle.node_id();
                let node_name = self
                    .key_mapping
                    .get_by_right(&node_id)
                    .ok_or(Error::DagOperationFailed)?;
                return Err(Error::GraphIsCyclic(node_name.to_string()).into());
            }
        }
        if algo::connected_components(&self.graph) != 1 {
            // TODO handle in a specific way the case with no node and no component
            return Err(Error::UnconnectedGraphNodes.into());
        }
        Ok(sorted_nodes)
    }
}
