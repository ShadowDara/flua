use mlua::LuaSerdeExt;
use mlua::{Error, Lua, Result, String as LuaString, Value};
use serde_json::Value as JsonValue;
use toml;

// Import the Helper Method
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

// Tests for TOML
#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{Function as LuaFunction, Lua, LuaSerdeExt, Table, Value as LuaValue};
    use toml;

    fn setup_lua() -> Lua {
        Lua::new()
    }

    #[test]
    fn test_decode_simple_table() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let decode: LuaFunction = module.get("decode")?;

        let toml_str = r#"
            [user]
            name = "Alice"
            age = 30
        "#;

        let value: LuaValue = decode.call(toml_str)?;
        let tbl: Table = match value {
            LuaValue::Table(t) => t,
            _ => panic!("decode should return a Lua table"),
        };

        let user: Table = tbl.get("user")?;
        assert_eq!(user.get::<String>("name")?.as_str(), "Alice");
        assert_eq!(user.get::<i64>("age")?, 30);

        Ok(())
    }

    #[test]
    fn test_encode_simple_table() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let encode: LuaFunction = module.get("encode")?;

        let tbl = lua.create_table()?;
        let user = lua.create_table()?;
        user.set("name", "Bob")?;
        user.set("age", 42)?;
        tbl.set("user", user)?;

        let toml_str: String = encode.call(tbl)?;
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed["user"]["name"].as_str().unwrap(), "Bob");
        assert_eq!(parsed["user"]["age"].as_integer().unwrap(), 42);

        Ok(())
    }

    #[test]
    fn test_decode_with_array() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let decode: LuaFunction = module.get("decode")?;

        let toml_str = r#"
            numbers = [1, 2, 3]
        "#;

        let value: LuaValue = decode.call(toml_str)?;
        let tbl: Table = value.as_table().unwrap().clone();
        let numbers: Vec<i64> = lua.from_value(tbl.get("numbers")?)?;

        assert_eq!(numbers, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_encode_array() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let encode: LuaFunction = module.get("encode")?;

        let tbl = lua.create_table()?;
        tbl.set("numbers", lua.create_sequence_from(vec![1, 2, 3])?)?;

        let toml_str: String = encode.call(tbl)?;
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();

        let numbers = parsed["numbers"].as_array().unwrap();
        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers[0].as_integer().unwrap(), 1);

        Ok(())
    }

    #[test]
    fn test_decode_invalid_toml_should_fail() {
        let lua = setup_lua();
        let module = register(&lua).unwrap();
        let decode: LuaFunction = module.get("decode").unwrap();

        let invalid_toml = "this = is not valid";
        let result: Result<LuaValue> = decode.call(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_nested_tables() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let encode: LuaFunction = module.get("encode")?;

        let tbl = lua.create_table()?;
        let inner = lua.create_table()?;
        let nested = lua.create_table()?;

        nested.set("flag", true)?;
        inner.set("nested", nested)?;
        tbl.set("outer", inner)?;

        let toml_str: String = encode.call(tbl)?;
        let parsed: toml::Value = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed["outer"]["nested"]["flag"].as_bool().unwrap(), true);
        Ok(())
    }

    #[test]
    fn test_round_trip_encode_decode() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let encode: LuaFunction = module.get("encode")?;
        let decode: LuaFunction = module.get("decode")?;

        let tbl = lua.create_table()?;
        tbl.set("name", "Charlie")?;
        tbl.set("age", 25)?;
        tbl.set(
            "tags",
            lua.create_sequence_from(vec!["rust", "lua", "toml"])?,
        )?;

        let toml_str: String = encode.call(tbl.clone())?;
        let decoded: LuaValue = decode.call(toml_str)?;
        let decoded_tbl: Table = decoded.as_table().unwrap().clone();

        assert_eq!(decoded_tbl.get::<String>("name")?.as_str(), "Charlie");
        assert_eq!(decoded_tbl.get::<i64>("age")?, 25);

        let tags: Vec<String> = lua.from_value(decoded_tbl.get("tags")?)?;
        assert_eq!(tags, vec!["rust", "lua", "toml"]);

        Ok(())
    }

    #[test]
    fn test_encode_empty_table() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let encode: LuaFunction = module.get("encode")?;

        let tbl = lua.create_table()?;
        let toml_str: String = encode.call(tbl)?;
        assert!(toml_str.trim().is_empty());

        Ok(())
    }

    #[test]
    fn test_decode_empty_toml() -> Result<()> {
        let lua = setup_lua();
        let module = register(&lua)?;
        let decode: LuaFunction = module.get("decode")?;

        let value: LuaValue = decode.call("")?;
        let tbl: Table = value.as_table().unwrap().clone();
        assert_eq!(tbl.len()?, 0);

        Ok(())
    }
}
