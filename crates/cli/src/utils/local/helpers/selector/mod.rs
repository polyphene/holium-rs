//! Helper methods related to selectors associated to local Holium connection objects.

use anyhow::{Result, Context};
use jsonschema::JSONSchema;
use serde_json::value::Value;
use serde_json::{json, Map};
use ellipse::Ellipse;

lazy_static::lazy_static! {
    static ref HOLIUM_SELECTOR_SCHEMA: JSONSchema = {
        let json_schema: Value = serde_json::from_str(include_str!("./assets/schema.json"))
            .expect("invalid Holium selector schema");
        JSONSchema::compile(&json_schema)
            .expect("invalid Holium selector schema")
    };
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("invalid string can not be parsed to json")]
    StringNotParsableToJSON,
    #[error("invalid holium selector")]
    InvalidHoliumSelector,
}

/// Validate a Holium selector JSON instance against the reference JSON Schema.
pub fn validate_selector(literal: &str) -> Result<()> {
    // parse the instance literal into serde_json::Value
    let instance: Value = serde_json::from_str(literal).context(Error::StringNotParsableToJSON)?;
    // validate it against Holium selector schema
    HOLIUM_SELECTOR_SCHEMA.validate(&instance)
        .ok()
        .context(Error::InvalidHoliumSelector)
}