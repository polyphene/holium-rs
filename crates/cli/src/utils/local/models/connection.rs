use anyhow::Context;
use humansize::{FileSize, file_size_opts};
use optional_struct::OptionalStruct;
use prettytable::{cell, row, Row, Table};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils::errors::Error::BinCodeSerializeFailed;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::helpers::jsonschema::json_schema_string_to_short_string;

pub const TREE_NAME: &[u8] = b"connection";

#[derive(Serialize, Deserialize, OptionalStruct)]
#[optional_derive(Serialize, Deserialize)]
pub struct Connection {
    #[serde(skip)]
    pub id: String,
}

pub fn merge(
    _key: &[u8],
    old_value: Option<&[u8]>,
    merged_bytes: &[u8],
) -> Option<Vec<u8>> {
    match old_value {
        None => Some(Vec::from(merged_bytes)),
        Some(old_bytes) => {
            let old_decoded: Connection = bincode::deserialize(&old_bytes[..]).unwrap();
            let merged_decoded: OptionalConnection = bincode::deserialize(&merged_bytes[..]).unwrap();
            let new_decoded = Connection {
                id: merged_decoded.id.unwrap_or_else(|| old_decoded.id.clone()),
            };
            let new_encoded = bincode::serialize(&new_decoded)
                .context(BinCodeSerializeFailed).ok()?;
            Some(new_encoded)
        }
    }
}

impl PrintableModel for Connection {
    fn title_row() -> Row {
        row![
            b->"ID",
        ]
    }

    fn object_to_row(&self) -> Row {
        row![
            b->self.id,
        ]
    }
}
