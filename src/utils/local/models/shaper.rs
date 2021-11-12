use anyhow::Context;

use optional_struct::OptionalStruct;
use prettytable::{cell, row, Row};
use serde::{Deserialize, Serialize};

use crate::utils::errors::Error::BinCodeSerializeFailed;
use crate::utils::local::helpers::prints::json::shorten_prettify_json_literal;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;

pub const TREE_NAME: &[u8] = b"shaper";

#[derive(Serialize, Deserialize, OptionalStruct)]
#[optional_derive(Serialize, Deserialize)]
pub struct Shaper {
    #[serde(skip)]
    pub name: String,
    pub json_schema: String,
}

pub fn merge(_key: &[u8], old_value: Option<&[u8]>, merged_bytes: &[u8]) -> Option<Vec<u8>> {
    match old_value {
        None => Some(Vec::from(merged_bytes)),
        Some(old_bytes) => {
            let old_decoded: Shaper = bincode::deserialize(&old_bytes[..]).unwrap();
            let merged_decoded: OptionalShaper = bincode::deserialize(&merged_bytes[..]).unwrap();
            let new_decoded = Shaper {
                name: merged_decoded
                    .name
                    .unwrap_or_else(|| old_decoded.name.clone()),
                json_schema: merged_decoded
                    .json_schema
                    .unwrap_or_else(|| old_decoded.json_schema.clone()),
            };
            let new_encoded = bincode::serialize(&new_decoded)
                .context(BinCodeSerializeFailed)
                .ok()?;
            Some(new_encoded)
        }
    }
}

impl PrintableModel for Shaper {
    fn title_row() -> Row {
        row![
            b->"NAME",
            "JSON Schema",
        ]
    }

    fn object_to_row(&self) -> Row {
        row![
            b->self.name,
            shorten_prettify_json_literal(&self.json_schema),
        ]
    }
}
