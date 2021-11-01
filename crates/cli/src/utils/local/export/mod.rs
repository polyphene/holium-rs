use anyhow::{Context, Result};

use crate::utils::local::context::LocalContext;
use crate::utils::errors::Error::{DbOperationFailed, BinCodeDeserializeFailed};
use crate::utils::local::context::helpers::db_key_to_str;
use crate::utils::local::models::transformation::Transformation;
use std::collections::HashMap;
use cid::Cid;
use crate::utils::interplanetary::kinds;
use std::convert::TryFrom;
use crate::utils::interplanetary::kinds::module_bytecode_envelope::ModuleBytecodeEnvelope;
use sk_cbor::Value;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::kinds::module_bytecode::ModuleBytecode;
use crate::utils::interplanetary::kinds::dry_transformation::DryTransformation;

/// [ VertexContent ] holds the Map used as an attribute of vertices of a pipeline graph.
pub struct VertexContent {
    dry_transformation: Option<Cid>,
    metadata: Option<Cid>,
}

/// [ VerticesContentMap ] is used to map nodes' name to their content while constructing the
/// interplanetary representation of a pipeline.
type VerticesContentMap = HashMap<String, VertexContent>;

pub fn export_project(context: &LocalContext) -> Result<Cid> {
    // initialize an object to store the content of the graph nodes
    let mut vertices_content = VerticesContentMap::new();
    // export dry transformations
    export_dry_transformations(&context, &mut vertices_content)?;
    // export metadata
    todo!();
    // export connections
    todo!();
    // export the pipeline itself, and return its cid
    todo!()
}

fn export_dry_transformations(local_context: &LocalContext, vertices_content: &mut VerticesContentMap) -> Result<()> {
    // TODO we could use multiple threads here
    for o in local_context
        .transformations
        .iter() {
        // decode the ob
        let (name_vec, encoded) = o.context(DbOperationFailed)?;
        let name = db_key_to_str(name_vec)?;
        let decoded: Transformation = bincode::deserialize(&encoded[..])
            .ok()
            .context(BinCodeDeserializeFailed)?;
        // store the bytecode
        let mut module_bytecode = ModuleBytecode::new(decoded.bytecode);
        let module_bytecode_cid = &module_bytecode.write_to_ip_area(&local_context)?;
        // store the module bytecode envelope
        let module_bytecode_envelope = ModuleBytecodeEnvelope::new(&module_bytecode_cid);
        let module_bytecode_envelope_cid = Value::from(module_bytecode_envelope).write_to_ip_area(&local_context)?;
        // store the dry transformation
        let dry_transformation = DryTransformation::new(&module_bytecode_envelope_cid, &decoded.handle);
        let dry_transformation_cid = Value::from(dry_transformation).write_to_ip_area(&local_context)?;
        // add it to the vertices context map
        todo!();
    }
    Ok(())
}