use anyhow::{anyhow, Context, Result};
use crate::utils::interplanetary::context::InterplanetaryContext;
use crate::utils::local::context::LocalContext;
use std::path::{PathBuf, Path};
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{SeekFrom, Seek, Read};
use crate::utils::interplanetary::multiformats::path_to_cid;
use cid::Cid;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::kinds::pipeline::Pipeline;
use crate::utils::interplanetary::kinds::metadata::Metadata;
use crate::utils::interplanetary::kinds::connection::Connection as ConnectionBlock;
use crate::utils::local::models::connection::Connection;
use std::convert::TryFrom;
use crate::utils::local::export::VerticesKeyMap;
use crate::utils::local::helpers::selector::validate_selector;
use crate::utils::interplanetary::kinds::selector::SelectorEnvelope;
use crate::utils::errors::Error::{BinCodeSerializeFailed, DbOperationFailed};
use std::collections::HashMap;
use crate::utils::local::context::helpers::{build_connection_id, parse_node_typed_name, NodeType};
use crate::utils::interplanetary::kinds::pipeline_vertex::PipelineVertex;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::shaper::Shaper;
use crate::utils::local::models::transformation::Transformation;
use crate::utils::interplanetary::kinds::dry_transformation::DryTransformation;
use crate::utils::interplanetary::kinds::module_bytecode_envelope::ModuleBytecodeEnvelope;
use crate::utils::interplanetary::kinds::module_bytecode::ModuleBytecode;


/// Ending bytes of any Pipeline interplanetary block
const PIPELINE_BLOC_SUFFIX: &[u8; 18] =
    b"\x6c\x74\x79\x70\x65\x64\x56\x65\x72\x73\x69\x6f\x6e\x64\x70\x6c\x5f\x30";

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to find a Pipeline block in the interplanetary area")]
    FindToFindPipelineBlock,
    #[error("no Metadata block linked from the pipeline node")]
    NoMetadataFoundInTheNode,
    #[error("node name missing from the Metadata block")]
    NoNameInMetadata,
    #[error("schema missing from the Metadata block")]
    MissingSchemaInMetadata,
    #[error("tail node name missing from the Pipeline edge")]
    NoTailNameInEdge,
    #[error("head node name missing from the Pipeline edge")]
    NoHeadNameInEdge,
    #[error("failed to parse selector")]
    FailedToParseSelector,
    #[error("no Dry Transformation block linked from the pipeline node")]
    NoTransformationInNode,
}

pub type VerticesImportKeyMap = HashMap<u64, String>;

/// Import a project containing a pipeline in an interplanetary area into a local area
pub fn import_project(ip_context: &InterplanetaryContext, tmp_local_context: &LocalContext) -> Result<()> {
    // find a pipeline block in the interplanetary area and parse it
    let pipeline = import_pipeline(&ip_context)?;
    // import content from pipeline vertices
    let vertex_idx_mapping = import_vertices(&ip_context, &tmp_local_context, &pipeline)?;
    // import connections from edges
    import_edges(&ip_context, &tmp_local_context, &pipeline, &vertex_idx_mapping)?;
    // return
    Ok(())
}

/// Find and import the pipeline structure.
fn import_pipeline(ip_context: &InterplanetaryContext) -> Result<Pipeline> {
    // quickly find the Pipeline block in the interplanetary area, and get its cid
    let pipeline_cid = find_pipeline_block(&ip_context)?;
    // parse the pipeline object
    let pipeline_block = sk_cbor::Value::read_from_ip_area(&pipeline_cid, &ip_context)?;
    Pipeline::try_from(*pipeline_block)
}

/// Recognize a Pipeline interplanetary block.
fn is_pipeline_block(path: &PathBuf) -> Result<bool> {
    const SUFFIX_LEN: usize = PIPELINE_BLOC_SUFFIX.len();
    // open the file
    if !path.is_file() { return Ok(false); }
    let mut f = File::open(path)?;
    // read the last bytes
    let len = f.seek(SeekFrom::End(0))?;
    if len < SUFFIX_LEN as u64 { return Ok(false); }
    f.seek(SeekFrom::End(0))?;
    f.seek(SeekFrom::Current(-{ SUFFIX_LEN as i64 }))?;
    let mut buffer = [0; SUFFIX_LEN];
    f.read_exact(&mut buffer)?;
    // compare it to Pipeline blocks suffix
    Ok(buffer == *PIPELINE_BLOC_SUFFIX)
}

