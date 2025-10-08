use mlua::{Error, Lua, Result, Value};
use serde_yaml;

use crate::utils::json_utils; // Du nutzt json_utils für (de)serialization Lua <-> Serde

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?; // YAML-Tabelle

    // Decode YAML string to Lua Table
    let yaml_decode = lua.create_function(|lua, yaml_str: String| {
        // Deserialize YAML directly into serde_json::Value
        let json_value: serde_json::Value =
            serde_yaml::from_str(&yaml_str).map_err(Error::external)?;

        json_utils::json_to_lua(lua, &json_value)
    })?;

    // Encode Lua Table to YAML string
    let yaml_encode = lua.create_function(|_, value: Value| {
        let serde_value = json_utils::lua_to_json(&value)?; // unterstützt auch yaml::Value kompatibel
        let yaml_str = serde_yaml::to_string(&serde_value).map_err(Error::external)?;
        Ok(yaml_str)
    })?;

    table.set("decode", yaml_decode)?;
    table.set("encode", yaml_encode)?;

    Ok(table)
}
