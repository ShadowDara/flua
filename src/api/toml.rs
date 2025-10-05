use mlua::LuaSerdeExt;
use mlua::serde::Serializer;
use mlua::{Error, Lua, Result, String as LuaString, Value};
use serde_json::Value as JsonValue;
use toml;
use std::collections::BTreeMap;

use crate::utils::toml_utils::json_to_toml;

// TODO
// Probably fix this
/// Registriert das Lua-Modul `toml`
pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // toml.decode(toml_string)
    let decode = lua.create_function(|lua, toml_str: LuaString| {
        // 1. Lua-String → &str
        let toml_str = toml_str.to_str()?; // BorrowedStr

        // 2. parse TOML → toml::Value
        let toml_value: toml::Value = toml::from_str(&toml_str).map_err(Error::external)?;

        // 3. konvertiere zu serde_json::Value
        let json_value: JsonValue = toml_value.try_into().map_err(Error::external)?;

        // 4. konvertiere serde_json::Value → Lua-Value (Tabelle)
        let lua_value = lua.to_value(&json_value)?; // <– das ist korrekt!

        Ok(lua_value)
    })?;
    let encode = lua.create_function(|lua, value: Value| {
        // Lua → serde_json::Value
        let json_value: JsonValue = lua.from_value(value).map_err(Error::external)?;

        // serde_json::Value → toml::Value
        let toml_value = json_to_toml(json_value).map_err(Error::external)?;

        // toml::Value → TOML-String
        let toml_string = toml::to_string(&toml_value).map_err(Error::external)?;

        Ok(toml_string)
    })?;

    table.set("decode", decode)?;
    table.set("encode", encode)?;

    Ok(table)
}
