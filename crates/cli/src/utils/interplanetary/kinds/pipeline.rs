use std::io::Cursor;
use cid::Cid;
use std::collections::HashMap;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use std::convert::{TryInto, TryFrom};
use sk_cbor::Value;
use sk_cbor::cbor_map;
use crate::utils::interplanetary::fs::traits::as_ip_block::AsInterplanetaryBlock;
use crate::utils::interplanetary::fs::constants::block_multicodec::BlockMulticodec;
use crate::utils::interplanetary::kinds::link::Link;
use crate::utils::interplanetary::kinds::pipeline_vertex::PipelineVertex;
use crate::utils::interplanetary::kinds::pipeline_edge::PipelineEdge;
use crate::utils::local::export::{VerticesKeyMap, VerticesContentMap};
use sk_cbor::cbor_array;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to create pipeline vertex")]
    FailedToCreatePipelineVertex,
    #[error("failed to manipulate pipeline kind")]
    FailedToManipulate,
}

static DISCRIMINANT_KEY_V0: &str = "pl_0";

pub struct Pipeline {
    pub vertices: Vec<PipelineVertex>,
    pub edges: Vec<PipelineEdge>,
}

impl Pipeline {
    pub(crate) fn new(
        vertices_key_mapping: &VerticesKeyMap,
        vertices_content: &VerticesContentMap,
        edges: Vec<PipelineEdge>,
    ) -> Result<Self> {
        // create an array of rightly sorted vertices
        let nb_nodes = vertices_key_mapping.len();
        let mut vertices = vec![PipelineVertex::default(); nb_nodes];
        for (typed_name, &idx) in vertices_key_mapping {
            let content = vertices_content.get(typed_name)
                .ok_or(Error::FailedToCreatePipelineVertex)?;
            std::mem::replace(&mut vertices[idx as usize], content.clone());
        }
        Ok(Pipeline{ vertices, edges })
    }
}

impl Pipeline {}

impl From<Pipeline> for sk_cbor::Value {
    fn from(o: Pipeline) -> Self {
        let vertices = sk_cbor::Value::Array(o.vertices.iter().map(|v| -> Value { v.clone().into() }).collect());
        let edges = sk_cbor::Value::Array(o.edges.iter().map(|v| -> Value { v.clone().into() }).collect());
        cbor_map! {
            "typedVersion" => DISCRIMINANT_KEY_V0,
            "content" => cbor_array![ vertices, edges ],
        }
    }
}

impl TryFrom<sk_cbor::Value> for Pipeline {
    type Error = AnyhowError;
    fn try_from(value: Value) -> Result<Self> {
        if let Value::Map(map) = value {
            if map.get(0).is_some() {
                let (k, content) = &map[0];
                if let Value::Array(tuple) = content {
                    if tuple.get(0..2).is_some() {
                        if let Value::Array(vertices_value) = &tuple[0] {
                            let vertices: Result<Vec<PipelineVertex>> = vertices_value
                                .iter()
                                .map(|v| -> Result<PipelineVertex> { v.clone().try_into() })
                                .collect();
                            let vertices = vertices?;
                            if let Value::Array(edges_value) = &tuple[1] {
                                let edges: Result<Vec<PipelineEdge>> = edges_value
                                    .iter()
                                    .map(|v| -> Result<PipelineEdge> { v.clone().try_into() })
                                    .collect();
                                let edges = edges?;
                                return Ok(Pipeline{ vertices, edges })
                            }
                        }
                    }
                }
            }
        }
        Err(Error::FailedToManipulate.into())
    }
}