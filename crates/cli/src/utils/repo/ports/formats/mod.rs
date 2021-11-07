use anyhow::Result;
use std::io::Read;
use std::io::Write;
use crate::utils::local::helpers::jsonschema::HoliumJsonSchema;

pub mod json;
pub mod cbor;
pub mod bin;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to read json data")]
    FailedToReadJsonData,
    #[error("failed to read cbor data")]
    FailedToReadCborData,
    #[error("float value not handled yet in HoliumCBOR data")]
    UnhandledFloat,
    #[error("base64 decode error")]
    Base64DecodeError,
    #[error("invalid schema: missing key in schema of object type")]
    MissingKeyInObjectTypeSchema,
    #[error("missing key in imported object: {0}")]
    MissingObjectKey(String),
    #[error("schema and value are incompatible")]
    IncompatibleSchemaAndValue,
    #[error("failed to write HoliumCBOR data")]
    FailedToWriteHoliumCbor,
}

// trait FormatPorter with import_to_holium and export_from_holium
pub trait FormatPorter {
    fn import_to_holium<R: Read, W: Write>(json_schema: &HoliumJsonSchema, reader: &mut R, writer: &mut W) -> Result<()>;
}