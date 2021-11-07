//! Helper methods related to JSON schema fields of local Holium objects.

use anyhow::{Result, Context};
use jsonschema::JSONSchema;
use serde_json::value::Value;
use serde_json::{json, Map};

lazy_static::lazy_static! {
    static ref META_SCHEMA: JSONSchema = {
        let json_meta_schema: Value = serde_json::from_str(include_str!("./assets/core-2020-12.schema.json"))
            .expect("invalid core JSON meta schema");
        JSONSchema::compile(&json_meta_schema)
            .expect("invalid core JSON meta schema")
    };
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("invalid json schema")]
    InvalidJsonSchema,
    #[error("a json schema should be a json object")]
    SchemaShouldBeJsonObject,
    #[error("type field missing in the json schema")]
    MissingTypeField,
    #[error("properties field missing in the json schema")]
    MissingPropertiesField,
    #[error("items or prefixItems field missing in the json schema")]
    MissingItemsField,
    #[error("type field value should be a string in a json schema")]
    TypeFieldShouldHoldStringValue,
    #[error("properties field value should be an object in a json schema")]
    PropertiesFieldShouldHoldObjectValue,
    #[error("items field value should be an object in a json schema")]
    ItemsFieldShouldHoldObjectValue,
    #[error("prefixItems field value should be an array in a json schema")]
    PrefixItemsFieldShouldHoldArrayValue,
    #[error("invalid json schema type field value: {0}")]
    InvalidTypeFieldValue(String),
}


pub struct HoliumJsonSchema(pub HoliumJsonSchemaName, pub Box<HoliumJsonSchemaType>);

pub struct HoliumJsonSchemaName(pub Option<String>);

pub enum HoliumJsonSchemaType {
    Object(Vec<HoliumJsonSchema>),
    TupleArray(Vec<HoliumJsonSchema>),
    ItemsArray(HoliumJsonSchema),
    ByteString,
    TextString,
    Number,
    Boolean,
    Null,
}

/// Validate that a JSON literal is a valid JSON Schema, ready to be used as a feature of local
/// Holium objects.
pub fn validate_json_schema(literal: &str) -> Result<()> {
    // parse the string of data into serde_json::Value
    let schema: Value = serde_json::from_str(literal)?;
    // validate it against JSON schema meta schema
    META_SCHEMA.validate(&schema)
        .ok()
        .context(Error::InvalidJsonSchema)?;
    // TODO change comment
    // recursively check that all expected fields are present in the schema for it to be used in the
    // local Holium area
    parse_root_json_schema(&schema)?;
    Ok(())
}

/// Parse a JSON schema from a JSON Value into a HoliumJsonSchema.
pub fn parse_root_json_schema(schema: &Value)  -> Result<HoliumJsonSchema> {
    parse_json_schema(HoliumJsonSchemaName(None), schema)
}

/// Check for the presence of fields in a JSON Schema necessary to their use in local Holium objects,
/// and parse them into a HoliumJsonSchema.
/// This function fails iff this condition is not satisfied.
fn parse_json_schema(schema_name: HoliumJsonSchemaName, schema: &Value) -> Result<HoliumJsonSchema> {
    // check that the value is an object
    let schema_map = match schema {
        Value::Object(schema_map) => schema_map,
        _ => {
            return Err(Error::SchemaShouldBeJsonObject.into());
        }
    };
    // check for the presence of a `type` field
    let type_value = schema_map.get("type")
        .ok_or(Error::MissingTypeField)?;
    let type_name = match type_value {
        Value::String(type_name) => type_name,
        _ => {
            return Err(Error::TypeFieldShouldHoldStringValue.into());
        }
    };
    // match scalar and recursive types
    match type_name.as_str() {
        "null" => Ok(HoliumJsonSchema(schema_name, Box::from(HoliumJsonSchemaType::Null))),
        "boolean" => Ok(HoliumJsonSchema(schema_name, Box::from(HoliumJsonSchemaType::Boolean))),
        "number" => Ok(HoliumJsonSchema(schema_name, Box::from(HoliumJsonSchemaType::Number))),
        "string" => {
            if has_base64_encoding(&schema_map) {
                Ok(HoliumJsonSchema(schema_name, Box::from(HoliumJsonSchemaType::ByteString)))
            } else {
                Ok(HoliumJsonSchema(schema_name, Box::from(HoliumJsonSchemaType::TextString)))
            }
        }
        "object" => Ok(HoliumJsonSchema(
            schema_name,
            Box::from(HoliumJsonSchemaType::Object(parse_object_properties(schema_map)?))
        )),
        "array" => {
            if is_tuples_array(&schema_map) {
                Ok(HoliumJsonSchema(
                    schema_name,
                    Box::from(HoliumJsonSchemaType::TupleArray(parse_tuples_array_items(schema_map)?))
                ))
            } else {
                Ok(HoliumJsonSchema(
                    schema_name,
                    Box::from(HoliumJsonSchemaType::ItemsArray(parse_items_array_item(schema_map)?))
                ))
            }
        },
        invalid_type => Err(Error::InvalidTypeFieldValue(invalid_type.to_string()).into()),
    }
}

fn has_base64_encoding(schema_map: &Map<String, Value>) -> bool {
    schema_map.get("contentEncoding")
        .map(|encoding| encoding == &Value::String("base64".to_string()))
        .unwrap_or(false)
}

fn is_tuples_array(schema_map: &Map<String, Value>) -> bool {
    schema_map.get("prefixItems").is_some()
}

fn parse_object_properties(schema_map: &Map<String, Value>) -> Result<Vec<HoliumJsonSchema>> {
    schema_map.get("properties")
        .ok_or(Error::MissingPropertiesField)?
        .as_object()
        .ok_or(Error::PropertiesFieldShouldHoldObjectValue)?
        .into_iter()
        .map(|(prop_name, prop_schema)| {
            let prop_schema_name = HoliumJsonSchemaName(Some(prop_name.to_string()));
            parse_json_schema(prop_schema_name, prop_schema)
        })
        .collect::<Result<Vec<HoliumJsonSchema>>>()
}

fn parse_tuples_array_items(schema_map: &Map<String, Value>) -> Result<Vec<HoliumJsonSchema>> {
    schema_map.get("prefixItems")
        .ok_or(Error::MissingItemsField)?
        .as_array()
        .ok_or(Error::PrefixItemsFieldShouldHoldArrayValue)?
        .into_iter()
        .map(parse_root_json_schema)
        .collect::<Result<Vec<HoliumJsonSchema>>>()
}

fn parse_items_array_item(schema_map: &Map<String, Value>) -> Result<HoliumJsonSchema> {
    let items_field = schema_map.get("items")
        .ok_or(Error::MissingItemsField)?;
    if !items_field.is_object() {
        return Err(Error::ItemsFieldShouldHoldObjectValue.into());
    }
    parse_root_json_schema(items_field)
}