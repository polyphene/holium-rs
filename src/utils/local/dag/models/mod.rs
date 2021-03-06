//! Module related to the organisation of a transformation pipeline as a Directed Acyclic Graph (DAG).

use crate::utils::cbor::helpers::WriteError;
use crate::utils::cbor::write_holium_cbor::WriteHoliumCbor;
use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, NoDataForNodeInput, NoObjectForGivenKey,
};
use crate::utils::interplanetary::kinds::selector::Selector;
use crate::utils::local::context::helpers::{
    build_connection_id, build_node_typed_name, build_portation_id, db_key_to_str, get_node_data,
    parse_connection_id, parse_node_typed_name, store_node_output, NodeType,
    PortationDirectionType,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::models::connection::Connection;
use crate::utils::local::models::data::HoliumCbor;
use crate::utils::local::models::transformation::Transformation;
use crate::utils::repo::context::RepositoryContext;
use crate::utils::run::runtime::Runtime;
use anyhow::{anyhow, Context, Result};
use bimap::BiMap;

use petgraph::graph::{DiGraph, EdgeReference, NodeIndex};
use petgraph::prelude::EdgeRef;
use petgraph::{algo, Direction};

use std::convert::TryFrom;

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
    #[error("instantiation failed for transformation: {0}")]
    TransformationInstantiationFailed(String),
    #[error("execution failed for transformation: {0}")]
    TransformationRunFailed(String),
    #[error("could not collect connections details for node: {0}")]
    ConnectionsDetailsCollectionFailed(String),
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

    /// Check the a [PipelineDg] is healthy then runs the ordered list of node that it contains. It
    /// returns a vector of tuples containing the node typed name and the writen file path of nodes
    /// that had some export from Holium portation attached to them.
    pub fn run(
        runtime: &mut Runtime,
        local_context: &LocalContext,
        repo_context: &RepositoryContext,
    ) -> Result<Vec<(String, String)>> {
        // create pipeline dag
        let dag = PipelineDag::from_local_context(local_context)?;
        // check if the dag is healthy for export
        let ordered_node_list = dag.is_valid_pipeline()?;

        // Initialize Vec to return with node typed name and portation file path
        let mut node_portation_pairs: Vec<(String, String)> = Vec::new();

        for node_index in ordered_node_list.into_iter() {
            let node_typed_name = dag.node_typed_name(&node_index)?;
            let (node_type, node_name) = parse_node_typed_name(node_typed_name)?;

            // Check that if the node input is connected to no head selector then there are either
            // a portation or some data in local context. Otherwise error.
            if repo_context
                .portations
                .get(&build_portation_id(
                    &PortationDirectionType::toHolium,
                    &node_typed_name,
                ))
                .is_none()
                && local_context
                    .data
                    .get(node_typed_name)
                    .context(DbOperationFailed)?
                    .is_none()
                && dag
                    .graph
                    .edges_directed(node_index, Direction::Incoming)
                    .collect::<Vec<EdgeReference<_, _>>>()
                    .len()
                    == 0usize
            {
                return Err(NoDataForNodeInput(node_typed_name.to_string()).into());
            }

            // Initialize data for head connected node
            let mut data: HoliumCbor = Vec::new();

            // If we have no incoming connection then we set selected data as either the one in local
            // context or as the one coming from a portation
            if dag
                .graph
                .edges_directed(node_index, Direction::Incoming)
                .collect::<Vec<EdgeReference<_, _>>>()
                .len()
                == 0usize
            {
                data = get_node_data(local_context, repo_context, node_typed_name)?;
            } else {
                // Retrieve all information about connections so that we are able to form our selected
                // data
                let connections_details = dag
                    .graph
                    .edges_directed(node_index, Direction::Incoming)
                    .map(|edge_reference| {
                        dag.edge_details(local_context, repo_context, &edge_reference)
                    })
                    .collect::<Result<Vec<(String, HoliumCbor, Selector, Selector)>>>()
                    .context(Error::ConnectionsDetailsCollectionFailed(node_name.clone()))?;

                // Select data
                let res_copy = data.copy_cbor(&connections_details);
                match res_copy.err() {
                    Some(err) => {
                        return match err.downcast_ref::<WriteError>() {
                            Some(WriteError::CborGenerationFailed) => Err(anyhow!(format!(
                                "failed to generate cbor object for head data of node: {}",
                                node_typed_name
                            ))
                            .into()),
                            _ => Err(err),
                        }
                    }
                    _ => {}
                };
            }

            // If transformation then execute bytecode otherwise do nothing
            match node_type {
                NodeType::transformation => {
                    // get object from local database
                    let encoded = local_context
                        .transformations
                        .get(&node_name)
                        .context(DbOperationFailed)?
                        .ok_or(NoObjectForGivenKey(node_name.clone()))?;
                    let decoded_transformation: Transformation = bincode::deserialize(&encoded[..])
                        .ok()
                        .context(BinCodeDeserializeFailed)?;
                    // instantiate transformation
                    runtime
                        .instantiate(&decoded_transformation.bytecode)
                        .context(Error::TransformationInstantiationFailed(node_name.clone()))?;

                    // run transformation
                    data = runtime
                        .run(&decoded_transformation.handle, &data)
                        .context(Error::TransformationRunFailed(node_name.clone()))?;
                }
                _ => {}
            }

            // Store data in local context and execute *to-holium* portation if any.
            let portation_file_path =
                store_node_output(local_context, repo_context, node_typed_name, &data)?;
            if let Some(file_path) = portation_file_path {
                node_portation_pairs.push((node_typed_name.clone(), file_path.clone()));
            }
        }

        Ok(node_portation_pairs)
    }

    fn node_typed_name(&self, index: &NodeIndex) -> Result<&String> {
        self.key_mapping
            .get_by_right(index)
            .ok_or(Error::DagOperationFailed.into())
    }

    /// [edge_details] will return connection details based on an edge in our dag. The information
    /// are returned in a quartet containing the connection name, data at tail, the selector for the said data and
    /// the selector to re organize data for our receiving node
    fn edge_details(
        &self,
        local_context: &LocalContext,
        repo_context: &RepositoryContext,
        edge_reference: &EdgeReference<()>,
    ) -> Result<(String, HoliumCbor, Selector, Selector)> {
        // Get tail and head typed name
        let tail_typed_name = self.node_typed_name(&edge_reference.source())?;
        let head_typed_name = self.node_typed_name(&edge_reference.target())?;

        // Build connection id
        let connection_id = build_connection_id(tail_typed_name, head_typed_name);
        // Retrieve connection object
        let encoded_connection = local_context
            .connections
            .get(&connection_id)
            .context(DbOperationFailed)?
            .ok_or(NoObjectForGivenKey(connection_id.clone()))?;
        let decoded_connection: Connection = bincode::deserialize(&encoded_connection[..])
            .ok()
            .context(BinCodeDeserializeFailed)?;

        // Build selectors
        let tail_selector = Selector::try_from(decoded_connection.tail_selector.as_str())?;
        let head_selector = Selector::try_from(decoded_connection.head_selector.as_str())?;

        // Arrange data to fit head selector
        let data_at_tail = get_node_data(local_context, repo_context, tail_typed_name)?;

        Ok((connection_id, data_at_tail, tail_selector, head_selector))
    }
}
