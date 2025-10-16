use json5;
use mlua::{Error, Lua, Result, Value};
use serde_json;

use crate::utils::json_utils;

// TODO
// Probably fix this
pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?; // JSON-Tabelle

    // // json.decode to Lua Table
    // let json_decode = lua.create_function(|lua, json_str: String| {
    //     let json_value: JsonValue =
    //         serde_json::from_str(&json_str).map_err(Error::external)?;
    //     let lua_value = json_value.to_lua(lua)?;
    //     Ok(lua_value)
    // })?;

    // Decode JSON to a Lua Table
    let json_decode2 = lua.create_function(|lua, json_str: String| {
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(Error::external)?;
        Ok(json_utils::json_to_lua(lua, &json_value))
    })?;

    // JSON5 decode (unterstützt Kommentare)
    let json_decode_with_comments = lua.create_function(|lua, json_str: String| {
        let json_value: serde_json::Value = json5::from_str(&json_str).map_err(Error::external)?;
        Ok(json_utils::json_to_lua(lua, &json_value)?)
    })?;

    // json.encode form lua table
    let json_encode = lua.create_function(|_, value: Value| {
        let serde_value = json_utils::lua_to_json(&value)?;
        let json_str = serde_json::to_string(&serde_value).map_err(Error::external)?;
        Ok(json_str)
    })?;

    // table.set("decode", json_decode)?;
    table.set("decode2", json_decode2)?;
    table.set("decode_with_comments", json_decode_with_comments)?;
    table.set("encode", json_encode)?;

    Ok(table)
}
