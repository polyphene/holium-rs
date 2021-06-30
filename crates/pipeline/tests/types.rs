use holium_pipe::types::{Connector, Pipe};
use holium_pipeline::error::PipelineError;
use holium_pipeline::types::{HoliumCidPlaceHolder, HoliumObject, Pipeline};
use petgraph::graph::{EdgeIndex, NodeIndex};

/*************************************************************
 * Pipelines test
 *************************************************************/

#[test]
fn test_new_pipeline() {
    let name = String::from("name");

    let pipeline = Pipeline::new(name.clone());

    assert_eq!(name, pipeline.name);
    assert_eq!(String::new(), pipeline.documentation);
}

#[test]
fn test_add_lone_pipe_pipeline() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    /*************************************************************
     * Object is not Pipe
     *************************************************************/
    let wrong_object_cid = String::from("wrong_object_cid");
    let wrong_holium_cid_place_holder =
        HoliumCidPlaceHolder::new(wrong_object_cid.clone(), HoliumObject::Other);

    let result = pipeline.add_lone_pipe(&wrong_holium_cid_place_holder);

    assert_eq!(true, result.is_err());
    assert_eq!(
        PipelineError::ObjectNotPipe(wrong_object_cid),
        result.err().unwrap()
    );

    /*************************************************************
     * Passing test
     *************************************************************/

    let pipe_cid = String::from("pipe_cid");
    let pipe_bytecode_cid = String::from("pipe_bytecode_cid");
    let transformation_handle = String::from("transformation_handle");
    let holium_cid_place_holder = pipe_sample_holium_cid_place_holder(
        pipe_cid.clone(),
        pipe_bytecode_cid,
        transformation_handle,
        2,
    );

    pipeline.add_lone_pipe(&holium_cid_place_holder).unwrap();

    let node_indices: Vec<NodeIndex> = pipeline.dag().node_indices().collect();
    let node_cids: Vec<&str> = pipeline
        .dag()
        .node_indices()
        .map(|i| pipeline.dag()[i].as_str())
        .collect();

    assert_eq!(vec![NodeIndex::new(0)], node_indices);
    assert_eq!(vec![pipe_cid], node_cids);
}

