//! Helper methods related to selectors associated to local Holium connection objects.

use crate::utils::interplanetary::kinds::selector::SelectorEnvelope;
use anyhow::{Context, Result};
use ellipse::Ellipse;
use jsonschema::JSONSchema;
use serde_json::value::Value;
use serde_json::{json, Map};

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
    HOLIUM_SELECTOR_SCHEMA
        .validate(&instance)
        .ok()
        .context(Error::InvalidHoliumSelector)?;
    // further validate the literal by trying to parse it with the interplanetary kind
    SelectorEnvelope::new(literal).context(Error::InvalidHoliumSelector)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cannot_validate_non_json_string() {
        let non_json = "i am not a json";

        let res = validate_selector(non_json);

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("invalid string can not be parsed to json"));
    }

    #[test]
    fn cannot_validate_non_valid_json_object() {
        let non_json = "{\"non\":\"valid\"}";

        let res = validate_selector(non_json);

        assert!(res.is_err());
        assert!(res
            .err()
            .unwrap()
            .to_string()
            .contains("invalid holium selector"));
    }
}
