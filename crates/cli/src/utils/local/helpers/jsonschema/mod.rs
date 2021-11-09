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
    #[error("invalid string can not be parsed to json")]
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
    #[error(r#"json schema of a pipeline node should hold an array of tuples ({{ "type" : "array" , "prefixItems" : … }}) at its root"#)]
    InvalidSchemaForPipelineNode,
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
/// Holium pipeline nodes.
pub fn validate_pipeline_node_json_schema(literal: &str) -> Result<()> {
    // parse the string of data into serde_json::Value
    let schema: Value = serde_json::from_str(literal).context(Error::StringNotParsableToJSON)?;
    // fastly validate it against JSON schema meta schema
    META_SCHEMA
        .validate(&schema)
        .ok()
        .context(Error::InvalidJsonSchema)?;
    // check that the root element has the right *tuples array* type
    validate_has_tuple_array_root(&schema)?;
    // recursively check that all expected fields are present in the schema for it to be used in the
    // local Holium area
    parse_root_json_schema(&schema)?;
    Ok(())
}

/// Check that the root element of a JSON schema is an array of *tuples*.
fn validate_has_tuple_array_root(schema: &Value)  -> Result<()> {
    // get type of the root schema
    let (type_name, schema_map) = get_schema_details(schema)?;
    // check that it is of *tuples array* type
    if type_name != "array" || !is_tuples_array(schema_map) {
        return Err(Error::InvalidSchemaForPipelineNode.into());
    }
    Ok(())
}

/// Parse a JSON schema from a JSON Value into a HoliumJsonSchema. The `root` term refers to the fact
/// that the schema itself is freed from any attached name, as a root JSON schema would.
pub fn parse_root_json_schema(schema: &Value)  -> Result<HoliumJsonSchema> {
    parse_json_schema(HoliumJsonSchemaName(None), schema)
}

/// Check for the presence of fields in a JSON Schema necessary to their use in local Holium objects,
/// and parse them into a HoliumJsonSchema.
/// This function fails iff this condition is not satisfied.
fn parse_json_schema(schema_name: HoliumJsonSchemaName, schema: &Value) -> Result<HoliumJsonSchema> {
    // get type of the root schema
    let (type_name, schema_map) = get_schema_details(schema)?;
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

/// Checks that a [serde_json::Value] is a JSON object coding for a schema, and returns the
/// corresponding the type of the schema, as a String, and the corresponding map of fields.
fn get_schema_details(schema: &Value) -> Result<(&String, &Map<String, Value>)> {
    // check that the value is an object
    let schema_map = match schema {
        Value::Object(schema_map) => schema_map,
        _ => return Err(Error::SchemaShouldBeJsonObject.into()),
    };
    // check for the presence of a `type` field
    let type_value = schema_map.get("type").ok_or(Error::MissingTypeField)?;
    match type_value {
        Value::String(type_name) => Ok((type_name, schema_map)),
        _ => return Err(Error::TypeFieldShouldHoldStringValue.into()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*******************************************
     * Validate expected fields
     *******************************************/

    #[test]
    fn cannot_validate_if_not_object() {
        let no_type_json = json!("only string");

        let res = parse_root_json_schema(&no_type_json);

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("a json schema should be a json object"));
    }

    #[test]
    fn cannot_validate_if_type_is_not_string() {
        let no_type_json = json!({ "type": 0 });

        let res = parse_root_json_schema(&no_type_json);

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("type field value should be a string in a json schema"));
    }

    #[test]
    fn cannot_validate_if_type_bad_value() {
        let non_valid_type = "non_valid";
        let no_type_json = json!({ "type": non_valid_type });

        let res = parse_root_json_schema(&no_type_json);

        assert!(res.is_err());
        assert!(res.err().unwrap().to_string().contains(
            format!("invalid json schema type field value: {}", non_valid_type).as_str()
        ));
    }

    #[test]
    fn can_validate_with_type_non_recursive() {
        let valid_types = vec!["null", "boolean", "number", "string"];

        for valid_type in valid_types {
            let json = json!({ "type": valid_type });

            parse_root_json_schema(&json).unwrap();
        }
    }

    #[test]
    fn can_validate_with_type_recursive() {
        // Validate on object
        let object_json = json!({ "type": "object", "properties": { "id": { "type": "string" }}});

        parse_root_json_schema(&object_json).unwrap();

        // Validate on array
        let object_json = json!({ "type": "array", "prefixItems": [{ "type": "string" }]});

        parse_root_json_schema(&object_json).unwrap();
    }

    /*******************************************
     * Validate expected fields on object type
     *******************************************/

    #[test]
    fn cannot_validate_type_object_if_no_properties() {
        let non_valid_json = json!({ "type": "object" });

        let res = match &non_valid_json {
            Value::Object(map) => parse_object_properties(map),
            _ => unreachable!(),
        };

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("properties field missing in the json schema"));
    }

    #[test]
    fn cannot_validate_type_object_if_properties_is_not_object() {
        let non_valid_json = json!({ "type": "object", "properties": "string" });

        let res = match &non_valid_json {
            Value::Object(map) => parse_object_properties(map),
            _ => unreachable!(),
        };

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("properties field value should be an object in a json schema"));
    }

    #[test]
    fn can_validate_proper_object() {
        let non_valid_json = json!({ "type": "object", "properties": { "id": {"type": "string"}} });

        let res = match &non_valid_json {
            Value::Object(map) => parse_object_properties(map).unwrap(),
            _ => unreachable!(),
        };
    }

    /*******************************************
     * Validate expected fields on array
     *******************************************/

    #[test]
    fn cannot_validate_if_no_items_or_prefix_items_in_schema() {
        let invalid_json = json!({"type": "array"});

        let res = match &invalid_json {
            Value::Object(map) => parse_items_array_item(map),
            _ => unreachable!(),
        };

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("items or prefixItems field missing in the json schema"));
    }

    #[test]
    fn cannot_validate_if_items_not_object() {
        let invalid_json = json!({"type": "array", "items": "string"});

        let res = match &invalid_json {
            Value::Object(map) => parse_items_array_item(map),
            _ => unreachable!(),
        };

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("items field value should be an object in a json schema"));
    }

    #[test]
    fn cannot_validate_if_prefix_items_not_array() {
        let invalid_json = json!({"type": "array", "prefixItems": "string"});

        let res = match &invalid_json {
            Value::Object(map) => parse_tuples_array_items(map),
            _ => unreachable!(),
        };

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("prefixItems field value should be an array in a json schema"));
    }

    #[test]
    fn can_validate_items() {
        let invalid_json = json!({"type": "array", "items": {"type": "string"}});

        let res = match &invalid_json {
            Value::Object(map) => parse_items_array_item(map).unwrap(),
            _ => unreachable!(),
        };
    }

    #[test]
    fn can_validate_prefix_items() {
        let invalid_json = json!({"type": "array", "prefixItems": [{"type": "string"}]});

        let res = match &invalid_json {
            Value::Object(map) => parse_tuples_array_items(map).unwrap(),
            _ => unreachable!(),
        };
    }

    /*******************************************
     * Validate json schema
     *******************************************/

    #[test]
    fn cannot_validate_non_json_string() {
        let non_json = "i am not a json";

        let res = validate_pipeline_node_json_schema(non_json);

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("invalid string can not be parsed to json"));
    }
}
