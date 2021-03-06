use crate::utils::local::helpers::jsonschema::{HoliumJsonSchema, HoliumJsonSchemaType};

use crate::utils::repo::ports::formats::{Error, FormatPorter};

use anyhow::{Context, Result};
use sk_cbor::write;
use sk_cbor::{cbor_array_vec, cbor_map_collection};
use sk_cbor::{SimpleValue, Value};
use std::io::copy;
use std::io::Write;
use std::io::{Cursor, Read};

pub struct CborPorter;

impl FormatPorter for CborPorter {
    fn import_to_holium<R: Read, W: Write>(
        json_schema: &HoliumJsonSchema,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<()> {
        // read the CBOR contents
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let cbor_value = sk_cbor::read(&buffer).map_err(|_| Error::FailedToReadCborData)?;
        // convert the CBOR value to HoliumCBOR
        let holium_cbor = import_value_to_holium(&json_schema, &cbor_value)?;
        // write the HoliumCBOR to the writer
        let mut buffer: Vec<u8> = Vec::new();
        write(holium_cbor, &mut buffer).map_err(|_| Error::FailedToWriteHoliumCbor)?;
        writer
            .write_all(&buffer)
            .context(Error::FailedToWriteHoliumCbor)?;
        Ok(())
    }

    fn export_from_holium<R: Read, W: Write>(
        json_schema: &HoliumJsonSchema,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<()> {
        // read the Holium CBOR contents
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let holium_cbor_value =
            sk_cbor::read(&buffer).map_err(|_| Error::FailedToReadHoliumCborData)?;
        // convert the HoliumCBOR data to CBOR
        let contents = export_value_from_holium(&json_schema, &holium_cbor_value)?;
        // write the byte string to the writer
        let mut buffer = Vec::new();
        sk_cbor::writer::write(contents, &mut buffer).map_err(|_| Error::FailedToWriteCborData)?;
        let mut readable_buffer = Cursor::new(buffer);
        copy(&mut readable_buffer, writer).context(Error::FailedToWriteCborData)?;
        Ok(())
    }
}

fn import_value_to_holium(json_schema: &HoliumJsonSchema, v: &Value) -> Result<Value> {
    let schema: &HoliumJsonSchemaType = &json_schema.1.as_ref();
    match (schema, v) {
        (HoliumJsonSchemaType::Null, Value::Simple(SimpleValue::NullValue)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Boolean, Value::Simple(SimpleValue::TrueValue)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Boolean, Value::Simple(SimpleValue::FalseValue)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Number, Value::Negative(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Number, Value::Unsigned(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::TextString, Value::TextString(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::ByteString, Value::ByteString(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::ItemsArray(ref items_schema), Value::Array(values)) => {
            let holium_cbor_array = values
                .into_iter()
                .map(|v| import_value_to_holium(items_schema, &v))
                .collect::<Result<Vec<Value>>>()?;
            Ok(cbor_array_vec!(holium_cbor_array))
        }
        (HoliumJsonSchemaType::TupleArray(ref tuple_schemata), Value::Array(values)) => {
            let holium_cbor_array = tuple_schemata
                .into_iter()
                .zip(values.iter())
                .map(|(schema, v)| import_value_to_holium(schema, &v))
                .collect::<Result<Vec<Value>>>()?;
            Ok(cbor_array_vec!(holium_cbor_array))
        }
        (HoliumJsonSchemaType::Object(ref object_schemata), Value::Map(values)) => {
            let holium_cbor_array = object_schemata
                .into_iter()
                .map(|s| {
                    let key = s.0 .0.as_ref().ok_or(Error::MissingKeyInObjectTypeSchema)?;
                    let cbor_key = Value::TextString(key.to_string());
                    // get element from the list of tuples which matches the key
                    let value = &values
                        .iter()
                        .find(|(k, _)| *k == cbor_key)
                        .ok_or(Error::MissingObjectKey(key.to_string()))?
                        .1
                        .clone();
                    let holium_cbor_value = import_value_to_holium(s, value)?;
                    Ok(holium_cbor_value)
                })
                .collect::<Result<Vec<Value>>>()?;
            Ok(cbor_array_vec!(holium_cbor_array))
        }
        _ => Err(Error::IncompatibleSchemaAndValue.into()),
    }
}

fn export_value_from_holium(json_schema: &HoliumJsonSchema, v: &Value) -> Result<Value> {
    let boxed_schema = &json_schema.1;
    let schema: &HoliumJsonSchemaType = boxed_schema.as_ref();
    match (schema, v) {
        (HoliumJsonSchemaType::Null, Value::Simple(SimpleValue::NullValue)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Boolean, Value::Simple(SimpleValue::TrueValue)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Boolean, Value::Simple(SimpleValue::FalseValue)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Number, Value::Negative(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::Number, Value::Unsigned(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::TextString, Value::TextString(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::ByteString, Value::ByteString(_)) => Ok(v.clone()),
        (HoliumJsonSchemaType::ItemsArray(ref items_schema), Value::Array(values)) => {
            let holium_cbor_array = values
                .into_iter()
                .map(|v| export_value_from_holium(items_schema, &v))
                .collect::<Result<Vec<Value>>>()?;
            Ok(cbor_array_vec!(holium_cbor_array))
        }
        (HoliumJsonSchemaType::TupleArray(ref tuple_schemata), Value::Array(values)) => {
            let holium_cbor_array = tuple_schemata
                .into_iter()
                .zip(values.iter())
                .map(|(schema, v)| export_value_from_holium(schema, &v))
                .collect::<Result<Vec<Value>>>()?;
            Ok(cbor_array_vec!(holium_cbor_array))
        }
        (HoliumJsonSchemaType::Object(ref object_schemata), Value::Array(values)) => {
            let cbor_map = object_schemata
                .into_iter()
                .zip(values.iter())
                .map(|(schema, value)| {
                    // build text key
                    let key = schema
                        .0
                         .0
                        .as_ref()
                        .ok_or(Error::MissingKeyInObjectTypeSchema)?;
                    let cbor_key = Value::TextString(key.to_string());
                    // get element
                    let cbor_value = export_value_from_holium(schema, value)?;
                    Ok((cbor_key, cbor_value))
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;
            Ok(cbor_map_collection!(cbor_map))
        }
        _ => Err(Error::IncompatibleSchemaAndValue.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::local::helpers::jsonschema::HoliumJsonSchemaName;
    use sk_cbor::cbor_bool;
    use sk_cbor::cbor_unsigned;

    #[test]
    fn can_import_cbor_boolean_value() {
        let json_schema = HoliumJsonSchema(
            HoliumJsonSchemaName(None),
            Box::from(HoliumJsonSchemaType::Boolean),
        );
        let cbor_value = Value::Simple(SimpleValue::TrueValue);
        let holium_cbor = import_value_to_holium(&json_schema, &cbor_value).unwrap();
        assert_eq!(holium_cbor, Value::Simple(SimpleValue::TrueValue));
    }

    #[test]
    fn can_import_cbor_map_value() {
        let json_schema = HoliumJsonSchema(
            HoliumJsonSchemaName(None),
            Box::new(HoliumJsonSchemaType::Object(vec![
                HoliumJsonSchema(
                    HoliumJsonSchemaName(Some("key1".to_string())),
                    Box::new(HoliumJsonSchemaType::Boolean),
                ),
                HoliumJsonSchema(
                    HoliumJsonSchemaName(Some("key0".to_string())),
                    Box::new(HoliumJsonSchemaType::Number),
                ),
            ])),
        );
        let cbor_value = Value::Map(vec![
            (
                Value::TextString("key0".to_string()),
                Value::Unsigned(42u64),
            ),
            (
                Value::TextString("key1".to_string()),
                Value::Simple(SimpleValue::TrueValue),
            ),
        ]);
        let holium_cbor = import_value_to_holium(&json_schema, &cbor_value).unwrap();
        assert_eq!(
            holium_cbor,
            cbor_array_vec!(vec![cbor_bool!(true), cbor_unsigned!(42),])
        );
    }
}
