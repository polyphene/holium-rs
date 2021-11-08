use anyhow::{Result, Context};
use anyhow::Error as AnyhowError;
use std::io::Read;
use std::io::Write;
use serde_json::Map;
use serde_json::{Value as JsonValue, to_writer, Number};
use sk_cbor::Value as CborValue;
use sk_cbor::{cbor_null, cbor_bool, cbor_unsigned, cbor_int, cbor_text, cbor_bytes, cbor_array_vec};
use sk_cbor::write;
use sk_cbor::SimpleValue;
use crate::utils::local::models::data::HoliumCbor;
use crate::utils::local::helpers::jsonschema::{HoliumJsonSchema, HoliumJsonSchemaType, HoliumJsonSchemaName};
use crate::utils::repo::ports::formats::{FormatPorter, Error};

pub struct JsonPorter;

impl FormatPorter for JsonPorter {
    fn import_to_holium<R: Read, W: Write>(json_schema: &HoliumJsonSchema, reader: &mut R, writer: &mut W) -> Result<()> {
        // read the JSON contents
        let json_value: JsonValue = serde_json::from_reader(reader)
            .context(Error::FailedToReadJsonData)?;
        // convert the JSON value to HoliumCBOR
        let holium_cbor = import_value_to_holium(&json_schema, &json_value)?;
        // write the HoliumCBOR to the writer
        let mut buffer: Vec<u8> = Vec::new();
        write(holium_cbor, &mut buffer)
            .map_err(|_| Error::FailedToWriteHoliumCbor)?;
        writer.write_all(&buffer)
            .context(Error::FailedToWriteHoliumCbor)?;
        Ok(())
    }

    fn export_from_holium<R: Read, W: Write>(json_schema: &HoliumJsonSchema, reader: &mut R, writer: &mut W) -> Result<()> {
        // read the Holium CBOR contents
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let holium_cbor_value = sk_cbor::read(&buffer)
            .map_err(|_| Error::FailedToReadHoliumCborData)?;
        // convert the HoliumCBOR data to CBOR
        let contents = export_value_from_holium(&json_schema, &holium_cbor_value)?;
        // write the byte string to the writer
        to_writer(writer, &contents)
            .context(Error::FailedToWriteJsonData)?;
        Ok(())
    }
}

fn import_value_to_holium(json_schema: &HoliumJsonSchema, v: &JsonValue) -> Result<CborValue> {
    let boxed_schema = &json_schema.1;
    let schema: &HoliumJsonSchemaType = boxed_schema.as_ref();
    match (schema, v) {
        (HoliumJsonSchemaType::Null, _) => Ok(cbor_null!()),
        (HoliumJsonSchemaType::Boolean, JsonValue::Bool(v)) => Ok(cbor_bool!(*v)),
        (HoliumJsonSchemaType::Number, JsonValue::Number(v)) => {
            if let Some(v) = v.as_u64() {
                Ok(cbor_unsigned!(v))
            } else if let Some(v) = v.as_i64() {
                Ok(cbor_int!(v))
            } else {
                Err(Error::UnhandledFloat.into())
            }
        },
        (HoliumJsonSchemaType::TextString, JsonValue::String(v)) => Ok(cbor_text!(v.clone())),
        (HoliumJsonSchemaType::ByteString, JsonValue::String(v)) => {
            // decode base64-encoded string to Vec<u8>
            let bytes = base64::decode(&v).context("base64 decode error")?;
            Ok(cbor_bytes!(bytes))
        },
        (HoliumJsonSchemaType::ItemsArray(ref items_schema), JsonValue::Array(values)) => {
            let cbor_array = values
                .into_iter()
                .map(|v| import_value_to_holium(items_schema, &v))
                .collect::<Result<Vec<CborValue>>>()?;
            Ok(cbor_array_vec!(cbor_array))
        }
        (HoliumJsonSchemaType::TupleArray(ref tuple_schemata), JsonValue::Array(values)) => {
            let cbor_array = tuple_schemata
                .into_iter()
                .zip(values.iter())
                .map(|(schema, v)| import_value_to_holium(schema, &v))
                .collect::<Result<Vec<CborValue>>>()?;
            Ok(cbor_array_vec!(cbor_array))
        }
        (HoliumJsonSchemaType::Object(ref object_schemata), JsonValue::Object(values)) => {
            let cbor_array = object_schemata
                .into_iter()
                .map(|s| {
                    let key = s.0.0.as_ref().ok_or(Error::MissingKeyInObjectTypeSchema)?;
                    let value = values
                        .get(key)
                        .ok_or(Error::MissingObjectKey(key.to_string()))?;
                    let cbor_value = import_value_to_holium(
                        s,
                        value,
                    )?;
                    Ok(cbor_value)
                })
                .collect::<Result<Vec<(CborValue)>>>()?;
            Ok(cbor_array_vec!(cbor_array))
        }
        _ => Err(Error::IncompatibleSchemaAndValue.into()),
    }
}

