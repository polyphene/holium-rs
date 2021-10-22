use serde::{Serialize, Deserialize};
use optional_struct::OptionalStruct;
use crate::utils::errors::Error::BinCodeSerializeFailed;
use anyhow::Context;

pub const TREE_NAME: &[u8] = b"transformation";

#[derive(Serialize, Deserialize, OptionalStruct)]
#[optional_derive(Serialize, Deserialize)]
pub struct Transformation {
    pub handle: String,
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
            let new_decoded = Transformation{
                handle: merged_decoded.handle.unwrap_or_else(|| old_decoded.handle) //old_decoded.handle
            };
            let new_encoded = bincode::serialize(&new_decoded)
                .context(BinCodeSerializeFailed).ok()?;
            Some(new_encoded)
        }
    }
}
