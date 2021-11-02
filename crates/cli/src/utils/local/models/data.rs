use anyhow::Context;
use humansize::{FileSize, file_size_opts};
use optional_struct::OptionalStruct;
use prettytable::{cell, row, Row, Table};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils::errors::Error::BinCodeSerializeFailed;
use crate::utils::local::helpers::prints::printable_model::PrintableModel;
use crate::utils::local::helpers::prints::json::shorten_prettify_json_literal;

pub const TREE_NAME: &[u8] = b"data";
pub type HoliumCbor = Vec<u8>;

#[derive(Serialize, Deserialize, OptionalStruct)]
#[optional_derive(Serialize, Deserialize)]
pub struct HoliumData {
    #[serde(skip)]
    pub content: HoliumCbor,
}

pub fn merge(
    _key: &[u8],
    old_value: Option<&[u8]>,
    merged_bytes: &[u8],
) -> Option<Vec<u8>> {
    match old_value {
        None => Some(Vec::from(merged_bytes)),
        Some(old_bytes) => {
            let old_decoded: HoliumData = bincode::deserialize(&old_bytes[..]).unwrap();
            let merged_decoded: OptionalShaper = bincode::deserialize(&merged_bytes[..]).unwrap();
            let new_decoded = HoliumData {
                content: merged_decoded.name.unwrap_or_else(|| old_decoded.content.clone()),
            };
            let new_encoded = bincode::serialize(&new_decoded)
                .context(BinCodeSerializeFailed).ok()?;
            Some(new_encoded)
        }
    }
}