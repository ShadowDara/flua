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

    // json encode from lua table
    let json_encode = lua.create_function(|_, (value, pretty): (Value, Option<bool>)| {
        let serde_value = json_utils::lua_to_json(&value)?;

        // Decide for pretty JSON
        let json_str = if pretty.unwrap_or(false) {
            serde_json::to_string_pretty(&serde_value)
        } else {
            serde_json::to_string(&serde_value)
        }
        .map_err(Error::external)?;

        Ok(json_str)
    })?;

    // TODO
    // add function to docs
    // Convert compact JSON -> pretty JSON
    let compact_to_pretty = lua.create_function(|_, json_str: String| {
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(Error::external)?;
        let pretty = serde_json::to_string_pretty(&json_value).map_err(Error::external)?;
        Ok(pretty)
    })?;

    // TODO
    // add function to docs
    // Convert pretty JSON -> compact JSON
    let pretty_to_compact = lua.create_function(|_, json_str: String| {
        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(Error::external)?;
        let compact = serde_json::to_string(&json_value).map_err(Error::external)?;
        Ok(compact)
    })?;

    // table.set("decode", json_decode)?;
    table.set("decode2", json_decode2)?;
    table.set("decode_with_comments", json_decode_with_comments)?;
    table.set("encode", json_encode)?;
    table.set("compact_to_pretty", compact_to_pretty)?;
    table.set("pretty_to_compact", pretty_to_compact)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    fn init() -> (Lua, mlua::Table) {
        let lua = Lua::new();
        let table = register(&lua).expect("Failed to register JSON functions");
        (lua, table)
    }

    #[test]
    fn test_encode_decode() {
        let (_lua, table) = init();

        let decode: mlua::Function = table.get("decode2").unwrap();
        let encode: mlua::Function = table.get("encode").unwrap();

        let input = r#"{"name":"Alice","age":30}"#;
        let lua_table: mlua::Value = decode.call(input).unwrap();
        let encoded: String = encode.call((lua_table, Some(false))).unwrap();

        // Vergleich als JSON-Werte, nicht Strings
        let expected: serde_json::Value = serde_json::from_str(input).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_encode_pretty() {
        let (lua, table) = init();
        let encode: mlua::Function = table.get("encode").unwrap();

        let tbl = lua
            .load(r#"{ name = "Bob", age = 25 }"#)
            .eval::<mlua::Value>()
            .unwrap();

        let pretty: String = encode.call((tbl, Some(true))).unwrap();
        assert!(pretty.contains("\n")); // should be pretty
        assert!(pretty.contains("Bob"));
    }

    #[test]
    fn test_json5_decode() {
        let (_lua, table) = init();
        let decode: mlua::Function = table.get("decode_with_comments").unwrap();
        let input = r#"
            // this is a comment
            {
                name: "Charlie", // name field
                age: 22
            }
        "#;
        let val: mlua::Value = decode.call(input).unwrap();
        match val {
            mlua::Value::Table(t) => {
                let name: String = t.get("name").unwrap();
                let age: i64 = t.get("age").unwrap();
                assert_eq!(name, "Charlie");
                assert_eq!(age, 22);
            }
            _ => panic!("Expected Lua table"),
        }
    }

    #[test]
    fn test_pretty_to_compact_conversion() {
        let (_lua, table) = init();
        let compact_fn: mlua::Function = table.get("pretty_to_compact").unwrap();

        let input = r#"{
        "hello": "world",
        "foo": "bar"
    }"#;

        let output: String = compact_fn.call(input).unwrap();

        // Vergleich strukturell statt String
        let expected: serde_json::Value =
            serde_json::from_str(r#"{"hello":"world","foo":"bar"}"#).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_compact_to_pretty_conversion() {
        let (_lua, table) = init();
        let pretty_fn: mlua::Function = table.get("compact_to_pretty").unwrap();

        let input = r#"{"hello":"world","foo":"bar"}"#;
        let output: String = pretty_fn.call(input).unwrap();

        assert!(output.contains("\n"));
        assert!(output.contains("hello"));
    }
}