#[test]
fn test_connect_pipe_pipeline() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("pipe_cid");
    let pipe_bytecode_cid = String::from("pipe_bytecode_cid");
    let transformation_handle = String::from("transformation_handle");
    let holium_cid_place_holder = pipe_sample_holium_cid_place_holder(
        pipe_cid.clone(),
        pipe_bytecode_cid.clone(),
        transformation_handle.clone(),
        1,
    );

    pipeline.add_lone_pipe(&holium_cid_place_holder).unwrap();

    /*************************************************************
     * Object is not Pipe
     *************************************************************/
    let non_pipe_object = HoliumCidPlaceHolder::new(pipe_cid.clone(), HoliumObject::Other);
    let connections: Vec<&HoliumCidPlaceHolder> = vec![];

    let result = pipeline.connect_pipe(&non_pipe_object, connections);

    assert_eq!(true, result.is_err());
    assert_eq!(
        PipelineError::ObjectNotPipe(pipe_cid.clone()),
        result.err().unwrap()
    );

    /*************************************************************
     * Pipe to connect CID does not exists, returns None
     *************************************************************/
    let wrong_object_cid = String::from("wrong_object_cid");
    let wrong_cid_object = pipe_sample_holium_cid_place_holder(
        wrong_object_cid.clone(),
        pipe_bytecode_cid.clone(),
        transformation_handle.clone(),
        1,
    );
    let connections: Vec<&HoliumCidPlaceHolder> = vec![];

    let result = pipeline.connect_pipe(&wrong_cid_object, connections);

    assert_eq!(true, result.is_err());
    assert_eq!(
        PipelineError::CidNotFound(wrong_object_cid.clone()),
        result.err().unwrap()
    );

    /*************************************************************
     * Pipe to connect connectors length does not equal passed
     * connections
     *************************************************************/
    let connections: Vec<&HoliumCidPlaceHolder> = vec![];

    let result = pipeline.connect_pipe(&holium_cid_place_holder, connections);

    assert_eq!(true, result.is_err());
    assert_eq!(PipelineError::ConnectionError, result.err().unwrap());

    /*************************************************************
     * Connections CID does not exists
     *************************************************************/
    let wrong_connection_cid = String::from("wrong_connection_cid");
    let wrong_connection_object = pipe_sample_holium_cid_place_holder(
        wrong_connection_cid.clone(),
        pipe_bytecode_cid.clone(),
        transformation_handle.clone(),
        1,
    );
    let connections: Vec<&HoliumCidPlaceHolder> = vec![&wrong_connection_object];

    let result = pipeline.connect_pipe(&holium_cid_place_holder, connections);

    assert_eq!(true, result.is_err());
    assert_eq!(
        PipelineError::CidNotFound(wrong_connection_cid),
        result.err().unwrap()
    );

    /*************************************************************
     * Add a proper pipe to personify a connection later on
     *************************************************************/
    let connection_cid = String::from("connection_cid");
    let connection_bytecode_cid = String::from("connection_bytecode_cid");
    let connection_transformation_handle = String::from("connection_transformation_handle");
    let connection = pipe_sample_holium_cid_place_holder(
        connection_cid.clone(),
        connection_bytecode_cid.clone(),
        connection_transformation_handle.clone(),
        1,
    );
    let proper_connections: Vec<&HoliumCidPlaceHolder> = vec![&connection];

    pipeline.add_lone_pipe(&connection).unwrap();

    /*************************************************************
     * Connections are not Pipe object
     *************************************************************/
    let non_pipe_connection_object =
        HoliumCidPlaceHolder::new(connection_cid.clone(), HoliumObject::Other);
    let connections: Vec<&HoliumCidPlaceHolder> = vec![&non_pipe_connection_object];
    let result = pipeline.connect_pipe(&holium_cid_place_holder, connections);

    assert_eq!(true, result.is_err());
    assert_eq!(
        PipelineError::ObjectNotPipe(connection_cid.clone()),
        result.err().unwrap()
    );

    /*************************************************************
     * Passing test
     *************************************************************/
    let result = pipeline.connect_pipe(&holium_cid_place_holder, proper_connections);

    assert_eq!(true, result.is_ok());

    let edge_indices: Vec<EdgeIndex> = pipeline.dag().edge_indices().collect();

    assert_eq!(1 as usize, edge_indices.len());

    let edge_endpoints: (NodeIndex, NodeIndex) =
        pipeline.dag().edge_endpoints(edge_indices[0]).unwrap();

    assert_eq!(
        (
            pipeline
                .dag()
                .node_indices()
                .find(|i| pipeline.dag()[*i] == connection_cid)
                .unwrap(),
            pipeline
                .dag()
                .node_indices()
                .find(|i| pipeline.dag()[*i] == pipe_cid)
                .unwrap()
        ),
        edge_endpoints
    );
}

#[test]
fn test_remove_pipe_pipeline() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("pipe_cid");
    let pipe_bytecode_cid = String::from("pipe_bytecode_cid");
    let transformation_handle = String::from("transformation_handle");
    let holium_cid_place_holder = pipe_sample_holium_cid_place_holder(
        pipe_cid.clone(),
        pipe_bytecode_cid,
        transformation_handle,
        2,
    );

    pipeline.add_lone_pipe(&holium_cid_place_holder).unwrap();

    /*************************************************************
     * CID does not exists, returns None
     *************************************************************/

    let non_existing_cid = String::from("non_existing_cid");

    let result = pipeline.remove_pipe(non_existing_cid.clone());

    assert_eq!(true, result.is_err());
    assert_eq!(
        PipelineError::CidNotFound(non_existing_cid),
        result.err().unwrap()
    );

    /*************************************************************
     * Passing test
     *************************************************************/

    pipeline.remove_pipe(pipe_cid).unwrap();

    let node_indices: Vec<NodeIndex> = pipeline.dag().node_indices().collect();
    let node_cids: Vec<&str> = pipeline
        .dag()
        .node_indices()
        .map(|i| pipeline.dag()[i].as_str())
        .collect();

    let empty_indices: Vec<NodeIndex> = vec![];
    let empty_cids: Vec<&str> = vec![];
    assert_eq!(empty_indices, node_indices);
    assert_eq!(empty_cids, node_cids);
}

