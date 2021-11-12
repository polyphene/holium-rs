use std::collections::HashMap;

use anyhow::{Context, Result};
use cid::Cid;
use sk_cbor::Value;

use crate::utils::errors::Error::{BinCodeDeserializeFailed, DbOperationFailed};
use crate::utils::interplanetary::context::InterplanetaryContext;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;

use crate::utils::interplanetary::kinds::connection::Connection as ConnectionBlock;
use crate::utils::interplanetary::kinds::dry_transformation::DryTransformation;
use crate::utils::interplanetary::kinds::helpers::holium_data::HoliumInterplanetaryNodeData;
use crate::utils::interplanetary::kinds::metadata::Metadata;
use crate::utils::interplanetary::kinds::module_bytecode::ModuleBytecode;
use crate::utils::interplanetary::kinds::module_bytecode_envelope::ModuleBytecodeEnvelope;
use crate::utils::interplanetary::kinds::pipeline::Pipeline;
use crate::utils::interplanetary::kinds::pipeline_edge::PipelineEdge;
use crate::utils::interplanetary::kinds::pipeline_vertex::PipelineVertex;
use crate::utils::interplanetary::kinds::selector::SelectorEnvelope;
use crate::utils::local::context::helpers::{
    build_node_typed_name, db_key_to_str, parse_connection_id, NodeType,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::models::connection::Connection;

use crate::utils::local::models::transformation::Transformation;

/// [ VerticesContentMap ] is used to map nodes' name to their content while constructing the
/// interplanetary representation of a pipeline.
pub type VerticesContentMap = HashMap<String, PipelineVertex>;

/// [ VerticesKeyMap ] is used to map nodes' name to their index in the final pipeline list of
/// vertices while constructing the interplanetary representation of a pipeline.
pub type VerticesKeyMap = HashMap<String, u64>;

pub fn export_project(
    local_context: &LocalContext,
    ip_context: &InterplanetaryContext,
) -> Result<Cid> {
    // initialize an object to store the content of the graph nodes
    let mut vertices_content = VerticesContentMap::new();
    // export dry transformations
    export_dry_transformations(&local_context, &ip_context, &mut vertices_content)?;
    // export data
    export_data(&local_context, &ip_context, &mut vertices_content)?;
    // export metadata
    export_metadata(&local_context, &ip_context, &mut vertices_content)?;
    // export connections
    let (edges, vertices_key_mapping) = export_connections(&local_context, &ip_context)?;
    // export the pipeline itself, and return its cid
    export_pipeline(&ip_context, &vertices_key_mapping, &vertices_content, edges)
}

fn export_dry_transformations(
    local_context: &LocalContext,
    ip_context: &InterplanetaryContext,
    vertices_content: &mut VerticesContentMap,
) -> Result<()> {
    for object in local_context.transformations.iter() {
        // decode the object
        let (name_vec, encoded) = object.context(DbOperationFailed)?;
        let name = db_key_to_str(name_vec)?;
        let decoded: Transformation = bincode::deserialize(&encoded[..])
            .ok()
            .context(BinCodeDeserializeFailed)?;
        // store the bytecode
        let module_bytecode = ModuleBytecode::new(decoded.bytecode);
        let module_bytecode_cid = module_bytecode.write_to_ip_area(&ip_context)?;
        // store the module bytecode envelope
        let module_bytecode_envelope = ModuleBytecodeEnvelope::new(module_bytecode_cid);
        let module_bytecode_envelope_cid =
            Value::from(module_bytecode_envelope).write_to_ip_area(&ip_context)?;
        // store the dry transformation
        let dry_transformation =
            DryTransformation::new(module_bytecode_envelope_cid, decoded.handle);
        let dry_transformation_cid =
            Value::from(dry_transformation).write_to_ip_area(&ip_context)?;
        // add it to the vertices context map
        let typed_name = build_node_typed_name(&NodeType::transformation, name.as_str());
        let vertex_content = vertices_content
            .entry(typed_name)
            .or_insert(PipelineVertex::default());
        vertex_content.dry_transformation = Some(dry_transformation_cid);
    }
    Ok(())
}

fn export_data(
    local_context: &LocalContext,
    ip_context: &InterplanetaryContext,
    vertices_content: &mut VerticesContentMap,
) -> Result<()> {
    for object in local_context.data.iter() {
        // decode the object
        let (name_vec, data_vec) = object.context(DbOperationFailed)?;
        let node_typed_name = db_key_to_str(name_vec)?;
        let data = data_vec.to_vec();
        // recursively build and write the data structure
        let data_cid =
            HoliumInterplanetaryNodeData::new(data)?.recursively_write_to_ip_area(&ip_context)?;
        // add it to the vertices context map
        let vertex_content = vertices_content
            .entry(node_typed_name)
            .or_insert(PipelineVertex::default());
        vertex_content.data = Some(data_cid);
    }
    Ok(())
}

fn export_metadata(
    local_context: &LocalContext,
    ip_context: &InterplanetaryContext,
    vertices_content: &mut VerticesContentMap,
) -> Result<()> {
    for (local_context_tree, node_type) in local_context.get_nodes_tree_type_tuples().iter() {
        for object in local_context_tree.iter() {
            // decode the object
            let (name_vec, encoded) = object.context(DbOperationFailed)?;
            let name = db_key_to_str(name_vec)?;
            // create and write a metadata block
            let metadata = Metadata::new(&name, &encoded, &node_type)?;
            let metadata_cid = Value::from(metadata).write_to_ip_area(&ip_context)?;
            // add it to the vertices context map
            let typed_name = build_node_typed_name(&node_type, name.as_str());
            let vertex_content = vertices_content
                .entry(typed_name)
                .or_insert(PipelineVertex::default());
            vertex_content.metadata = Some(metadata_cid);
        }
    }
    Ok(())
}

fn export_connections(
    local_context: &LocalContext,
    ip_context: &InterplanetaryContext,
) -> Result<(Vec<PipelineEdge>, VerticesKeyMap)> {
    // initialize mapping from nodes' typed names to content block future index in the pipeline's
    // list of vertices, and the list of edges
    let mut next_vertex_idx: u64 = 0;
    let mut vertex_idx_mapping = VerticesKeyMap::new();
    let mut edges: Vec<PipelineEdge> = Vec::with_capacity(local_context.connections.len());
    // iterate on the list of connexions
    for object in local_context.connections.iter() {
        // decode the object
        let (id_vec, encoded) = object.context(DbOperationFailed)?;
        let id = db_key_to_str(id_vec)?;
        let (tail_typed_name, head_typed_name) = parse_connection_id(&id)?;
        let decoded: Connection = bincode::deserialize(&encoded[..])
            .ok()
            .context(BinCodeDeserializeFailed)?;
        // get nodes indices or assign new ones if necessary
        let tail_index = vertex_idx_mapping
            .entry(tail_typed_name.to_string())
            .or_insert_with(|| increment_after(&mut next_vertex_idx))
            .to_owned();
        let head_index = vertex_idx_mapping
            .entry(head_typed_name.to_string())
            .or_insert_with(|| increment_after(&mut next_vertex_idx))
            .to_owned();
        // store selectors
        let tail_selector = SelectorEnvelope::new(&decoded.tail_selector)?;
        let tail_selector_cid = Value::from(tail_selector).write_to_ip_area(&ip_context)?;
        let head_selector = SelectorEnvelope::new(&decoded.head_selector)?;
        let head_selector_cid = Value::from(head_selector).write_to_ip_area(&ip_context)?;
        // store connection
        let connection = ConnectionBlock::new(tail_selector_cid, head_selector_cid);
        let connection_cid = Value::from(connection).write_to_ip_area(&ip_context)?;
        // add new edge to the list
        edges.push(PipelineEdge {
            tail_index,
            head_index,
            connection_cid,
        })
    }
    Ok((edges, vertex_idx_mapping))
}

fn increment_after(x: &mut u64) -> u64 {
    let prev: u64 = x.clone();
    *x += 1;
    prev
}

fn export_pipeline(
    ip_context: &InterplanetaryContext,
    vertices_key_mapping: &VerticesKeyMap,
    vertices_content: &VerticesContentMap,
    edges: Vec<PipelineEdge>,
) -> Result<Cid> {
    // create the pipeline object
    let pipeline = Pipeline::new(&vertices_key_mapping, &vertices_content, edges)?;
    // store it and return its cid
    Value::from(pipeline).write_to_ip_area(&ip_context)
}
