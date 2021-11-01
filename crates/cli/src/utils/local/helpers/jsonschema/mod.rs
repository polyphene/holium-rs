//! Helper methods related to JSON schema fields of local Holium objects.

use anyhow::{Context, Result};
use ellipse::Ellipse;
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
    #[error("invalid string can not be passed to json")]
    StringNotParsableToJSON,
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

/// Validate that a JSON literal is a valid JSON Schema, ready to be used as a feature of local
/// Holium objects.
pub fn validate_json_schema(literal: &str) -> Result<()> {
    // parse the string of data into serde_json::Value
    let schema: Value = serde_json::from_str(literal).context(Error::StringNotParsableToJSON)?;
    // validate it against JSON schema meta schema
    META_SCHEMA
        .validate(&schema)
        .ok()
        .context(Error::InvalidJsonSchema)?;
    // recursively check that all expected fields are present in the schema for it to be used in the
    // local Holium area
    check_expected_fields(&schema)?;
    Ok(())
}

/// Check for the presence of fields in a JSON Schema necessary to their use in local Holium objects.
/// This function fails iff this condition is not satisfied.
fn check_expected_fields(schema: &Value) -> Result<()> {
    // check that the value is an object
    let schema_map = match schema {
        Value::Object(schema_map) => schema_map,
        _ => return Err(Error::SchemaShouldBeJsonObject.into()),
    };
    // check for the presence of a `type` field
    let type_value = schema_map.get("type").ok_or(Error::MissingTypeField)?;
    let type_name = match type_value {
        Value::String(type_name) => type_name,
        _ => return Err(Error::TypeFieldShouldHoldStringValue.into()),
    };
    // match scalar and recursive types
    match type_name.as_str() {
        "null" | "boolean" | "number" | "string" => Ok(()),
        "object" => check_expected_fields_in_object_typed_value(schema_map),
        "array" => check_expected_fields_in_array_typed_value(schema_map),
        invalid_type => Err(Error::InvalidTypeFieldValue(invalid_type.to_string()).into()),
    }
}

fn check_expected_fields_in_object_typed_value(schema_map: &Map<String, Value>) -> Result<()> {
    // check for the presence of a `properties` field
    let properties_value = schema_map
        .get("properties")
        .ok_or(Error::MissingPropertiesField)?;
    let properties_map = match properties_value {
        Value::Object(properties_map) => properties_map,
        _ => return Err(Error::PropertiesFieldShouldHoldObjectValue.into()),
    };
    // recursively check properties' schemata
    let properties: Vec<&Value> = properties_map.values().collect();
    properties
        .into_iter()
        .map(|v| check_expected_fields(v))
        .collect()
}

fn check_expected_fields_in_array_typed_value(schema_map: &Map<String, Value>) -> Result<()> {
    // check for the presence of an `items` or `prefixItems` field
    if let Some(items_value) = schema_map.get("items") {
        // recursively check the items' schema
        if !items_value.is_object() {
            // this test may run twice (here and in the recursive call)
            return Err(Error::ItemsFieldShouldHoldObjectValue.into());
        }
        check_expected_fields(items_value)
    } else if let Some(prefix_items_value) = schema_map.get("prefixItems") {
        // recursively check the items' schemata
        let prefix_items = match prefix_items_value {
            Value::Array(prefix_items) => prefix_items,
            _ => return Err(Error::PrefixItemsFieldShouldHoldArrayValue.into()),
        };
        prefix_items
            .into_iter()
            .map(|v| check_expected_fields(v))
            .collect()
    } else {
        return Err(Error::MissingItemsField.into());
    }
}
