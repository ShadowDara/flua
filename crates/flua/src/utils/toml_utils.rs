use mlua::{Error, Result};
use serde_json::Value as JsonValue;

// TODO
// Probably fix this
// Toml Parser
pub fn json_to_toml(value: JsonValue) -> Result<toml::Value> {
    match value {
        JsonValue::Null => Ok(toml::Value::String("".to_string())),
        JsonValue::Bool(b) => Ok(toml::Value::Boolean(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(Error::external("Invalid number"))
            }
        }
        JsonValue::String(s) => Ok(toml::Value::String(s)),
        JsonValue::Array(arr) => {
            let mut toml_arr = Vec::new();
            for item in arr {
                toml_arr.push(json_to_toml(item)?);
            }
            Ok(toml::Value::Array(toml_arr))
        }
        JsonValue::Object(obj) => {
            let mut table = toml::map::Map::new();
            for (key, value) in obj {
                table.insert(key, json_to_toml(value)?);
            }
            Ok(toml::Value::Table(table))
        }
    }
}