/// Find the first Pipeline block in an interplanetary area
fn find_pipeline_block(ip_context: &InterplanetaryContext) -> Result<Cid> {
    let ip_area_path = &ip_context.ip_area_path;
    if ip_area_path.is_dir() {
        for first_level_entry in fs::read_dir(ip_area_path)? {
            let first_level_path = first_level_entry?.path();
            if first_level_path.is_dir() {
                for second_level_entry in fs::read_dir(first_level_path)? {
                    let second_level_path = second_level_entry?.path();
                    if second_level_path.is_file() {
                        if is_pipeline_block(&second_level_path)? {
                            // compute related block path
                            let cid = path_to_cid(&second_level_path, &ip_context)?;
                            return Ok(cid);
                        }
                    }
                }
            }
        }
    }
    Err(Error::FindToFindPipelineBlock.into())
}

/// Use the interplanetary context to import a pipeline vertices into the local area.
/// Returns a mapping from the interplanetary vertex index to its typed name, used as an index in
/// the local area.
fn import_vertices(ip_context: &InterplanetaryContext, local_context: &LocalContext, pipeline: &Pipeline) -> Result<VerticesImportKeyMap> {
    // loop through vertices to import sources, shapers and transformations
    let mut vertex_idx_mapping = VerticesImportKeyMap::new();
    for (idx, v) in pipeline.vertices.iter().enumerate() {
        // fetch and parse metadata
        let metadata_cid = &v.metadata.ok_or(Error::NoMetadataFoundInTheNode)?;
        let metadata_block = sk_cbor::Value::read_from_ip_area(&metadata_cid, &ip_context)?;
        let metadata = Metadata::try_from(*metadata_block)?;
        // get typed name
        let node_typed_name = metadata.name.as_ref().ok_or(Error::NoNameInMetadata)?;
        // map index in the list of vertices to this typed name
        vertex_idx_mapping.insert(idx as u64, node_typed_name.clone());
        // get node type and untyped name
        let (node_type, untyped_node_name) = parse_node_typed_name(&node_typed_name)?;
        // import node according to its type
        match node_type {
            NodeType::transformation => import_transformation(&ip_context, &local_context, &metadata, &untyped_node_name, &v)?,
            NodeType::source => import_source(&local_context, &metadata, &untyped_node_name)?,
            NodeType::shaper => import_shaper(&local_context, &metadata, &untyped_node_name)?,
        }
    }
    Ok(vertex_idx_mapping)
}

/// Use the interplanetary context to import a pipeline edges into the local area.
fn import_edges(
    ip_context: &InterplanetaryContext,
    local_context: &LocalContext,
    pipeline: &Pipeline,
    vertex_idx_mapping: &VerticesImportKeyMap,
) -> Result<()> {
    for e in &pipeline.edges {
        // parse connection block
        let connection_block = sk_cbor::Value::read_from_ip_area(&e.connection_cid, &ip_context)?;
        let connection = ConnectionBlock::try_from(*connection_block)?;
        // parse selectors' blocks
        let tail_selector = parse_selector(&connection.tail_selector, &ip_context)?;
        let head_selector = parse_selector(&connection.head_selector, &ip_context)?;
        // create connection id
        let tail_typed_name = vertex_idx_mapping.get(&e.tail_index).ok_or(Error::NoTailNameInEdge)?;
        let head_typed_name = vertex_idx_mapping.get(&e.head_index).ok_or(Error::NoHeadNameInEdge)?;
        let id = build_connection_id(tail_typed_name, head_typed_name);
        // create new object
        let object = Connection { id, tail_selector, head_selector };
        // store new object
        let encoded: Vec<u8> = bincode::serialize(&object)
            .context(BinCodeSerializeFailed)?;
        local_context.connections
            .compare_and_swap(&object.id, None as Option<&[u8]>, Some(encoded))
            .context(DbOperationFailed)?
            .ok()
            .context(anyhow!("cannot create connection: {}", &object.id))?;
    }
    Ok(())
}

