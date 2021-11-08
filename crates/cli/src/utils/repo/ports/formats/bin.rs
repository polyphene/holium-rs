use anyhow::{Result, Context};
use anyhow::Error as AnyhowError;
use std::io::{Read, Cursor, copy};
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
        // check that the json schema is coherent
        validate_json_schema_for_bin_porter(&json_schema)?;
        // read the CBOR contents
        let mut contents = Vec::new();
        reader.read_to_end(&mut contents)?;
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

    fn export_from_holium<R: Read, W: Write>(json_schema: &HoliumJsonSchema, reader: &mut R, writer: &mut W) -> Result<()> {
        // check that the json schema is coherent
        validate_json_schema_for_bin_porter(&json_schema)?;
        // read the Holium CBOR contents
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let holium_cbor_value = sk_cbor::read(&buffer)
            .map_err(|_| Error::FailedToReadHoliumCborData)?;
        // get the expected inner byte string
        let tuples_array = match holium_cbor_value {
            CborValue::Array(tuples_array) => tuples_array,
            _ => return Err(Error::IncompatibleSchemaAndValue.into()),
        };
        let sub_value = tuples_array.get(0).ok_or(Error::IncompatibleSchemaAndValue)?;
        let mut bytes = match sub_value {
            CborValue::ByteString(bytes) => bytes,
            _ => return Err(Error::IncompatibleSchemaAndValue.into()),
        };
        // copy the byte string into the writer
        let mut readable_buffer = Cursor::new(bytes);
        copy(&mut readable_buffer, writer)
            .context(Error::FailedToWriteBinData)?;
        Ok(())
    }
}

fn validate_json_schema_for_bin_porter(json_schema: &HoliumJsonSchema) -> Result<()> {
    let boxed_schema = &json_schema.1;
    let schema: &HoliumJsonSchemaType = boxed_schema.as_ref();
    match schema {
        HoliumJsonSchemaType::TupleArray(tuples_array) => tuples_array,
        _ => return Err(Error::IncompatibleSchemaAndValue.into())
    };
    let tuples_array = match schema {
        HoliumJsonSchemaType::TupleArray(tuples_array) => tuples_array,
        _ => return Err(Error::IncompatibleSchemaAndValue.into())
    };
    let sub_schema = tuples_array.get(0).ok_or(Error::IncompatibleSchemaAndValue)?;
    let boxed_sub_schema = &sub_schema.1;
    let sub_schema: &HoliumJsonSchemaType = boxed_sub_schema.as_ref();
    match sub_schema {
        HoliumJsonSchemaType::ByteString => {}
        _ => return Err(Error::IncompatibleSchemaAndValue.into())
    }
    Ok(())
}