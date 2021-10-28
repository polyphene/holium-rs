//! Helper methods related to the manipulation of JSON literals.

use ellipse::Ellipse;

/// Shorten and prettify a JSON literal
pub fn shorten_prettify_json_literal(json_string: &str) -> String {
    // prettify using serde_json
    let json_value_result: Result<serde_json::Value, _> = serde_json::from_str(json_string);
    match json_value_result {
        Err(_) => "".to_string(),
        Ok(json_value) => {
            let prettified_string = serde_json::to_string_pretty(&json_value).unwrap_or_default();
            let prettified = prettified_string.as_str();
            // truncate string
            prettified.truncate_ellipse(256).to_string()
        }
    }
}