fn export_value_from_holium(json_schema: &HoliumJsonSchema, v: &CborValue) -> Result<JsonValue> {
    let boxed_schema = &json_schema.1;
    let schema: &HoliumJsonSchemaType = boxed_schema.as_ref();
    match (schema, v) {
        (HoliumJsonSchemaType::Null, _) => Ok(JsonValue::Null),
        (HoliumJsonSchemaType::Boolean, CborValue::Simple(SimpleValue::TrueValue)) => Ok(JsonValue::Bool(true)),
        (HoliumJsonSchemaType::Boolean, CborValue::Simple(SimpleValue::FalseValue)) => Ok(JsonValue::Bool(false)),
        (HoliumJsonSchemaType::Number, CborValue::Unsigned(v)) => {
            Ok(JsonValue::Number(Number::from_f64(*v as f64).ok_or(Error::FailedToConvertNumberFromHoliumCborToJson)?))
        },
        (HoliumJsonSchemaType::Number, CborValue::Negative(v)) => {
            Ok(JsonValue::Number(Number::from_f64(*v as f64).ok_or(Error::FailedToConvertNumberFromHoliumCborToJson)?))
        },
        (HoliumJsonSchemaType::TextString, CborValue::TextString(v)) => Ok(JsonValue::String(v.clone())),
        (HoliumJsonSchemaType::ByteString, CborValue::ByteString(v)) => {
            // encode Vec<u8> to base64-encoded string
            let bytes = base64::encode(&v);
            Ok(JsonValue::String(bytes))
        },
        (HoliumJsonSchemaType::ItemsArray(ref items_schema), CborValue::Array(values)) => {
            let json_array = values
                .into_iter()
                .map(|v| export_value_from_holium(items_schema, &v))
                .collect::<Result<Vec<JsonValue>>>()?;
            Ok(JsonValue::Array(json_array))
        },
        (HoliumJsonSchemaType::TupleArray(ref tuple_schemata), CborValue::Array(values)) => {
            let json_array = tuple_schemata
                .into_iter()
                .zip(values.iter())
                .map(|(schema, v)| export_value_from_holium(schema, &v))
                .collect::<Result<Vec<JsonValue>>>()?;
            Ok(JsonValue::Array(json_array))
        }
        (HoliumJsonSchemaType::Object(ref object_schemata), CborValue::Array(values)) => {
            let mut cbor_map = Map::new();
            for (s, v) in object_schemata.into_iter().zip(values.iter()) {
                let key = s.0.0.as_ref().ok_or(Error::MissingKeyInObjectTypeSchema)?;
                let cbor_value = export_value_from_holium(s, v)?;
                cbor_map.insert(key.to_string(), cbor_value);
            }
            Ok(JsonValue::Object(cbor_map))
        }
        _ => Err(Error::IncompatibleSchemaAndValue.into()),
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_import_json_boolean_value() {
        let json_schema = HoliumJsonSchema(
            HoliumJsonSchemaName(None),
            Box::new(HoliumJsonSchemaType::Boolean),
        );
        let json_value = JsonValue::Bool(true);
        let holium_cbor = import_value_to_holium(&json_schema, &json_value).unwrap();
        assert_eq!(holium_cbor, cbor_bool!(true));
    }

    #[test]
    fn cannot_import_boolean_with_wrong_schema() {
        let json_schema = HoliumJsonSchema(
            HoliumJsonSchemaName(None),
            Box::new(HoliumJsonSchemaType::Number),
        );
        let json_value = JsonValue::Bool(true);
        let holium_cbor = import_value_to_holium(&json_schema, &json_value);
        assert!(holium_cbor.is_err());
    }

    #[test]
    fn can_import_json_object_value() {
        let json_schema = HoliumJsonSchema(
            HoliumJsonSchemaName(None),
            Box::new(HoliumJsonSchemaType::Object(vec![
                HoliumJsonSchema(HoliumJsonSchemaName(Some("key1".to_string())), Box::new(HoliumJsonSchemaType::Boolean)),
                HoliumJsonSchema(HoliumJsonSchemaName(Some("key0".to_string())), Box::new(HoliumJsonSchemaType::Number)),
            ])),
        );
        let data = r#"
        {
            "key0": 42,
            "key1": true
        }"#;
        // Parse the string of data into serde_json::Value.
        let json_value = serde_json::from_str(data).unwrap();
        let holium_cbor = import_value_to_holium(&json_schema, &json_value).unwrap();
        assert_eq!(holium_cbor, cbor_array_vec!(vec![
            cbor_bool!(true),
            cbor_unsigned!(42),
        ]));
    }

    #[test]
    fn can_import_json_bytes_value() {
        let json_schema = HoliumJsonSchema(
            HoliumJsonSchemaName(None),
            Box::new(HoliumJsonSchemaType::ByteString),
        );
        let data: Vec<u8> = vec![1,2,3];
        let json_value = JsonValue::String(base64::encode(data));
        let holium_cbor = import_value_to_holium(&json_schema, &json_value).unwrap();
        assert_eq!(holium_cbor, cbor_bytes!(vec![0x01, 0x02, 0x03]));
    }
}