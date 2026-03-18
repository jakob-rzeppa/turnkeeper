use serde_json::Value;

/// Converts a `serde_valid` error string into a single formatted string summarizing validation errors.
///
/// The `serde_valid` crate emits validation errors as a JSON string with the
/// shape similar to:
///
/// {
///   "errors": [],
///   "properties": {
///     "name": {
///       "errors": ["The length of the value must be `>= 1`."]
///     }
///   }
/// }
///
/// This function parses that string and produces a flat string where each
/// property key is followed by its error messages, joined by spaces, and
/// multiple properties are separated by ` / `. For the example above the returned value will be:
///
/// "name: The length of the value must be `>= 1`."
///
/// Behavior:
/// - If the input string is valid JSON and contains a `properties` object,
///   each property's `errors` array (if present) will be joined into a string under the corresponding key.
/// - If parsing fails or no property errors are found, an empty string is returned.
///
/// Returns: `String` (formatted as "<field>: error1 error2 / <field2>: ...").
pub fn convert_serde_valid_error(error: &str) -> Result<String, String> {
    match serde_json::from_str::<Value>(error) {
        Ok(json) => {
            let mut parts = Vec::new();
            if let Some(props) = json.get("properties").and_then(|p| p.as_object()) {
                for (key, val) in props.iter() {
                    if let Some(errors) = val.get("errors").and_then(|e| e.as_array()) {
                        let msgs = errors
                            .iter()
                            .filter_map(|v| v.as_str())
                            .collect::<Vec<_>>();
                        if !msgs.is_empty() {
                            let joined = msgs.join(" ");
                            parts.push(format!("{}: {}", key, joined));
                        }
                    }
                }
            }
            Ok(parts.join(" / "))
        },
        Err(e) => Err(e.to_string()),
    }
}


#[cfg(test)]
mod tests {
    use serde_valid::Validate;
    use super::*;

    #[derive(Validate)]
    pub struct TestLoginRequest {
        #[validate(min_length = 1)]
        name: String,
        #[validate(min_length = 1)]
        #[validate(min_length = 5)]
        password: String,
    }

    #[test]
    fn test_single_field() {
        let test_login_request = TestLoginRequest {
            name: "".to_string(),
            password: "password".to_string(),
        };

        let err = test_login_request.validate().unwrap_err();

        let result = convert_serde_valid_error(&err.to_string()).unwrap();
        assert_eq!(result, "name: The length of the value must be `>= 1`.");
    }

    #[test]
    fn test_single_field_multiple_errors() {
        let test_login_request = TestLoginRequest {
            name: "name".to_string(),
            password: "".to_string()
        };

        let err = test_login_request.validate().unwrap_err();

        let result = convert_serde_valid_error(&err.to_string()).unwrap();
        assert_eq!(result, "password: The length of the value must be `>= 1`. The length of the value must be `>= 5`.");
    }

    #[test]
    fn test_multiple_fields() {
        let test_login_request = TestLoginRequest {
            name: "".to_string(),
            password: "pass".to_string()
        };

        let err = test_login_request.validate().unwrap_err();

        let result = convert_serde_valid_error(&err.to_string()).unwrap();
        assert_eq!(result, "name: The length of the value must be `>= 1`. / password: The length of the value must be `>= 5`.");
    }
}
