use ini;

use crate::utils::json_utils;

use mlua::{Error, Lua, Result, Table, Value};
use serde_json;

use std::collections::HashMap;

type IniMap = HashMap<String, HashMap<String, Option<String>>>;

/// Hilfsfunktion: Konvertiert eine mlua Lua Table zurück in HashMap
fn lua_table_to_ini(table: &Table) -> Result<IniMap> {
    let mut map = HashMap::new();

    for pair in table.clone().pairs::<String, Table>() {
        let (section, sub_table) = pair?;
        let mut props = HashMap::new();

        for pair2 in sub_table.pairs::<String, Value>() {
            let (key, value) = pair2?;
            let opt_val = match value {
                Value::Nil => None,
                Value::String(s) => Some(s.to_str()?.to_string()),
                // Optional: Andere Typen konvertieren wir auch in Strings
                Value::Integer(i) => Some(i.to_string()),
                Value::Number(n) => Some(n.to_string()),
                Value::Boolean(b) => Some(b.to_string()),
                _ => None,
            };
            props.insert(key.to_lowercase(), opt_val); // optional: keys lowercase machen
        }
        map.insert(section.to_lowercase(), props);
    }

    Ok(map)
}

fn map_to_ini_string(map: &HashMap<String, HashMap<String, Option<String>>>) -> String {
    let mut result = String::new();

    for (section, values) in map {
        // Skip "default" if you want no section header
        if section.to_lowercase() != "default" {
            result.push_str(&format!("[{}]\n", section));
        }

        for (key, value_opt) in values {
            match value_opt {
                Some(value) => result.push_str(&format!("{} = {}\n", key, value)),
                None => result.push_str(&format!("{}\n", key)), // key with no value
            }
        }

        result.push('\n'); // Add blank line between sections
    }

    result
}

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Start with the content of a Ini File, returns a Lua Table
    let parse_ini = lua.create_function(|lua, value: String| {
        let ini_value: &str = &value;
        let ini_map = ini::inistr!(ini_value);
        let json_str = serde_json::to_string_pretty(&ini_map)
            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(Error::external)?;
        json_utils::json_to_lua(lua, &json_value)
    })?;

    // Covernts a Lua Table to ini
    let create_ini = lua.create_function(|_, value: Table| {
        let map = lua_table_to_ini(&value)?;
        Ok(map_to_ini_string(&map))
    })?;

    table.set("parse", parse_ini)?;
    table.set("convert", create_ini)?;

    Ok(table)
}
