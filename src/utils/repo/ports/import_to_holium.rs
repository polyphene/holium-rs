use crate::utils::local::context::LocalContext;

use crate::utils::repo::helpers::get_root_path;
use crate::utils::repo::models::portation::{Portation, PortationFileFormat};
use crate::utils::repo::ports::formats::bin::BinPorter;
use crate::utils::repo::ports::formats::cbor::CborPorter;
use crate::utils::repo::ports::formats::json::JsonPorter;
use crate::utils::repo::ports::formats::FormatPorter;
use crate::utils::repo::ports::helpers::get_portation_json_schema;
use anyhow::{bail, Context, Result};

use std::io::{BufReader, Write};

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to open file {0} for portation {1}")]
    FailedToOpenFile(String, String),
}

/// Read data from a Portation and write it as HoliumCBOR data.
pub fn import_to_holium<W: Write>(
    local_context: &LocalContext,
    portation: &Portation,
    writer: &mut W,
) -> Result<()> {
    // get json schema from the portation
    let json_schema = get_portation_json_schema(&local_context, &portation)?;
    // open file from its path
    let path = get_root_path()?.join(&portation.file_path);
    let file = std::fs::File::open(&path).context(Error::FailedToOpenFile(
        path.file_name()
            .map(|oss| oss.to_string_lossy().to_string())
            .unwrap_or("".to_string()),
        portation.id.clone(),
    ))?;
    let mut reader = BufReader::new(file);
    // parse data in Holium CBOR format
    match portation.file_format {
        PortationFileFormat::bin => BinPorter::import_to_holium(&json_schema, &mut reader, writer),
        PortationFileFormat::cbor => {
            CborPorter::import_to_holium(&json_schema, &mut reader, writer)
        }
        PortationFileFormat::csv => bail!("import from CSV format is not supported yet"),
        PortationFileFormat::json => {
            JsonPorter::import_to_holium(&json_schema, &mut reader, writer)
        }
    }?;
    Ok(())
}
