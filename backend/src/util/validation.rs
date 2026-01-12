use serde_json::Value;

/// Converts a `serde_valid` error string into a flattened JSON object mapping
/// property names to arrays of error messages.
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
/// This function parses that string and produces a simpler structure where each
/// property key maps directly to an array of error strings. For the example
/// above the returned value will be:
///
/// {
///   "name": ["The length of the value must be `>= 1`."]
/// }
///
/// Behavior:
/// - If the input string is valid JSON and contains a `properties` object,
///   each property's `errors` array (if present) will be flattened into a
///   `String` array under the corresponding key.
/// - If parsing fails or no property errors are found, an empty JSON object
///   (`{}`) is returned.
///
/// Returns: `serde_json::Value` (always an `Object` when property errors exist,
/// otherwise an empty object).
pub fn convert_serde_valid_error(error: &str) -> Result<Value, String> {
    // Try to parse the error string as JSON
    match serde_json::from_str::<Value>(error) {
        Ok(json) => {
            // Create a new map to hold the flattened errors
            let mut flat = serde_json::Map::new();
            // Look for the "properties" object in the parsed JSON
            if let Some(props) = json.get("properties").and_then(|p| p.as_object()) {
                // Iterate over each property in "properties"
                for (key, val) in props.iter() {
                    // For each property, get its "errors" array if present
                    if let Some(errors) = val.get("errors").and_then(|e| e.as_array()) {
                        // Convert each error value to a string and collect into a Vec
                        let arr = errors.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>();
                        // Insert the array of error strings under the property key
                        flat.insert(key.clone(), Value::Array(arr.into_iter().map(Value::String).collect()));
                    }
                }
            }
            // Return the flattened error map as a JSON object
            Ok(Value::Object(flat))
        },
        // If parsing fails, return an empty JSON object
        Err(e) => Err(e.to_string()),
    }
}
