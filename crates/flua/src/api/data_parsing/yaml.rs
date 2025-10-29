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

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{Lua, Value};

    #[test]
    fn test_yaml_decode() {
        let lua = Lua::new();
        let yaml = r#"
        key1: value1
        key2:
          - listitem1
          - listitem2
        key3:
          nested_key: 123
        "#;

        let table = register(&lua).expect("Failed to register YAML functions");
        let decode: mlua::Function = table.get("decode").expect("No decode function");

        let result: mlua::Table = decode.call(yaml.to_string()).expect("Decode failed");

        assert_eq!(result.get::<String>("key1").unwrap(), "value1");

        let key2: mlua::Table = result.get("key2").unwrap();
        assert_eq!(key2.get::<String>(1).unwrap(), "listitem1");
        assert_eq!(key2.get::<String>(2).unwrap(), "listitem2");

        let key3: mlua::Table = result.get("key3").unwrap();
        assert_eq!(key3.get::<i64>("nested_key").unwrap(), 123);
    }

    #[test]
    fn test_yaml_encode() {
        let lua = Lua::new();
        let table = register(&lua).expect("Failed to register YAML functions");
        let encode: mlua::Function = table.get("encode").expect("No encode function");

        // Erstelle eine Lua-Tabelle, die YAML repräsentiert
        let lua_table = lua.create_table().unwrap();
        lua_table.set("foo", "bar").unwrap();

        let list = lua.create_table().unwrap();
        list.set(1, "eins").unwrap();
        list.set(2, "zwei").unwrap();
        lua_table.set("zahlen", list).unwrap();

        let nested = lua.create_table().unwrap();
        nested.set("key", 42).unwrap();
        lua_table.set("verschachtelt", nested).unwrap();

        // Rufe encode auf und erhalte YAML-String
        let yaml_str: String = encode.call(Value::Table(lua_table)).expect("Encode failed");

        // Einfacher Check, ob der String die erwarteten Teile enthält
        assert!(yaml_str.contains("foo: bar"));
        assert!(yaml_str.contains("- eins"));
        assert!(yaml_str.contains("key: 42"));
    }

    // TODO
    //
    // Test in beide Richtungen
}
