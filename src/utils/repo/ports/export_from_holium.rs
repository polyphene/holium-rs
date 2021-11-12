use crate::utils::errors::Error::{
    BinCodeDeserializeFailed, DbOperationFailed, NoObjectForGivenKey,
};
use crate::utils::local::context::helpers::{
    parse_node_typed_name, parse_portation_id, NodeType, PortationDirectionType,
};
use crate::utils::local::context::LocalContext;
use crate::utils::local::helpers::jsonschema::{parse_root_json_schema, HoliumJsonSchema};
use crate::utils::local::models::shaper::Shaper;
use crate::utils::local::models::source::Source;
use crate::utils::local::models::transformation::Transformation;
use crate::utils::repo::context::RepositoryContext;
use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::models::portation::{Portation, PortationFileFormat};
use crate::utils::repo::ports::formats::bin::BinPorter;
use crate::utils::repo::ports::formats::cbor::CborPorter;
use crate::utils::repo::ports::formats::json::JsonPorter;
use crate::utils::repo::ports::formats::FormatPorter;
use crate::utils::repo::ports::helpers::get_portation_json_schema;
use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::io::{BufReader, Read, Write};
use std::path::Path;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to create file {0} for portation {1}")]
    FailedToCreateFile(String, String),
}

/// Read Holium CBOR data from a Portation and write it in a file in the repository.
pub fn export_from_holium<R: Read>(
    local_context: &LocalContext,
    portation: &Portation,
    reader: &mut R,
) -> Result<()> {
    // get json schema from the portation
    let json_schema = get_portation_json_schema(&local_context, &portation)?;
    // open file from its path
    let path = get_root_path()?.join(&portation.file_path);
    let mut file = std::fs::File::create(&path).context(Error::FailedToCreateFile(
        path.file_name()
            .map(|oss| oss.to_string_lossy().to_string())
            .unwrap_or("".to_string()),
        portation.id.clone(),
    ))?;
    // parse Holium CBOR data into the right format and write it
    match portation.file_format {
        PortationFileFormat::bin => BinPorter::export_from_holium(&json_schema, reader, &mut file),
        PortationFileFormat::cbor => {
            CborPorter::export_from_holium(&json_schema, reader, &mut file)
        }
        PortationFileFormat::csv => bail!("export to CSV format is not supported yet"),
        PortationFileFormat::json => {
            JsonPorter::export_from_holium(&json_schema, reader, &mut file)
        }
    }?;
    Ok(())
}
