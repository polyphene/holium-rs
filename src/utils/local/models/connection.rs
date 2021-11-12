use anyhow::Context;

use optional_struct::OptionalStruct;
use prettytable::{cell, row, Row};
use serde::{Deserialize, Serialize};

use crate::utils::errors::Error::BinCodeSerializeFailed;
use crate::utils::local::helpers::prints::json::shorten_prettify_json_literal;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;

pub const TREE_NAME: &[u8] = b"connection";

#[derive(Debug, Serialize, Deserialize, OptionalStruct)]
#[optional_derive(Serialize, Deserialize)]
pub struct Connection {
    #[serde(skip)]
    pub id: String,
    pub tail_selector: String,
    pub head_selector: String,
}

pub fn merge(_key: &[u8], old_value: Option<&[u8]>, merged_bytes: &[u8]) -> Option<Vec<u8>> {
    match old_value {
        None => Some(Vec::from(merged_bytes)),
        Some(old_bytes) => {
            let old_decoded: Connection = bincode::deserialize(&old_bytes[..]).unwrap();
            let merged_decoded: OptionalConnection =
                bincode::deserialize(&merged_bytes[..]).unwrap();
            let new_decoded = Connection {
                id: merged_decoded.id.unwrap_or_else(|| old_decoded.id.clone()),
                tail_selector: merged_decoded
                    .tail_selector
                    .unwrap_or_else(|| old_decoded.tail_selector.clone()),
                head_selector: merged_decoded
                    .head_selector
                    .unwrap_or_else(|| old_decoded.head_selector.clone()),
            };
            let new_encoded = bincode::serialize(&new_decoded)
                .context(BinCodeSerializeFailed)
                .ok()?;
            Some(new_encoded)
        }
    }
}

impl PrintableModel for Connection {
    fn title_row() -> Row {
        row![
            b->"ID",
            "TAIL SELECTOR (JSON Schema)",
            "HEAD SELECTOR (JSON Schema)",
        ]
    }

    fn object_to_row(&self) -> Row {
        row![
            b->self.id,
            shorten_prettify_json_literal(&self.tail_selector),
            shorten_prettify_json_literal(&self.head_selector),
        ]
    }
}
