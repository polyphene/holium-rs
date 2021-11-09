use anyhow::{Result, Context};
use anyhow::Error as AnyhowError;
use std::io::Read;
use std::io::Write;
use serde_json::Value as JsonValue;
use sk_cbor::Value as CborValue;
use sk_cbor::{cbor_null, cbor_bool, cbor_unsigned, cbor_int, cbor_text, cbor_bytes, cbor_array_vec};
use sk_cbor::write;
use crate::utils::local::models::data::HoliumCbor;
use crate::utils::local::helpers::jsonschema::{HoliumJsonSchema, HoliumJsonSchemaType, HoliumJsonSchemaName};
use crate::utils::repo::ports::formats::{FormatPorter, Error};

pub struct BinPorter;

impl FormatPorter for BinPorter {
    fn import_to_holium<R: Read, W: Write>(json_schema: &HoliumJsonSchema, reader: &mut R, writer: &mut W) -> Result<()> {
        // read the binary contents
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents)?;
        // check that the json schema is coherent
        validate_json_schema_for_bin_porter(&json_schema)?;
        // encode the binary contents as a cbor byte string and write it
        let holium_cbor = cbor_array_vec!(vec![cbor_bytes!(contents)]);    // todo: we could just compute the cbor headers, and prepend them to contents
        // write the HoliumCBOR to the writer
        let mut buffer: Vec<u8> = Vec::new();
        write(holium_cbor, &mut buffer)
            .map_err(|_| Error::FailedToWriteHoliumCbor)?;
        writer.write_all(&buffer)
            .context(Error::FailedToWriteHoliumCbor)?;
        Ok(())
    }
}

fn validate_json_schema_for_bin_porter(json_schema: &HoliumJsonSchema) -> Result<()> {
    let schema: &HoliumJsonSchemaType = &json_schema.1.as_ref();
    let tuples_array = match schema {
        HoliumJsonSchemaType::TupleArray(tuples_array) => tuples_array,
        _ => return Err(Error::IncompatibleSchemaAndValue.into())
    };
    let sub_schema = tuples_array.get(0).ok_or(Error::IncompatibleSchemaAndValue)?;
    let sub_schema: &HoliumJsonSchemaType = &sub_schema.1.as_ref();
    match sub_schema {
        HoliumJsonSchemaType::ByteString => {}
        _ => return Err(Error::IncompatibleSchemaAndValue.into())
    }
    Ok(())
}