/// Find and read a Selector from its CID in the interplanetary area and parse it to a String.
fn parse_selector(selector_cid: &Cid, ip_context: &InterplanetaryContext) -> Result<String> {
    let selector_block = sk_cbor::Value::read_from_ip_area(selector_cid, &ip_context)?;
    let selector = SelectorEnvelope::try_from(*selector_block)?.0;
    serde_json::to_string(&serde_json::Value::from(selector))
        .context(Error::FailedToParseSelector)
}

/// Use the interplanetary context to import a transformation, including its bytecode, into the
/// local area.
fn import_transformation(ip_context: &InterplanetaryContext, local_context: &LocalContext, metadata: &Metadata, node_name: &String, vertex: &PipelineVertex) -> Result<()> {
    // fetch and parse dry transformation
    let dry_transformation_cid = &vertex.dry_transformation.ok_or(Error::NoTransformationInNode)?;
    let dry_transformation_block = sk_cbor::Value::read_from_ip_area(&dry_transformation_cid, &ip_context)?;
    let dry_transformation = DryTransformation::try_from(*dry_transformation_block)?;
    // fetch and parse module bytecode envelope
    let module_bytecode_envelope_cid = &dry_transformation.module_bytecode_envelope_cid;
    let module_bytecode_envelope_block = sk_cbor::Value::read_from_ip_area(&module_bytecode_envelope_cid, &ip_context)?;
    let module_bytecode_envelope = ModuleBytecodeEnvelope::try_from(*module_bytecode_envelope_block)?;
    // fetch and parse module bytecode
    let module_bytecode_cid = &module_bytecode_envelope.module_bytecode_cid;
    let module_bytecode = ModuleBytecode::read_from_ip_area(&module_bytecode_cid, &ip_context)?;
    let json_schema_in = metadata.json_schema_in.as_ref().ok_or(Error::MissingSchemaInMetadata)?;
    let json_schema_out= metadata.json_schema_out.as_ref().ok_or(Error::MissingSchemaInMetadata)?;
    // create new object
    let object = Transformation {
        name: node_name.to_string(),
        bytecode: module_bytecode.bytecode.into_inner(),
        handle: dry_transformation.handle,
        json_schema_in: json_schema_in.to_string(),
        json_schema_out: json_schema_out.to_string(),
    };
    // store new object
    let encoded: Vec<u8> = bincode::serialize(&object)
        .context(BinCodeSerializeFailed)?;
    local_context.transformations
        .compare_and_swap(&object.name, None as Option<&[u8]>, Some(encoded))
        .context(DbOperationFailed)?
        .ok()
        .context(anyhow!("cannot create transformation with name: {}", &object.name))?;
    Ok(())
}

/// Use the interplanetary context to import a source pipeline node into the local area.
fn import_source(local_context: &LocalContext, metadata: &Metadata, node_name: &String) -> Result<()> {
    // create new object
    let json_schema= metadata.json_schema.as_ref().ok_or(Error::MissingSchemaInMetadata)?;
    let object = Source {
        name: node_name.to_string(),
        json_schema: json_schema.to_string(),
    };
    // store new object
    let encoded: Vec<u8> = bincode::serialize(&object)
        .context(BinCodeSerializeFailed)?;
    local_context.sources
        .compare_and_swap(&object.name, None as Option<&[u8]>, Some(encoded))
        .context(DbOperationFailed)?
        .ok()
        .context(anyhow!("cannot create source with name: {}", &object.name))?;
    Ok(())
}

/// Use the interplanetary context to a import shaper pipeline node into the local area.
fn import_shaper(local_context: &LocalContext, metadata: &Metadata, node_name: &String) -> Result<()> {
    // create new object
    let json_schema= metadata.json_schema.as_ref().ok_or(Error::MissingSchemaInMetadata)?;
    let object = Shaper {
        name: node_name.to_string(),
        json_schema: json_schema.to_string(),
    };
    // store new object
    let encoded: Vec<u8> = bincode::serialize(&object)
        .context(BinCodeSerializeFailed)?;
    local_context.shapers
        .compare_and_swap(&object.name, None as Option<&[u8]>, Some(encoded))
        .context(DbOperationFailed)?
        .ok()
        .context(anyhow!("cannot create shaper with name: {}", &object.name))?;
    Ok(())
}