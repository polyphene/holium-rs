use anyhow::Context;
use ellipse::Ellipse;
use humansize::{FileSize, file_size_opts};
use optional_struct::OptionalStruct;
use prettytable::{cell, row, Row, Table};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils::errors::Error::BinCodeSerializeFailed;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;

pub const TREE_NAME: &[u8] = b"transformation";

#[derive(Serialize, Deserialize, OptionalStruct)]
#[optional_derive(Serialize, Deserialize)]
pub struct Transformation {
    #[serde(skip)]
    pub name: String,
    pub bytecode: Vec<u8>,
    pub handle: String,
    pub json_schema_in: String,
}

pub fn merge(
    _key: &[u8],
    old_value: Option<&[u8]>,
    merged_bytes: &[u8],
) -> Option<Vec<u8>> {
    match old_value {
        None => Some(Vec::from(merged_bytes)),
        Some(old_bytes) => {
            let old_decoded: Transformation = bincode::deserialize(&old_bytes[..]).unwrap();
            let merged_decoded: OptionalTransformation = bincode::deserialize(&merged_bytes[..]).unwrap();
            let new_decoded = Transformation {
                name: merged_decoded.name.unwrap_or_else(|| old_decoded.name.clone()),
                bytecode: merged_decoded.bytecode.unwrap_or_else(|| old_decoded.bytecode.clone()),
                handle: merged_decoded.handle.unwrap_or_else(|| old_decoded.handle.clone()),
                json_schema_in: merged_decoded.json_schema_in.unwrap_or_else(|| old_decoded.json_schema_in.clone()),
            };
            let new_encoded = bincode::serialize(&new_decoded)
                .context(BinCodeSerializeFailed).ok()?;
            Some(new_encoded)
        }
    }
}

impl PrintableModel for Transformation {
    fn title_row() -> Row {
        row![
            b->"NAME",
            "HANDLE",
            "BYTECODE (size)",
            "IN (JSON Schema)",
        ]
    }

    fn object_to_row(&self) -> Row {
        row![
            b->self.name,
            self.handle,
            self.bytecode.len().file_size(file_size_opts::CONVENTIONAL).unwrap_or("".to_string()),
            json_schema_string_to_short_string(&self.json_schema_in),
        ]
    }
}

fn json_schema_string_to_short_string(json_schema_string: &str) -> String {
    // prettify using serde_json
    let json_value_result: Result<serde_json::Value, _> = serde_json::from_str(json_schema_string);
    match json_value_result {
        Err(_) => "".to_string(),
        Ok(json_value) => {
            let prettified_string = serde_json::to_string_pretty(&json_value).unwrap_or_default();
            let prettified = prettified_string.as_str();
            // truncate string
            prettified.truncate_ellipse(256).to_string()
        }
    }
}
