use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;

use crate::error::PipelineError;
use crate::error::PipelineError::ConnectionError;
use holium_pipe::types::Pipe;

/*************************************************************
 * Pipelines
 *************************************************************/

pub type HoliumGraph = DiGraph<String, String>;

/// `Pipeline` is a structure organizing the execution of different pipes to form a Holium data. Its
/// main component is a directed acyclic graph.
#[derive(Clone, Debug)]
pub struct Pipeline {
    pub name: String,
    pub documentation: String,
    dag: HoliumGraph,
}

impl Pipeline {
    pub fn new(name: String) -> Self {
        Pipeline {
            name,
            documentation: String::new(),
            dag: HoliumGraph::new(),
        }
    }

    /*************************************************************
     * Getter
     *************************************************************/
    pub fn dag(&self) -> &HoliumGraph {
        &self.dag
    }

    /*************************************************************
     * Setter
     *************************************************************/
    /// Function to set a new lone pipe in the pipeline
    // TODO here set cid as the type that will exist once developed
    pub fn add_pipe(
        &mut self,
        holium_object: &HoliumCidPlaceHolder,
        connections: Vec<&HoliumCidPlaceHolder>,
    ) -> Result<&mut Self, PipelineError> {
        self.add_lone_pipe(holium_object)?;
        self.connect_pipe(holium_object, connections)
    }

    /// Function to set a new lone pipe in the pipeline
    // TODO here set cid as the type that will exist once developed
    pub fn add_lone_pipe(
        &mut self,
        holium_object: &HoliumCidPlaceHolder,
    ) -> Result<&mut Self, PipelineError> {
        if !holium_object.object.is_pipe() {
            return Err(PipelineError::ObjectNotPipe(holium_object.cid.clone()));
        }

        self.dag.add_node(holium_object.cid.clone());
        Ok(self)
    }

    /// Function to remove a pipe from the pipeline, returns `None` if the pipe is not in the pipeline
    /// and the pipe CID in case of success
    pub fn remove_pipe(&mut self, pipe_cid: String) -> Result<&mut Self, PipelineError> {
        let result = self.dag.node_indices().find(|i| self.dag[*i] == pipe_cid);
        if result.is_none() {
            return Err(PipelineError::CidNotFound(pipe_cid));
        }

        self.dag.remove_node(result.unwrap());
        Ok(self)
    }

    pub fn connect_pipe(
        &mut self,
        holium_object_to_connect: &HoliumCidPlaceHolder,
        connections: Vec<&HoliumCidPlaceHolder>,
    ) -> Result<&mut Self, PipelineError> {
        // Basic data check
        if !holium_object_to_connect.object.is_pipe() {
            return Err(PipelineError::ObjectNotPipe(
                holium_object_to_connect.cid.clone(),
            ));
        }

        let node_result = self
            .dag
            .node_indices()
            .find(|i| self.dag[*i] == holium_object_to_connect.cid);
        if node_result.is_none() {
            return Err(PipelineError::CidNotFound(
                holium_object_to_connect.cid.clone(),
            ));
        }

        let pipe_to_connect = holium_object_to_connect.object.pipe().unwrap();

        if pipe_to_connect.connectors.len() != connections.len() {
            return Err(ConnectionError);
        }

        // Before connecting, making sure all nodes exists
        let mut connections_indexes: Vec<NodeIndex> = vec![];

        for i in 0..connections.len() {
            let connection = self
                .dag
                .node_indices()
                .find(|n_i| self.dag[*n_i] == connections[i].cid);
            if connection.is_none() {
                return Err(PipelineError::CidNotFound(connections[i].cid.clone()));
            }

            if !connections[i].object.is_pipe() {
                return Err(PipelineError::ObjectNotPipe(connections[i].cid.clone()));
            }
            connections_indexes.push(connection.unwrap());
        }

        // Connecting nodes
        let node_to_connect = node_result.unwrap();

        for connection in connections_indexes {
            self.dag
                .add_edge(connection, node_to_connect.clone(), String::new());
        }

        Ok(self)
    }

    /*************************************************************
     * Utilities
     *************************************************************/

    /// Function to return all CIDs that are missing either an incoming or an outgoing pipe connection
    pub fn list_unconnected_pipes(&self) -> Vec<&str> {
        self.dag
            .node_indices()
            .filter(|i| {
                self.dag.neighbors_directed(*i, Direction::Incoming).count() == 0
                    || self.dag.neighbors_directed(*i, Direction::Outgoing).count() == 0
            })
            .map(|i| self.dag[i].as_str())
            .collect()
    }

    /// Function to return the incoming and outgoing pipe CIDs for a given CID
    pub fn connections(&self, pipe_cid: String) -> Option<(Vec<&str>, Vec<&str>)> {
        let result = self.dag.node_indices().find(|i| self.dag[*i] == pipe_cid);
        if result.is_none() {
            return None;
        }

        let incoming_cids = get_directed_cids(&self.dag, &result.unwrap(), Direction::Incoming);
        let outgoing_cids = get_directed_cids(&self.dag, &result.unwrap(), Direction::Outgoing);
        return Some((incoming_cids, outgoing_cids));
    }
}

/*************************************************************
 * Utilities
 *************************************************************/

fn get_directed_cids<'a>(
    dag: &'a HoliumGraph,
    node_index: &NodeIndex,
    direction: Direction,
) -> Vec<&'a str> {
    dag.neighbors_directed(*node_index, direction)
        .map(|i| dag[i].as_str())
        .collect()
}

/*************************************************************
 * TODO code to adapt once CID crate developed
 *************************************************************/

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HoliumObject {
    Pipe(Pipe),
    Other,
}

impl HoliumObject {
    fn is_pipe(&self) -> bool {
        match self {
            HoliumObject::Pipe(_) => true,
            _ => false,
        }
    }

    fn pipe(&self) -> Option<&Pipe> {
        match self {
            HoliumObject::Pipe(pipe) => Some(pipe),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoliumCidPlaceHolder {
    cid: String,
    object: HoliumObject,
}

impl HoliumCidPlaceHolder {
    pub fn new(cid: String, object: HoliumObject) -> Self {
        HoliumCidPlaceHolder { cid, object }
    }
}
