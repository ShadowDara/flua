use mlua::{Lua, Result, Value};
use serde_json::Value as JsonValue;

// TODO
// Probably fix this
// JSON Parsing
pub fn json_to_lua(lua: &Lua, json: &serde_json::Value) -> Result<mlua::Value> {
    match json {
        serde_json::Value::Null => Ok(mlua::Value::Nil),
        serde_json::Value::Bool(b) => Ok(mlua::Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(mlua::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(mlua::Value::Number(f))
            } else {
                Err(mlua::Error::external("Invalid number"))
            }
        }
        serde_json::Value::String(s) => Ok(mlua::Value::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, v) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, v)?)?;
            }
            Ok(mlua::Value::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (k, v) in obj {
                table.set(k.as_str(), json_to_lua(lua, v)?)?;
            }
            Ok(mlua::Value::Table(table))
        }
    }
}

// TODO
// Probably fix this
pub fn lua_to_json(value: &Value) -> Result<JsonValue> {
    match value {
        Value::Nil => Ok(JsonValue::Null),
        Value::Boolean(b) => Ok(JsonValue::Bool(*b)),
        Value::Integer(i) => Ok(JsonValue::Number((*i).into())),
        Value::Number(n) => {
            serde_json::Number::from_f64(*n)
                .map(JsonValue::Number)
                .ok_or_else(|| mlua::Error::external("Invalid f64 number for JSON"))
        }
        Value::String(s) => Ok(JsonValue::String(s.to_str()?.to_string())),
        Value::Table(table) => {
            // Lua tables können Array oder Map sein - wir unterscheiden das
            // Einfachster Ansatz: Wenn alle keys sind 1..n, dann Array, sonst Object
            let mut is_array = true;
            let mut max_index = 0usize;
            let mut keys = vec![];

            for pair in table.clone().pairs::<Value, Value>() {
                let (k, _) = pair?;
                match k {
                    Value::Integer(i) if i >= 1 => {
                        if i as usize > max_index {
                            max_index = i as usize;
                        }
                        keys.push(i as usize);
                    }
                    _ => {
                        is_array = false;
                        break;
                    }
                }
            }

            if is_array {
                // Array
                let mut vec = Vec::with_capacity(max_index);
                for i in 1..=max_index {
                    let v = table.get::<Value>(i)?;
                    vec.push(lua_to_json(&v)?);
                }
                Ok(JsonValue::Array(vec))
            } else {
                // Map / Objekt
                let mut map = serde_json::Map::new();
                for pair in table.clone().pairs::<Value, Value>() {
                    let (k, v) = pair?;
                    // key to string (Lua erlaubt nur string keys im JSON-Objekt)
                    let key_str = match k {
                        Value::String(s) => s.to_str()?.to_string(),
                        Value::Integer(i) => i.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        _ => return Err(mlua::Error::external("Unsupported table key type for JSON")),
                    };
                    map.insert(key_str, lua_to_json(&v)?);
                }
                Ok(JsonValue::Object(map))
            }
        }
        _ => Err(mlua::Error::external("Unsupported Lua value type for JSON serialization")),
    }
}
