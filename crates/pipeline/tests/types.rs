use holium_pipeline::types::Pipeline;
use petgraph::graph::{DiGraph, Node, NodeIndex};
use std::borrow::Borrow;

/*************************************************************
 * Pipelines test
 *************************************************************/

#[test]
fn test_new_pipeline() {
    let name = String::from("name");

    let pipeline = Pipeline::new(name.clone());

    assert_eq!(name, pipeline.name);
    assert_eq!(String::new(), pipeline.documentation);
    // TODO need to check if dag correctly init
}

#[test]
fn test_add_pipe() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("iamacid");

    pipeline.add_pipe(pipe_cid.clone());

    let node_indices: Vec<NodeIndex> = pipeline.dag.node_indices().collect();
    let node_cids: Vec<&str> = pipeline
        .dag
        .node_indices()
        .map(|i| pipeline.dag[i].as_str())
        .collect();

    assert_eq!(vec![NodeIndex::new(0)], node_indices);
    assert_eq!(vec![pipe_cid], node_cids);
}

fn test_remove_pipe() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("iamacid");

    pipeline.add_pipe(pipe_cid.clone());

    /*************************************************************
     * CID does not exists, returns None
     *************************************************************/

    let non_existing_cid = String::from("iamacid2");

    let result = pipeline.remove_pipe(non_existing_cid);

    assert_eq!(true, result.is_none());

    /*************************************************************
     * Passing test
     *************************************************************/

    pipeline.remove_pipe(pipe_cid);

    let node_indices: Vec<NodeIndex> = pipeline.dag.node_indices().collect();
    let node_cids: Vec<&str> = pipeline
        .dag
        .node_indices()
        .map(|i| pipeline.dag[i].as_str())
        .collect();

    let empty_indices: Vec<NodeIndex> = vec![];
    let empty_cids: Vec<&str> = vec![];
    assert_eq!(empty_indices, node_indices);
    assert_eq!(empty_cids, node_cids);
}

#[test]
fn test_list_unconnected_pipes_pipe() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("iamacid");

    pipeline.add_pipe(pipe_cid.clone());

    let unconnected_pipes = pipeline.list_unconnected_pipes();

    assert_eq!(vec![pipe_cid.as_str()], unconnected_pipes);
}

#[test]
fn test_connections_pipe() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("iamacid");

    pipeline.add_pipe(pipe_cid.clone());

    /*************************************************************
     * CID does not exists, returns None
     *************************************************************/

    let non_existing_cid = String::from("iamacid2");

    let result = pipeline.connections(non_existing_cid);

    assert_eq!(true, result.is_none());

    /*************************************************************
     * Passing test with no connections
     *************************************************************/

    let connections = pipeline.connections(pipe_cid.clone()).unwrap();

    let empty_connections: (Vec<&str>, Vec<&str>) = (vec![], vec![]);
    assert_eq!(empty_connections, connections);

    /*************************************************************
     * Passing test with connections
     *************************************************************/
    let outgoing_pipe_cid = String::from("outgoing_pipe_cid");
    let incoming_pipe_cid = String::from("incoming_pipe_cid");

    pipeline.connect(pipe_cid.clone(), outgoing_pipe_cid.clone());
    pipeline.connect(incoming_pipe_cid.clone(), pipe_cid.clone());

    let connections = pipeline.connections(pipe_cid.clone()).unwrap();

    let should_be: (Vec<&str>, Vec<&str>) = (
        vec![incoming_pipe_cid.as_str()],
        vec![outgoing_pipe_cid.as_str()],
    );
    assert_eq!(should_be, connections);
}
