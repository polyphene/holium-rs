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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cannot_prettify_non_json() {
        let string = "non_json";

        // Try to format a no JSON str
        let result_string = shorten_prettify_json_literal(string);

        assert_eq!(String::new(), result_string);
    }

    #[test]
    fn can_prettify_json() {
        // Non prettify JSON
        let string = "{\"type\": \"array\", \"prefixItems\": [{\"type\": \"string\"}]}";
        // We add indentation and return to line to have a pretty formatting
        let expected_result = "{\n  \"type\": \"array\",\n  \"prefixItems\": [\n    {\n      \"type\": \"string\"\n    }\n  ]\n}";

        // Format JSON
        let result_string = shorten_prettify_json_literal(string);

        assert_eq!(expected_result, result_string.as_str());
    }

    #[test]
    fn can_truncate_long_json() {
        let string = "{\"type\": \"array\", \"prefixItems\": [{\"type\": \"string\"}, \
        {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"},\
        {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"},\
        {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"},\
        {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"}, {\"type\": \"string\"}]}";

        // Format JSON
        let result_string = shorten_prettify_json_literal(string);

        // 256 characters of string and 3 of "..."
        assert_eq!(259, result_string.len());
    }
}
