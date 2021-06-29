use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;

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
    pub dag: HoliumGraph,
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
     * Setter
     *************************************************************/
    /// Function to set a new pipe in the pipeline
    // TODO here set cid as the type that will exist once developed
    pub fn add_pipe(&mut self, pipe_cid: String) -> &mut Self {
        self.dag.add_node(pipe_cid);
        self
    }

    /// Function to remove a pipe from the pipeline, returns `None` if the pipe is not in the pipeline
    /// and the pipe CID in case of success
    pub fn remove_pipe(&mut self, pipe_cid: String) -> Option<String> {
        let result = self.dag.node_indices().find(|i| self.dag[*i] == pipe_cid);
        if result.is_none() {
            return None;
        }

        self.dag.remove_node(result.unwrap())
    }

    pub fn connect(&mut self, left_pipe: String, right_pipe: String) -> &mut Self {
        // TODO

        self
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