#[test]
fn test_list_unconnected_pipes_pipeline() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("pipe_cid");
    let pipe_bytecode_cid = String::from("pipe_bytecode_cid");
    let transformation_handle = String::from("transformation_handle");
    let holium_cid_place_holder = pipe_sample_holium_cid_place_holder(
        pipe_cid.clone(),
        pipe_bytecode_cid,
        transformation_handle,
        2,
    );

    pipeline.add_lone_pipe(&holium_cid_place_holder).unwrap();

    let unconnected_pipes = pipeline.list_unconnected_pipes();

    assert_eq!(vec![pipe_cid.as_str()], unconnected_pipes);
}

#[test]
fn test_connections_pipeline() {
    let name = String::from("name");

    let mut pipeline = Pipeline::new(name.clone());

    let pipe_cid = String::from("pipe_cid");
    let pipe_bytecode_cid = String::from("pipe_bytecode_cid");
    let transformation_handle = String::from("transformation_handle");
    let holium_cid_place_holder = pipe_sample_holium_cid_place_holder(
        pipe_cid.clone(),
        pipe_bytecode_cid,
        transformation_handle,
        1,
    );

    pipeline.add_lone_pipe(&holium_cid_place_holder).unwrap();

    /*************************************************************
     * CID does not exists, returns None
     *************************************************************/

    let non_existing_cid = String::from("non_existing_cid");

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
    let connection_cid_str = "connection_cid";
    let connection_cid = String::from(connection_cid_str);
    let connection_bytecode_cid = String::from("connection_bytecode_cid");
    let connection_transformation_handle = String::from("connection_transformation_handle");
    let connection = pipe_sample_holium_cid_place_holder(
        connection_cid.clone(),
        connection_bytecode_cid.clone(),
        connection_transformation_handle.clone(),
        1,
    );
    let proper_connections: Vec<&HoliumCidPlaceHolder> = vec![&connection];

    pipeline.add_lone_pipe(&connection).unwrap();

    pipeline
        .connect_pipe(&holium_cid_place_holder, proper_connections)
        .unwrap();

    let connections = pipeline.connections(pipe_cid.clone()).unwrap();

    let should_be: (Vec<&str>, Vec<&str>) = (vec![connection_cid_str], vec![]);
    assert_eq!(should_be, connections);

    let connections = pipeline.connections(connection_cid).unwrap();

    let should_be: (Vec<&str>, Vec<&str>) = (vec![], vec![pipe_cid.as_str()]);
    assert_eq!(should_be, connections);
}

/*************************************************************
 * Utilities
 *************************************************************/

fn pipe_sample_holium_cid_place_holder(
    pipe_cid: String,
    pipe_bytecode_cid: String,
    transformation_handle: String,
    nbr_connector: u32,
) -> HoliumCidPlaceHolder {
    let mut vec_connector: Vec<Connector> = vec![];

    for i in 0..nbr_connector {
        let cid = String::from(format!("connector_cid_{}", i));
        let outputs_indexes = vec![format!("{}.0", i), format!("{}.1", i)];
        let inputs_indexes = vec![format!("{}.0", i), format!("{}.1", i)];
        vec_connector.push(Connector::new(cid, outputs_indexes, inputs_indexes).unwrap())
    }

    let pipe = HoliumObject::Pipe(Pipe::new(
        pipe_bytecode_cid,
        transformation_handle,
        vec_connector,
    ));
    HoliumCidPlaceHolder::new(pipe_cid, pipe)
}
